//! RTOS Scheduler
//!
//! Contains the global task list and methods for interacting with it.

use core;
use core::u32;
use bare_metal::CriticalSection;
use cortex_m;

use rtos::Result;
use rtos::tasks::{TaskFn, TaskDescriptor, MIN_STACK_SIZE};

/// Maximum task count, excluding the idle task
pub const MAX_TASKS_COUNT: usize = 4;

/// Task collection
static mut TASKS: TaskCollection = TaskCollection::new();

struct TaskCollection {
    current_task: Option<usize>,
    task_count: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT],
}

impl TaskCollection {
    /// Creates a new task collection
    const fn new() -> TaskCollection {
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
}

static IDLE_STACK: [u8; MIN_STACK_SIZE as usize] = [0; MIN_STACK_SIZE as usize];

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
    add_task(idle_task, &IDLE_STACK[..])?;

    switch_context();
    Err(())
}

/// Requests a context switch by setting PendSV.
#[inline(always)]
pub fn switch_context() {
    let scb = cortex_m::peripheral::SCB.get();
    unsafe { (*scb).icsr.write(0x10000000); }
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
    //TODO: The startup case!! There is no stack top to set...
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

