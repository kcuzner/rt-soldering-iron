//! RTOS basic synchronization primitives.
//!
//! All of these primitives must be declared in the 'static lifetime. This
//! prevents issues such as tasks being blocked in a mutex whose lifetime has
//! ended or two mutexes sharing the same memory location (as could easily
//! happen if they were located on the stack)

use core::ops::{Deref, DerefMut};
use core::cell::UnsafeCell;
use core::ptr;
use cortex_m;
use bare_metal::CriticalSection;

use rtos::sched::{block, unblock, switch_context};

/// Trait implemented by things which can block tasks
pub trait Block {
    /// Gets the "unique" block id. Note that it is unique only for the lifetime
    /// of the block, since it is just the memory location of the block. To
    /// keep weird behavior from occuring, we force a static lifetime.
    fn id(&'static self) -> u32 {
        &self as *const _ as u32
    }
}

/// Unsafe List
///
/// This list is unsafe because it discards lifetimes. The nodes are meant to
/// be stack-allocated. They must live for the duration of time that the node
/// is present in the list.
struct UnsafeList<T> {
    head: *mut UnsafeListNode<T>
}

/// Node in an unsafe list.
struct UnsafeListNode<T> {
    data: *mut T,
    next: *mut UnsafeListNode<T>
}

impl<T> UnsafeListNode<T> {
    const fn new() -> Self {
        UnsafeListNode { data: ptr::null_mut(), next: ptr::null_mut() }
    }
}

/// Safe to share: All manipulations of this type should be done through
/// UnsafeList<T>
unsafe impl<T> Sync for UnsafeListNode<T> {
}

impl<T> UnsafeList<T> {
    const fn new() -> Self {
        UnsafeList { head: ptr::null_mut() }
    }

    /// Appends the passed node to this list
    ///
    /// The caller must ensure that the node lives at least as long as it is
    /// on this list.
    unsafe fn append(&mut self, node: *mut UnsafeListNode<T>, _: &CriticalSection) {
        // Important note: This does not use recursion, as I am not confident
        // it will be tail-optimized. We have extremely limited stack space and
        // the recursion depth will increase with O(N) tasks.
        match self.head.as_mut() {
            None => self.head = node,
            Some(head) => {
                let mut current = head;
                loop {
                    match current.next.as_mut() {
                        Some(n) => current = n,
                        None => {
                            current.next = node;
                            break;
                        }
                    };
                }
            }
        };
    }

    /// Removes the passed node from this list
    unsafe fn remove(&mut self, node: *mut UnsafeListNode<T>, _: &CriticalSection) {
        // Important note: This does not use recursion, as I am not confident
        // it will be tail-optimized. We have extremely limited stack space and
        // the recursion depth will increase with O(N) tasks.
        match self.head.as_mut() {
            None => { },
            Some(head) => {
                if self.head == node {
                    self.head = head.next;
                }
                else {
                    let mut current = head;
                    while let Some(next) = current.next.as_mut() {
                        if current.next == node {
                            match node.as_mut() {
                                None => { /* We should never get here */ },
                                Some(n) => current.next = n.next
                            };
                            break;
                        }
                        else {
                            current = next;
                        }
                    }
                }
            }
        }
    }
}

/// Safe to share: All methods require a CriticalSection to be invoked, which
/// guarantees sequential access.
unsafe impl<T> Sync for UnsafeList<T> {
}

/// Counting semaphore
pub struct Semaphore {
    count: UnsafeCell<i16>,
}

pub struct SemaphoreGuard {
    __i: u8,
    __lock: &'static Semaphore
}

impl Semaphore {
    /// Creates a new semaphore around the passed value
    pub const fn new(count: u8) -> Self {
        Semaphore { count: UnsafeCell::new(count as i16) }
    }

    /// Atomically waits on this mutex
    pub fn wait(&'static self, i: u8) -> SemaphoreGuard {
        loop {
            // Execute this sequence without the possibility for interruption to
            // guarantee immunity from data races.
            let acquired = cortex_m::interrupt::free(|cs| {
                let count = self.count.get();
                match unsafe { *count } {
                    n if n >= i as i16 => {
                        unsafe { *count -= i as i16 };
                        true
                    },
                    _ => {
                        // If we are unable to acquire, make this task block.
                        // It would be nicer if this was associated with the
                        // switch_context later, but we need interrupts
                        // enabled for that to happen. Therefore, we block here
                        // and then continue with the switch context after
                        // interrupts are re-enabled.
                        match block(self, cs) {
                            Err(_) => loop { },
                            _ => false
                        }
                    }
                }
            });

            if !acquired {
                unsafe { switch_context() };
            }
            else {
                return SemaphoreGuard { __lock: self, __i: i }
            }
        }
    }
}

impl Block for Semaphore {
}

/*impl<T> Deref for MutexGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.__lock.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.inner.get() }
    }
}*/

impl Drop for SemaphoreGuard {
    #[inline]
    fn drop(&mut self) {
        cortex_m::interrupt::free(|cs| {
            unsafe { *(self.__lock.count.get()) += 1 };
            unblock(self.__lock, cs);
        });
        unsafe { switch_context() };
    }
}

pub struct Queue {
}

impl Block for Queue {
}

/// Data used for waiting on a tick
struct WaitData {
    task_index: u32,
    wake_tick: u32
}

/// Structure that tracks global wait data
struct TickWait {
    waiting: UnsafeList<WaitData>,
    tick: u32
}

impl TickWait {
    const fn new() -> Self {
        TickWait { waiting: UnsafeList::new(), tick: 0 }
    }

    /// Gets the current tick count
    fn tick(&self) -> u32 {
        self.tick
    }

    /// Performs next-tick operations
    fn next_tick(&mut self) {
    }

    /// Schedules the current task to wait.
    ///
    /// Warning! This will effectively be called by multiple threads at the
    /// same time, so a mutable borrow occurs all the time. This should be safe
    /// because we require a critical section when this is called.
    unsafe fn wait(&mut self, until_tick: u32, cs: &CriticalSection) {

    }
}

impl Block for TickWait {
}


static TICK_WAIT: TickWait = TickWait::new();

/// Causes the current task to block for the passed number of SysTick events
pub fn wait(ticks: u32) {
    cortex_m::interrupt::free(|cs| {
        match block(&TICK_WAIT, cs) {
            Err(_) => loop { }
            _ => { }
        }
    });
}

