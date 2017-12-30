//! Minimal RTOS, based loosly on kos-avr and zinc
//!
//! This operates by having the pendsv interrupt perform context switches.

use core::u32;
use core::cell::Cell;
use bare_metal::CriticalSection;
use cortex_m;

/// State of a task
#[derive(Copy, Clone)]
enum TaskState {
    /// The task is ready
    Ready,
    /// The task is blocked on something
    Blocked(u32),
}

pub type TaskFn = fn();

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

mod current_stack_offset {
    /// Stack offset, will be set to the current stack location when the RTOS
    /// is set up.
    static mut CURRENT_STACK_OFFSET: u32 = 0;

    pub fn get() -> u32 {
        unsafe { CURRENT_STACK_OFFSET }
    }

    pub fn set(val: u32) {
        unsafe { CURRENT_STACK_OFFSET = val };
    }
}

fn idle_task() {
    loop { }
}

/// Trait implemented by things which can block tasks
trait Block {
    /// Gets the "unique" block id. Note that it is unique only for the lifetime
    /// of the block, since it is just the memory location of the block. To
    /// keep weird behavior from occuring, we force a static lifetime.
    fn id(&'static self) -> u32 {
        &self as *const _ as u32
    }
}

#[derive(Copy, Clone)]
struct TaskDescriptor {
    state: TaskState,
    stack_start: u32,
    stack_end: u32
}

impl TaskDescriptor {
    fn new(t: TaskFn, stack_size: u32) -> TaskDescriptor {
        cortex_m::interrupt::free(|_| {
            let offset = current_stack_offset::get();
            let stack_size: u32 = (
                stack_size +
                8*4 + //Registers saved by nvic
                8*4 + //Registers saved by sw
                8+4 //Extra buffer
            ) & !0b1111;

            TaskDescriptor {
                state: TaskState::Ready,
                stack_start: stack_size,
                stack_end: 0
            }
        })
    }

    /// Blocks this task with the passed Block
    pub fn block(&mut self, block: &'static Block, _: &CriticalSection) {
        self.state = TaskState::Blocked(block.id());
    }

    /// Attempts to unblock this task
    ///
    /// Returns whether or not the task was transitioned from Blocked(_) to
    /// Ready.
    pub fn try_unblock(&mut self, block: &'static Block, _: &CriticalSection) -> bool {
        let block_id = block.id();
        match self.state {
            TaskState::Blocked(id) => {
                if id == block_id {
                    self.state = TaskState::Ready;
                    true
                }
                else {
                    false
                }
            },
            _ => false
        }
    }
}

/// Maximum task count, including the idle task
const MAX_TASKS_COUNT: usize = 4;

pub struct TaskCollection {
    current_task: usize,
    tasks: [TaskDescriptor; MAX_TASKS_COUNT]
}

pub static mut Tasks: TaskCollection = TaskCollection {
    current_task: 0,
    tasks: [TaskDescriptor { state: TaskState::Ready, stack_start: 0, stack_end: 0 }; MAX_TASKS_COUNT]
};

impl TaskCollection {
    pub fn add_task(mut self, t: TaskFn, stack_size: u32) -> TaskCollection {
        self
    }
    pub fn run(mut self) {
    }

}

pub struct Mutex {
}

pub struct Queue {
}

