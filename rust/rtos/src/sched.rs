//! RTOS Scheduler
//!
//! Contains the global task list and methods for interacting with it.

use core::u32;
use bare_metal::CriticalSection;
use cortex_m;

use Result;
use tasks::{TaskFn, TaskDescriptor, MIN_STACK_SIZE};
use sync::Block;

/// Maximum task count, excluding the idle task
pub const MAX_TASKS_COUNT: usize = 4;

/// Task collection
static mut TASKS: TaskCollection = TaskCollection::new();

struct TaskCollection {
    current_task: Option<usize>,
    task_count: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT],
    idle_task: TaskDescriptor,
}

impl TaskCollection {
    /// Creates a new task collection
    const fn new() -> Self {
        TaskCollection {
            current_task: None,
            task_count: 0,
            tasks: [TaskDescriptor::new(); MAX_TASKS_COUNT],
            idle_task: TaskDescriptor::new()
        }
    }

    /// Runs initial setup for the task collection.
    fn setup(&mut self, cs: &CriticalSection) -> Result<()> {
        self.idle_task.setup(idle_task, unsafe { &IDLE_STACK }, cs)
    }

    /// Sets the stack top for the current task.
    ///
    /// The caller must ensure that a task was already runing at the time that
    /// this is called.
    unsafe fn set_stack_top(&mut self, sp: u32, cs: &CriticalSection)  -> Result<()>{
        match self.current_task {
            Some(n) => self.tasks[n].set_stack_top(sp, cs),
            None => self.idle_task.set_stack_top(sp, cs)
        }
    }

    /// Sets the current task to the next one and returns the resulting stack
    /// pointer
    fn next(&mut self, cs: &CriticalSection) -> Result<u32> {
        // Find the first task in the array that is ready
        for i in 0..self.task_count {
            if self.tasks[i].is_ready() {
                self.current_task = Some(i);
                return self.tasks[i].stack_top(cs);
            }
        }
        // If no tasks are ready, resume from the idle task
        self.current_task = None; //prevent block from functioning in this situation
        self.idle_task.stack_top(cs)
    }

    /// Adds a new task to this collection
    fn add_task(&mut self, t: TaskFn, stack: &'static[u8], cs: &CriticalSection) -> Result<()> {
        if self.task_count >= MAX_TASKS_COUNT {
            return Err(());
        }
        self.tasks[self.task_count].setup(t, stack, cs)?;
        self.task_count += 1;
        Ok(())
    }

    /// Changes the current task state to blocked. Fails if the RTOS has not
    /// been started or a task not properly runtime-initialized.
    fn block(&mut self, block: &Block, cs: &CriticalSection) -> Result<()> {
        match self.current_task {
            Some(n) => self.tasks[n].block(block, cs),
            _ => Err(())
        }
    }

    /// Runs through the tasks and unblocks the highest priority blocked task
    /// that accepts the passed Block. A task may or may not unblock, but this
    /// function succeeds in all cases.
    fn unblock(&mut self, block: &Block, cs: &CriticalSection) -> bool {
        for i in 0..self.task_count {
            if self.tasks[i].try_unblock(block, cs) {
                return true
            }
        }
        false
    }

    /// Gets an id representing the current task. Not valid to call during the
    /// idle task.
    fn task_id(&self, _: &CriticalSection) -> Result<u32> {
        match self.current_task {
            Some(i) => Ok(i as u32),
            _ => Err(())
        }
    }

    /// Unblocks the task based on the passed ID and block
    fn unblock_by_id(&mut self, id: u32, block: &Block, cs: &CriticalSection) {
        self.tasks[id as usize].try_unblock(block, cs);
    }
}

static mut IDLE_STACK: [u8; MIN_STACK_SIZE as usize] = [0; MIN_STACK_SIZE as usize];

/// Idle task
#[inline(never)]
fn idle_task() {
    loop { }
}

/// Adds a task to the scheduler. Tasks added first have the highest
/// priority.
pub fn add_task(t: TaskFn, stack: &'static[u8]) -> Result<()> {
    cortex_m::interrupt::free(|cs| unsafe {
        TASKS.add_task(t, stack, &cs)
    })
}

/// Runs the scheduler. This function never returns unless there is an
/// error.
#[inline(never)]
pub fn run() -> Result<()> {
    cortex_m::interrupt::free(|cs| unsafe {
        TASKS.setup(cs)
    })?;

    //Interrupts must be enabled at this point
    unsafe {
        cortex_m::interrupt::enable();
        switch_context();
    };

    Err(())
}

/// Requests a context switch by setting PendSV.
///
/// Interrupts must be enabled when this is called. If they are disabled, the
/// context switch will occur immediately after they are re-enabled. This is
/// unsafe, because misbehavior may result if this is called with interrupts
/// disabled.
#[inline(always)]
pub unsafe fn switch_context() {
    //This is an atomic write and should be safe at all times.
    (*cortex_m::peripheral::SCB::ptr()).icsr.write(0x10000000);
}

