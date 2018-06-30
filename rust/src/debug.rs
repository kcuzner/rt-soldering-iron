//! Debug subcomponent which provides a useful hard fault handler.
//!
//! I have found this to be indispensible for C-based firmware, so this is my
//! attempt at doing it in rust.

use cortex_m_rt as rt;

#[used]
#[no_mangle]
pub static mut HARD_FAULT_STACK: rt::ExceptionFrame = rt::ExceptionFrame {
    r0: 0,
    r1: 0,
    r2: 0,
    r3: 0,
    r12: 0,
    lr: 0,
    pc: 0,
    xpsr: 0,
};


/// User hard fault handler function. Called by cortex-m-rt when a hard fault occurs.
pub fn hard_fault_handler(ef: &rt::ExceptionFrame) -> ! {
    unsafe { HARD_FAULT_STACK.clone_from(ef) };
    loop { }
}

