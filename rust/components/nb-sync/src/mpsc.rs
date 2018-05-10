//! Multi-producer Single-Consumer FIFO implemented using the `nb` non-blocking I/O API.
//!
//! As this is meant to be used in a `no_std` environment, there are no heap allocations. This
//! requires extra lifetime parameters to explicitly keep references alive for the lifetime of
//! channel senders and receivers.

use nb;
use core::cell::UnsafeCell;
use core::mem::swap;
use cortex_m;

/// Channel for a multi-producer, single consumer FIFO
pub struct Channel<'a, T: 'a> {
    buffer: UnsafeCell<&'a mut [Option<T>]>,
    send_index: UnsafeCell<usize>,
    recv_index: UnsafeCell<usize>,
}

impl<'a, T: 'a> Channel<'a, T> {
    /// Creates a new channel
    ///
    /// The passed buffer will be borrowed for the lifetime of this channel and will serve as the
    /// shared storage between the sender and receiver.
    ///
    /// To aid in optimization, the length of the slice should be a power of 2. But it doesn't have
    /// to be.
    pub fn new(buffer: &'a mut [Option<T>]) -> Self {
        // clear the buffer
        for el in buffer.iter_mut() {
            *el = None;
        }
        // create the channel
        Channel {
            buffer: UnsafeCell::new(buffer),
            send_index: UnsafeCell::new(0),
            recv_index: UnsafeCell::new(0),
        }
    }
}

impl<'a, 'b: 'a, T: 'b> Channel<'b, T> {
    /// Builds a sender and receiver for this channel
    ///
    /// The mutable borrow of self in this function will be for as long as the lifetimes of the
    /// receiver and sender. This ensures that the Channel stays in one place in memory while the sender
    /// and receiver are doing their thing.
    pub fn build(&'a mut self) -> (Receiver<'a, 'b, T>, Sender<'a, 'b, T>) {
        (Receiver::new(self), Sender::new(self))
    }

    /// Gets the length of this channel's buffer
    pub fn len(&self) -> usize {
        // This is safe because our slice's len cannot change
        unsafe { (*self.buffer.get()).len() }
    }

    /// Returns whether or not the underlying buffer is empty
    ///
    /// Unsafe because the caller must ensure thread safety here.
    unsafe fn empty(&self) -> bool {
        *self.send_index.get() == *self.recv_index.get()
    }

    /// Returns whether or not the underlying buffer is full
    ///
    /// Unsafe because the caller must ensure thread safety here.
    unsafe fn full(&self) -> bool {
        ((*self.send_index.get()) + 1) % (self.len()) == *self.recv_index.get()
    }
}

pub struct Receiver<'a, 'b: 'a, T: 'b> {
    inner: &'a Channel<'b, T>,
}

impl<'a, 'b: 'a, T: 'b> Receiver<'a, 'b, T> {
    fn new(channel: &'a Channel<'b, T>) -> Self {
        Receiver { inner: channel }
    }

    pub fn recv(&mut self) -> nb::Result<T, !> {
        // This is safe because only one receiver can exist at a time for each channel.
        if unsafe { self.inner.empty() } {
            Err(nb::Error::WouldBlock)
        }
        else {
            let mut val: Option<T> = None;
            unsafe {
                let index = *self.inner.recv_index.get();
                // This is safe because the recv_index and send_index don't reference the same item
                // in the array unless it is empty (in which case we already returned WouldBlock).
                // Between then and now, the buffer could not have become empty.
                swap(&mut val, &mut (*self.inner.buffer.get())[index]);
                *self.inner.recv_index.get() = (index + 1) % self.inner.len();
            }
            match val {
                None => Err(nb::Error::WouldBlock),
                Some(v) => Ok(v),
            }
        }
    }
}

pub struct Sender<'a, 'b: 'a, T: 'b> {
    inner: &'a Channel<'b, T>,
}

impl<'a, 'b: 'a, T: 'b> Sender<'a, 'b, T> {
    fn new(channel: &'a Channel<'b, T>) -> Self {
        Sender { inner: channel }
    }

    pub fn send(&self, value: T) -> nb::Result<(), !> {
        cortex_m::interrupt::free(|_| {
            // This is safe because we are in an interrupt free context
            if unsafe { self.inner.full() } {
                Err(nb::Error::WouldBlock)
            }
            else {
                let mut val: Option<T> = Some(value);
                // This is all safe because we are in an interrupt free context
                unsafe {
                    let index = *self.inner.send_index.get();
                    swap(&mut val, &mut (*self.inner.buffer.get())[index]);
                    *self.inner.send_index.get() = (index + 1) & self.inner.len();
                }
                Ok(())
            }
        })
    }
}

