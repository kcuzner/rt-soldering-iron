//! Task module
//! This operates by having the pendsv interrupt perform context switches.

use core;
use core::u32;
use core::mem::size_of;
use bare_metal::CriticalSection;
use cortex_m;

use rtos::Result;
use rtos::sync::Block;

const REGISTER_SIZE: u32 = size_of::<u32>() as u32;
/// Minimum required stack size in bytes.
///
/// 32 bytes for SavedState
/// 32 bytes for r4-r12
/// 32 bytes for stack space
pub const MIN_STACK_SIZE: u32 = REGISTER_SIZE*8 + REGISTER_SIZE*8 + REGISTER_SIZE*8;

/// State of a task
#[derive(Copy, Clone)]
enum TaskState {
    /// The task is invalid
    Invalid,
    /// The task is ready
    Ready,
    /// The task is blocked on something
    Blocked(u32),
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

/// Task function type
pub type TaskFn = fn();

/// State saved by the NVIC when an interrupt occurs
///
/// This is used to set up the initial stack contents for when a task is
/// created. Note the repr(C), which should make r0 have the lowest address,
/// being on top of the stack (stack is full descending).
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
    ///
    /// This sets up the following:
    /// - R0-R3, R12: Set to zero
    /// - LR: Points to the task return handler function just in case the task
    ///   decides to `bx lr` or `pop {pc}`
    /// - PC: Points to the task function so that the task will resume into the
    ///   beginning of the function
    /// - PSR: Sets the process for thumb mode
    fn new(t: TaskFn) -> SavedState {
        SavedState {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
            lr: task_return_handler as u32,
            pc: t as u32,
            psr: 0x01000000
        }
    }
}

/// Catch function for when a task decides to return. Just an infinite loop.
#[inline(never)]
fn task_return_handler() {
    loop {}
}

/// Task descriptor. Contains information about the state and stack of a
/// particular task.
#[derive(Copy, Clone)]
pub struct TaskDescriptor {
    /// Current task state
    state: TaskState,
    /// Top of the stack
    stack_top: u32,
    /// Lowest permissible address for this stack
    stack_end: u32
}

impl TaskDescriptor {
    /// Returns a blank task descriptor
    pub const fn new() -> TaskDescriptor {
        TaskDescriptor {
            state: TaskState::Invalid,
            stack_top: 0,
            stack_end: 0
        }
    }
    /// Sets up a task descriptor for the passed task function
    ///
    /// This requires a critical section
    pub fn setup(&mut self, t: TaskFn, stack: &'static[u8], _: &CriticalSection) -> Result<()> {
        match self.state {
            TaskState::Invalid => {
                self.state = TaskState::Ready;
                self.stack_end = stack.as_ptr() as u32;
                self.stack_top = (self.stack_end - stack.len() as u32) - size_of::<SavedState>() as u32;
                // Add the NVIC-saved state to resume into the task entry point
                unsafe { *(self.stack_top as *mut SavedState) = SavedState::new(t) };
                // Add r4-r7 and r8-r12 on top
                self.stack_top -= REGISTER_SIZE*8;
                Ok(())
            },
            _ => Err(())
        }
    }

    /// Blocks this task with the passed Block
    pub fn block(&mut self, block: &'static Block, _: &CriticalSection) -> Result<()> {
        match self.state {
            TaskState::Invalid => Err(()),
            _ => {
                self.state = TaskState::Blocked(block.id());
                Ok(())
            }
        }
    }

    /// Attempts to unblock this task.
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

    /// Returns whether or not this task is ready
    pub fn is_ready(&self) -> bool {
        match self.state {
            TaskState::Ready => true,
            _ => false
        }
    }

    /// Gets the previously set task stack top memory address
    pub fn stack_top(&self, _: &CriticalSection) -> Result<u32> {
        match self.state {
            TaskState::Invalid => Err(()),
            _ => Ok(self.stack_top)
        }
    }

    /// Stores the stack top memory address
    pub fn set_stack_top(&mut self, sp: u32, _: &CriticalSection) -> Result<()> {
        match self.state {
            TaskState::Invalid => Err(()),
            _ => { self.stack_top = sp; Ok(()) }
        }
    }
}

