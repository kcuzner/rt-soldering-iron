//! System timer abstraction layer

use cortex_m;
use stm32f031x::SYST;
use stm32f031x_hal::rcc::Clocks;
use nb;

static mut TICK_COUNT: u32 = 0;

exception!(SysTick, systick);

fn systick() {
    cortex_m::interrupt::free(|_| {
        unsafe { TICK_COUNT += 1; }
    });
}

/// Gets the current tick
pub fn now() -> u32 {
    // atomic read, no side effects
    unsafe { TICK_COUNT }
}

/// Nonblocking function waiting until a certain time has arrived
pub fn wait_until(tick: u32) -> nb::Result<u32, !> {
    cortex_m::interrupt::free(|_| unsafe {
        if TICK_COUNT >= tick {
            Ok(TICK_COUNT)
        }
        else {
            Err(nb::Error::WouldBlock)
        }
    })
}

/// Calibrates the system tick to have a 1ms period with the passed clocks
pub fn calibrate(syst: &mut SYST, clocks: Clocks) {
    let stclock = clocks.hclk().0 / 8;
    syst.set_reload(stclock / 1000);
    syst.clear_current();
    syst.enable_counter();

    syst.enable_interrupt();
}

