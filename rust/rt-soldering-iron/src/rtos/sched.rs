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

/// State saved by the NVIC when an interrupt occurs
///
/// This is used to set up the initial stack contents for when a task is
/// created.
#[repr(C)]
struct SavedState {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32
}

impl SavedState {
    /// Creates a new saved state from a task
    fn new(t: TaskFn) -> SavedState {
        SavedState {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
            lr: 0,
            pc: t as u32,
            psr: 0x01000000
        }
    }
}

struct TaskCollection {
    current_task: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT]
}

/// Gets the current stack pointer
#[cfg(target_arch = "arm")]
#[inline(always)]
fn get_stack_pointer() -> u32 {
    let mut sp : u32;
    unsafe { asm!("mov $0, sp" : "=r"(sp) ::: "volatile") };
    sp
}

/// Gets the current stack pointer (unimplemented for non-ARM architectures)
#[cfg(not(target_arch = "arm"))]
fn get_stack_pointer() -> u32 {
    unimplemented!();
}

/// Sets the current stack pointer
#[cfg(target_arch = "arm")]
#[inline(always)]
fn set_stack_pointer(sp: u32) {
    unsafe { asm!("mov sp, $0" :: "r"(sp) :: "volatile") };
}

/// Sets the current stack pointer (unimplemented for non-ARM architectures)
#[cfg(not(target_arch = "arm"))]
fn set_stack_pointer(sp: u32) {
    unimplemented!();
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

/// Requests a context switch.
#[inline(always)]
pub fn switch_context() {
}