/// Blocks the current task. Does not request a context switch. The context
/// switch must be requested after the critical section. The task will not
/// actually block until switch_context() is called.
pub fn block(block: &Block, cs: &CriticalSection) -> Result<()> {
    unsafe { TASKS.block(block, cs) }?;
    Ok(())
}

/// Unblocks zero or one tasks, in priority order. Does not request a context
/// switch. The context switch must be requested after the critical section.
/// The task will not actually yield until switch_context() is called.
pub fn unblock(block: &Block, cs: &CriticalSection) -> bool {
    unsafe { TASKS.unblock(block, cs) }
}

/// Gets the id of the current task.
///
/// Requires a critical section to ensure that the task id won't change.
pub fn task_id(cs: &CriticalSection) -> Result<u32> {
    unsafe { TASKS.task_id(cs) }
}

/// Scheduler implementation, presented as a safe interface
///
/// This takes in the stack pointer to store for the currently running task
/// and returns the stack pointer for the next task. The caller should change
/// the sp register to point to this new stack pointer.
///
/// One assumption here is that the stack pointer passed in was gathered after
/// the context of the task was stored on the stack. It would be nice if we
/// could somehow enforce this precondition, since if that condition isn't met
/// this will crash and burn. So many things could go wrong: We could pop the
/// stack too many times, we could pop something weird into the pc, get a very
/// messed up register state, etc.
///
/// In reality, all we are doing is changing some internal state to track that
/// we are switching tasks. The callers of this function are responsible to
/// make sure that the stack pointers and such get set appropriately.
///
/// A critical section is required because it would be quite bad if this were
/// interrupted while the internal state was mutating. We are mutating statics
/// here, after all.
fn scheduler_impl(sp: u32, stored: bool, cs: &CriticalSection) -> Result<u32> {
    if stored {
        unsafe { TASKS.set_stack_top(sp, cs) }?;
    }
    unsafe { TASKS.next(cs) }
}

/// Rust side of the scheduler. Called from the PENDSV ISR. Unsafe interface.
///
/// This takes in the stack pointer to store in the task and returns the new
/// stack pointer to use after scheduling is complete. The PENDSV ISR will then
/// actually switch over the stack.
///
/// This should be run in a critical section, but I don't know how to create
/// a CriticalSection from a naked function, so the function is marked unsafe.
unsafe extern "C" fn scheduler(sp: u32, stored: u32) -> u32 {
    // The caller should be calling this from within a critical section, so we
    // now start one up to interface with the safe code.
    let cs = CriticalSection::new();
    // Attempt to switch to the next task. If unsuccessful, just use the
    // current task.
    match scheduler_impl(sp, stored > 0, &cs) {
        Ok(nsp) => nsp,
        _ => sp
    }
}

/// PendSV interrupt handler
///
/// This interrupt handler is implemented in assembly. It targets the thumbv6m
/// instruction set. It assumes that there are 12 registers in addition to the
/// core registers: r0-r11. It further assumes that if it interrupts something
/// running on the msp (rather than the psp), it is running for the first time.
/// This means that the PENDSV interrupt must be the lowest priority interrupt.
///
/// On a Cortex-M0+, it seems there is no preemption priority bits in the AICR.
/// This should mean that interrupts should be unable to preempt other
/// interrupts.
///
/// If this does preempt an ISR, it will believe that the OS has just started
/// and will not store the stack. This could seriously misbehave.
#[naked]
#[no_mangle]
#[cfg(target_arch = "arm")]
pub extern "C" fn PENDSV() {
    unsafe {
        // Things to remember:
        // At the point that this function is called, the NVIC has already
        // stored the following on the stack:
        // - R0-R3
        // - R12
        // - LR
        // - PC
        // - PSR
        asm!("mov r0, lr
             ldr r1, =0x4
             tst r0, r1
             beq 1f //if lr & 0x4 == 0, then we interrupted the msp.
             mrs r0, psp
             subs r0, r0, #32 //4*4 for r4-r7, 4*4 for r8-r11
             mov r2, r0
             stmia r2!, {r4-r7} //store r4-r7
             mov r4, r8
             mov r5, r9
             mov r6, r10
             mov r7, r11
             stmia r2!, {r4-r7} //store r8-r11
             //r0 contains the top of the stack
             //r1 contains the number 4, which is greater than zero and will mean that we stored
             //the context
             b 2f

             //if we get here, no context was stored since we interrupted the msp
             1:
             eors r1, r1 //zero out r1
             2:

             //enter critical section
             cpsid i
             bl $0
             cpsie i
             //exit critical section

             //the top of the stack contains r4-r7 followed by r8-r11 as the address increases
             adds r0, r0, #16 //start at r8-r11
             ldmia r0!, {r4-r7}
             mov r8, r4
             mov r9, r5
             mov r10, r6
             mov r11, r7
             //at this point, r0 contains the top of the stack for the NVIC
             msr psp, r0
             subs r0, r0, #32 //move to r4-r7
             ldmia r0!, {r4-r7}
             ldr r0, =0xfffffffd //tell the NVIC we are done, resume into psp
             mov lr, r0" : : "i"(scheduler as unsafe extern "C" fn(u32, u32) -> u32) : : "volatile");
        // Naked function does provide a "bx lr" at the end
    }
}

