//! Task module
//! This operates by having the pendsv interrupt perform context switches.

use core;
use core::u32;
use bare_metal::CriticalSection;
use cortex_m;

use rtos::Result;
use rtos::sched::switch_context;
use rtos::sync::Block;

/// Minimum required stack size in bytes
pub const MIN_STACK_SIZE: u32 = 4*8 + 4*8 + 4*8;

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

/// Task function type
pub type TaskFn = fn();

#[derive(Copy, Clone)]
pub struct TaskDescriptor {
    state: TaskState,
    stack_start: u32,
    stack_end: u32
}

impl TaskDescriptor {
    /// Returns a blank task descriptor
    pub const fn new() -> TaskDescriptor {
        TaskDescriptor {
            state: TaskState::Invalid,
            stack_start: 0,
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
                //TODO: Stack setup
                Ok(())
            },
            _ => Err(())
        }
    }

    /// Blocks this task with the passed Block and requests a context switch
    pub fn block(&mut self, block: &'static Block, _: &CriticalSection) {
        self.state = TaskState::Blocked(block.id());
        switch_context();
    }

    /// Attempts to unblock this task. If successful, a context switch is
    /// requested.
    ///
    /// Returns whether or not the task was transitioned from Blocked(_) to
    /// Ready.
    pub fn try_unblock(&mut self, block: &'static Block, _: &CriticalSection) -> bool {
        let block_id = block.id();
        match self.state {
            TaskState::Blocked(id) => {
                if id == block_id {
                    self.state = TaskState::Ready;
                    switch_context();
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

