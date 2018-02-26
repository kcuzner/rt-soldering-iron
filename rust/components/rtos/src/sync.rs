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

use Result;
use sched::{block, unblock, switch_context, task_id};

static mut CURRENT_BLOCK_ID: u32 = 0;

/// Tuple struct of an idenifier
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Id(u32);

/// Holds a unique idenfier for use with synchronization primitives
pub struct UniqueId {
    id: UnsafeCell<Option<Id>>
}

impl UniqueId {
    /// Creates a new unique id
    pub const fn new() -> Self {
        UniqueId { id: UnsafeCell::new(None) }
    }

    /// Gets the unique id value
    pub fn get(&self, _: &CriticalSection) -> Id {
        // This get is safe because id is Copy an we are in a critical section
        // which makes this function atomic, so no other call is inspecting
        // the value.
        match unsafe { *self.id.get() } {
            Some(id) => id,
            None => {
                // This unsafe block is safe because it is executed in a
                // critical section. The read-modify-write is therefore
                let id = unsafe {
                    let i = CURRENT_BLOCK_ID;
                    CURRENT_BLOCK_ID += 1;
                    i
                };
                unsafe { *self.id.get() = Some(Id(id)) };
                Id(id)
            }
        }
    }
}

/// UniqueId is sync because all access methods are atomic through the use of
/// CriticalSections. It is therefore safe to access in multiple threads.
unsafe impl Sync for UniqueId {
}

/// Trait implemented by things which can block tasks
pub trait Block {
    /// Returns a unique block id.
    fn id(&self, cs: &CriticalSection) -> Id;
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
    const fn new(data: *mut T) -> Self {
        UnsafeListNode { data: data, next: ptr::null_mut() }
    }

    unsafe fn data<'a>(&self) -> Option<&'a T> {
        self.data.as_ref()
    }

    unsafe fn next(&self) -> *mut UnsafeListNode<T> {
        self.next
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
                    match current.next().as_mut() {
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
    id: UniqueId,
    count: UnsafeCell<i16>,
}

pub struct SemaphoreGuard {
    __i: u8,
    __lock: &'static Semaphore
}

impl Semaphore {
    /// Creates a new semaphore around the passed value
    pub const fn new(count: u8) -> Self {
        Semaphore { id: UniqueId::new(), count: UnsafeCell::new(count as i16) }
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
    fn id(&self, cs: &CriticalSection) -> Id {
        self.id.get(cs)
    }
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
    fn drop(&mut self) {
        cortex_m::interrupt::free(|cs| {
            unsafe { *(self.__lock.count.get()) += 1 };
            unblock(self.__lock, cs);
        });
        unsafe { switch_context() };
    }
}

pub struct Queue {
    id: UniqueId
}

impl Block for Queue {
    fn id(&self, cs: &CriticalSection) -> Id {
        self.id.get(cs)
    }
}

/// Data used for waiting on a tick
struct WaitData {
    task_index: u32,
    wake_tick: u32
}

/// Structure that tracks global wait data
struct TickWait {
    id: UniqueId,
    waiting: UnsafeList<WaitData>,
    tick: u32
}

impl TickWait {
    const fn new() -> Self {
        TickWait { id: UniqueId::new(), waiting: UnsafeList::new(), tick: 0 }
    }

    /// Gets the current tick count
    fn tick(&self, _: &CriticalSection) -> u32 {
        self.tick
    }

    /// Performs next-tick operations
    fn next_tick(&mut self) {
    }

    /// Schedules the current task to wait in a interrupt-safe manner
    fn wait(&'static mut self, until_tick: u32) -> Result<()> {
        let mut wd = cortex_m::interrupt::free(|cs| {
            let id = task_id(&cs)?;
            Ok(WaitData { task_index: id, wake_tick: until_tick })
        })?;

        let mut node = UnsafeListNode::new(&mut wd);

        cortex_m::interrupt::free(|cs| {
            unsafe { self.waiting.append(&mut node, &cs) };
            block(self, &cs)?;
            Ok(())
        })?;

        while cortex_m::interrupt::free(|cs| { self.tick(&cs) < until_tick }) {
            unsafe { switch_context() };
        }

        cortex_m::interrupt::free(|cs| {
            unsafe { self.waiting.remove(&mut node, &cs) };
        });

        Ok(())
    }
}

impl Block for TickWait {
    fn id(&self, cs: &CriticalSection) -> Id {
        self.id.get(cs)
    }
}


static TICK_WAIT: TickWait = TickWait::new();

/// Causes the current task to block for the passed number of SysTick events
pub fn wait(until_tick: u32)  {
}

