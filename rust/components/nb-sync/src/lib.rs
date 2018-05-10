//! Nonblocking synchronization structures

#![no_std]
#![feature(const_fn)]

extern crate nb;
extern crate cortex_m;

pub mod mpsc;

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// Mutex with interior mutability
///
/// This mutex assues that `panic!` is unrecoverable and crashes the program. This is a safe
/// assumption for this embedded application since the `panic!` transforms into two `udf`
/// instructions which result in a hard fault. If the user program incorporates any measures to
/// recover from this sort of hard fault, this mutex is no longer safe since it does not implement
/// the concept of "poisoning".
///
/// Since this mutex is polled and does not block, it is easy to have resource starvation occur.
/// Care should be taken as to where the `lock` function is called to allow other tasks to have an
/// opportunity to also grab the mutex.
pub struct Mutex<T> {
    data: UnsafeCell<T>,
    count: UnsafeCell<i32>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex in the unlocked state
    pub const fn new(val: T) -> Self {
        Mutex { data: UnsafeCell::new(val), count: UnsafeCell::new(1) }
    }

    /// Attempts to lock the mutex
    ///
    /// Once this function returns a `MutexGuard`, all other ongoing calls to `try_lock` will fail
    /// until the `MutexGuard` is `Drop`d.
    pub fn lock(&self) -> nb::Result<MutexGuard<T>, !> {
        // This critical section ensures that the references to our interior are safe.
        cortex_m::interrupt::free(|_| unsafe {
            if *self.count.get() > 0 {
                *self.count.get() -= 1;
                Ok(MutexGuard::new(&self))
            }
            else {
                Err(nb::Error::WouldBlock)
            }
        })
    }

    /// Consumes this mutex and returns the underlying data
    ///
    /// This is statically safe since we consume `self`.
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    /// Unlocks this mutex
    ///
    /// This is unsafe because it is not thread-safe. The caller must ensure that they are the
    /// exclusive owner of this mutex's inner value. A `MutexGuard` is an example of an object that
    /// can make such a guarantee.
    unsafe fn unlock(&self) {
        *self.count.get() += 1;
    }
}

impl<T> From<T> for Mutex<T> {
    /// Creates a new mutex from a value in an unlocked state
    fn from(v: T) -> Mutex<T> {
        Mutex::new(v)
    }
}

unsafe impl<T> Sync for Mutex<T> {
}

/// Scoped mutex access. This will unlock the mutex when dropped.
pub struct MutexGuard<'a, T:'a> {
    mutex: &'a Mutex<T>,
}

impl<'a, T: 'a> MutexGuard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        MutexGuard { mutex: mutex }
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    /// Releases this mutex guard
    fn drop(&mut self) {
        // Being the exclusive owner of the mutex, we can call the unlock method. Since this is
        // inside drop, we ensure that this is only called once by us.
        unsafe { self.mutex.unlock() }
    }
}

impl<'a, T: 'a> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: 'a> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

