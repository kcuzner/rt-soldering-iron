//! This crate is an implementation of take_mut which does not
//! depend on the standard library.

#![no_std]
#![feature(use_extern_macros)]

/// Allows use of a value pointed to by `&mut T` as though it was owned, as long as a `T` is made available afterwards.
///
/// The closure must return a valid T.
/// # Important
/// Will abort the program if the closure panics.
///
/// # Example
/// ```
/// struct Foo;
/// let mut foo = Foo;
/// take_mut::take(&mut foo, |foo| {
///     // Can now consume the Foo, and provide a new value later
///     drop(foo);
///     // Do more stuff
///     Foo // Return new Foo from closure, which goes back into the &mut Foo
/// });
/// ```
pub fn take<T, F>(mut_ref: &mut T, closure: F)
  where F: FnOnce(T) -> T {
    use core::ptr;

    unsafe {
        let old_t = ptr::read(mut_ref);
        let new_t = closure(old_t);
        ptr::write(mut_ref, new_t);
    }
}
