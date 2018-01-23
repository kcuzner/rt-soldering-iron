//! RTOS Scheduler
//!
//! Contains the global task list and methods for interacting with it.

use core::u32;
use bare_metal::CriticalSection;
use cortex_m;

use rtos::Result;
use rtos::tasks::{TaskFn, TaskDescriptor, MIN_STACK_SIZE};
use rtos::sync::Block;

/// Maximum task count, excluding the idle task
pub const MAX_TASKS_COUNT: usize = 4;

/// Task collection
static mut TASKS: TaskCollection = TaskCollection::new();

//TODO: Handle the case where tasks are added after the scheduler is running. Right now, the idle
//task won't be the lowest priority

struct TaskCollection {
    current_task: Option<usize>,
    task_count: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT],
}

impl TaskCollection {
    /// Creates a new task collection
    const fn new() -> Self {
        TaskCollection {
            current_task: None,
            task_count: 0,
            tasks: [TaskDescriptor::new(); MAX_TASKS_COUNT]
        }
    }

    /// Sets the stack top for the current task
    fn set_stack_top(&mut self, sp: u32, cs: &CriticalSection)  -> Result<()>{
        match self.current_task {
            Some(n) => self.tasks[n].set_stack_top(sp, cs),
            _ => Ok(())
        }
    }

    /// Sets the current task to the next one and returns the resulting stack
    /// pointer
    fn next(&mut self, cs: &CriticalSection) -> Result<u32> {
        for i in 0..self.task_count {
            if self.tasks[i].is_ready() {
                self.current_task = Some(i);
                return self.tasks[i].stack_top(cs);
            }
        }
        match self.current_task {
            Some(i) => self.tasks[i].stack_top(cs),
            _ => Err(())
        }
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
    fn block(&mut self, block: &'static Block, cs: &CriticalSection) -> Result<()> {
        match self.current_task {
            Some(n) => self.tasks[n].block(block, cs),
            _ => Err(())
        }
    }

    /// Runs through the tasks and unblocks the highest priority blocked task
    /// that accepts the passed Block. A task may or may not unblock, but this
    /// function succeeds in all cases.
    fn unblock(&mut self, block: &'static Block, cs: &CriticalSection) -> bool {
        for i in 0..self.task_count {
            if self.tasks[i].try_unblock(block, cs) {
                return true
            }
        }
        false
    }

    /// Gets an id representing the current task
    fn task_id(&self) -> Option<u32> {
        match self.current_task {
            Some(i) => Some(i as u32),
            _ => None
        }
    }

    /// Unblocks the task based on the passed ID and block
    fn unblock_by_id(&mut self, id: u32, block: &'static Block, cs: &CriticalSection) {
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
    add_task(idle_task, unsafe { &IDLE_STACK[..] })?;

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
    let scb = cortex_m::peripheral::SCB.get();
    (*scb).icsr.write(0x10000000);
}

/// Blocks the current task. Does not request a context switch. The context
/// switch must be requested after the critical section. The task will not
/// actually block until switch_context() is called.
pub fn block(block: &'static Block, cs: &CriticalSection) -> Result<()> {
    unsafe { TASKS.block(block, cs) }?;
    Ok(())
}

/// Unblocks zero or one tasks, in priority order. Does not request a context
/// switch. The context switch must be requested after the critical section.
/// The task will not actually yield until switch_context() is called.
pub fn unblock(block: &'static Block, cs: &CriticalSection) -> bool {
    unsafe { TASKS.unblock(block, cs) }
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
fn scheduler_impl(sp: u32, cs: &CriticalSection) -> Result<u32> {
    unsafe { TASKS.set_stack_top(sp, cs) }?;
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
unsafe extern "C" fn scheduler(sp: u32) -> u32 {
    // The caller should be calling this from within a critical section, so we
    // now start one up to interface with the safe code.
    let cs = CriticalSection::new();
    // Attempt to switch to the next task. If unsuccessful, just use the
    // current task.
    match scheduler_impl(sp, &cs) {
        Ok(nsp) => nsp,
        _ => sp
    }
}

/// PendSV interrupt handler
///
/// This interrupt handler is implemented in assembly. It targets the thumbv6m
/// instruction set. It assumes that there are 12 registers in addition to the
/// core registers: r0-r11.
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
        //
        // Since this RTOS does not yet use the two different stack pointers,
        // we have the "push" and "pop" instructions available.
        asm!("push {r4-r7} //store r4-r7
             mov r4, r8
             mov r5, r9
             mov r6, r10
             mov r7, r11
             push {r4-r7} //store r8-r11
             mov r0, sp //this is the point where we save the stack

             //enter critical section
             cpsid i
             bl $0
             cpsie i
             //exit critical section

             mov sp, r0  //switch the stack over
             pop {r4-r7} //pop r8-r11
             mov r8, r4
             mov r9, r5
             mov r10, r6
             mov r11, r7
             pop {r4-r7} //pop r4-r7
             ldr r0, =0xfffffff9 //tell the NVIC we are done
             mov lr, r0" : : "i"(scheduler as unsafe extern "C" fn(u32) -> u32) : : "volatile");
        // Naked function does provide a "bx lr" at the end
    }
}

