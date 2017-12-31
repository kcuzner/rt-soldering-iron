//! RTOS basic synchronization primitives.
//!
//! All of these primitives must be declared in the 'static lifetime. This
//! prevents issues such as tasks being blocked in a mutex whose lifetime has
//! ended or two mutexes sharing the same memory location (as could easily
//! happen if they were located on the stack)

/// Trait implemented by things which can block tasks
pub trait Block {
    /// Gets the "unique" block id. Note that it is unique only for the lifetime
    /// of the block, since it is just the memory location of the block. To
    /// keep weird behavior from occuring, we force a static lifetime.
    fn id(&'static self) -> u32 {
        &self as *const _ as u32
    }
}


pub struct Mutex {
}

impl Block for Mutex {
}

pub struct Queue {
}

impl Block for Queue {
}

