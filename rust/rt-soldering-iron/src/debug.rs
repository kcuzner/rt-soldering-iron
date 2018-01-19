//! Debug subcomponent which provides a useful hard fault handler.
//!
//! I have found this to be indispensible for C-based firmware, so this is my
//! attempt at doing it in rust.

/// State saved by the NVIC when an interrupt occurs
///
/// Used to read out the hard fault stack to assist in debugging.
#[repr(C)]
#[derive(Clone)]
pub struct NVICState {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32
}

impl NVICState {
    const fn new() -> Self {
        NVICState {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0,
            lr: 0,
            pc: 0,
            psr: 0
        }
    }
}

#[used]
#[no_mangle]
pub static mut HARD_FAULT_STACK: NVICState = NVICState::new();

/// Rust side of the debug hard fault handler. Takes the the top of the stack
/// as its first argument, which is read as an NVICState since this is called
/// only from inside a fault handler.
unsafe extern "C" fn hard_fault_handler(stack_top: *const NVICState) {
    match stack_top.as_ref() {
        Some(s) => HARD_FAULT_STACK.clone_from(s),
        _ => ()
    }
    loop { }
}

/// Hard Fault handler
///
/// This interrupt handler is implemented in assembly, targeting the thumbv6m
/// instruction set.
#[naked]
#[no_mangle]
#[cfg(target_arch = "arm")]
pub extern "C" fn HARD_FAULT() {
    unsafe {
        asm!("movs r0, #4 //LR will have 0xFFFFFFF9 or 0xFFFFFFFD.
            mov r1, lr    //If it ends in 0xD, then psp is the stack
            tst r0, r1    //Otherwise, msp is the stack. The stack is
            beq 1f        //used as the first argument to the hard fault
            mrs r0, psp   //handler.
            b 2f
            1:
            mrs r0, msp
            2:
            ldr r1, [r0, #20]
            b $0
            b ." : : "i"(hard_fault_handler as unsafe extern "C" fn(*const NVICState)) : : "volatile");
    }
}



