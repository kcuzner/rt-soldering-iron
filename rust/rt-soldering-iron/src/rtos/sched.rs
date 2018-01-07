//! RTOS Scheduler
//!
//! Contains the global task list and methods for interacting with it.

use core;
use core::u32;
use bare_metal::CriticalSection;
use cortex_m;

use rtos::Result;
use rtos::tasks::{TaskFn, TaskDescriptor, MIN_STACK_SIZE};

/// Maximum task count, including the idle task
pub const MAX_TASKS_COUNT: usize = 4;

/// Task collection
static mut TASKS: TaskCollection = TaskCollection {
    current_task: 0,
    tasks: [TaskDescriptor::new(); MAX_TASKS_COUNT]
};

struct TaskCollection {
    current_task: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT]
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
        if TASKS.current_task >= MAX_TASKS_COUNT {
            return Err(());
        }
        TASKS.tasks[TASKS.current_task].setup(t, stack, cs)?;
        Ok(())
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

/// Rust side of the scheduler. Called from the PENDSV ISR.
///
/// This takes in the stack pointer to store in the task and returns the new
/// stack pointer to use after scheduling is complete. The PENDSV ISR will then
/// actually switch over the stack.
///
/// This should be run in a critical section, but I don't know how to create
/// a CriticalSection from a naked function, so the function is marked unsafe.
unsafe extern "C" fn scheduler(sp: u32) -> u32 {
    sp
}

/// PendSV interrupt handler
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
             mov lr, r0
             bx lr" : : "i"(scheduler as unsafe extern "C" fn(u32) -> u32) : : "volatile");
    }
}

