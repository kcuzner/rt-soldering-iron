//! Minimal RTOS, based loosly on kos-avr and zinc
//!
//! This RTOS is optmized towards small memory footprint at the cost of code
//! size. This is a direct response to FreeRTOS's "large" memory footprint (a
//! TCB is 54 bytes and a semaphore is 168 bytes). In small microcontrollers
//! with extremely limited RAM (less than 4K), this is simply not acceptable,
//! especially with the size of a semaphore (strictly speaking, a semaphore
//! only needs 1 byte with some liberties taken on the TCB level).
//!
//! As part of this optimization, the RTOS does not support the following:
//! - Task removal.
//! - Tasks that return. There is no compiler enforcement for this as of yet.
//! - Task re-prioritization. The last task added is the lowest priority.
//! - First-come-first-serve task unblocking. Tasks are always unblocked in
//!   priority order.
//! - Round-robining. All tasks have a priority, so a higher priority task
//!   can never switch to a lower priority task unless it is explicitly
//!   blocked.
//!
//! Since this is designed to be used by microcontrollers, the method for stack
//! allocation is through static byte arrays declared outside the rtos module.
//! This should allow for quick checks of stack size allocation vs total
//! memory footprint. As always with microcontrollers, extreme care should be
//! taken when allocating items on the stack.
//!
//! Although discouraged generally by rust, the singleton pattern is used here
//! and certain items (such as the Block trait) require use of the static
//! lifetime. Proper use of `Cell` and `cortex_m::interrupt::free` should allow
//! `static mut` to be avoided.

mod tasks;
mod sched;
pub mod sync;

use core;

pub use self::sched::{add_task, run};

pub type Result<T> = core::result::Result<T, ()>;

