//! Firmware for the rt soldering iron

#![feature(used, const_fn, asm, naked_functions, const_size_of, const_unsafe_cell_new, const_ptr_null_mut, never_type, generators, generator_trait)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate embedded_hal;
extern crate stm32f031x;
extern crate stm32f031x_hal;
extern crate board_support as bs;
extern crate nb_sync;
#[macro_use(await)]
extern crate nb;

extern crate bare_take_mut as take_mut;

use core::ops::Generator;
use core::str::from_utf8_unchecked_mut;

use stm32f031x_hal::rcc::RccExt;
use stm32f031x_hal::time::{U32Ext};
use stm32f031x_hal::flash::{FlashExt};
use stm32f031x_hal::gpio::GpioExt;
use stm32f031x_hal::i2c::{I2CExt, IntoScl, IntoSda, I2cTiming, I2cTimingSetting};
use stm32f031x_hal::adc::{AdcExt, IntoAnalog};

use embedded_hal::digital::OutputPin;

pub use bs::{TIM1_BRK_UP_IRQ, SYS_TICK};
pub use debug::{HARD_FAULT, HARD_FAULT_STACK};

mod debug;
mod font;
mod gfx;

/// Performs an integer to hex conversion
///
/// Requires an &str of at least 8 bytes length
fn hex(value: u32, out: &mut str) {
    unsafe {
        let bytes = out.as_bytes_mut();
        for i in 0..8 {
            let shift = i * 4;
            let mask = 0xF0000000 >> shift;
            let nibble = ((value & mask) >> 28 - shift) as u8;
            bytes[i] = match nibble {
                0...9 => b'0' + nibble,
                10...15 => b'A' + (nibble - 10),
                _ => b'Z',
            }
        }
    }
}

fn test() {
    let core_peripherals = stm32f031x::CorePeripherals::take().unwrap();
    let peripherals = stm32f031x::Peripherals::take().unwrap();
    let mut syst = core_peripherals.SYST;
    let mut nvic = core_peripherals.NVIC;
    let mut rcc = peripherals.RCC.constrain();
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let mut flash = peripherals.FLASH.constrain();
    let mut uncalibrated_adc = peripherals.ADC.constrain(&mut rcc.apb2);
    let tim1 = peripherals.TIM1;

    rcc.cfgr = rcc.cfgr.sysclk(48.mhz())
        .hclk(48.mhz())
        .pclk(8.mhz());
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    bs::systick::calibrate(&mut syst, clocks.clone());

    //TODO: Put this in the SSD1306...I don't even know what its doing here...
    let mut reset = gpiob.pb3.into_output_open_drain_pull_up(&mut gpiob.regs);
    for _i in 0..80100 {
        reset.set_low();
    }
    reset.set_high();

    let i2c = peripherals.I2C1.constrain(&mut rcc.apb1)
        .bind(gpiob.pb6.into_scl(&mut gpiob.regs), gpiob.pb7.into_sda(&mut gpiob.regs))
        .master(I2cTiming::from(I2cTimingSetting::Fast));
    let mut buzzer = bs::Buzzer::new(tim1, &mut rcc.apb2, &mut nvic, gpioa.pa8, &mut gpioa.regs);

    let mut heater_sense = gpioa.pa0.into_analog_input(&mut gpioa.regs);
    let mut array: [u8; 8] = [0; 8];
    array[..].clone_from_slice("000000ZZ".as_bytes());
    let shared_value: nb_sync::Mutex<&mut str> = nb_sync::Mutex::new(unsafe { from_utf8_unchecked_mut(&mut array[..]) });

    let mut ui_fn = || {
        let mut ssd1306_init = bs::ssd1306::Uninitialized::new(i2c, bs::ssd1306::SSD1306Address::Low).initialize();
        let mut display_write = await!(ssd1306_init.poll()).unwrap().commit(ssd1306_init);
        buzzer.beep(100, 1000.hz(), clocks.clone());
        let mut now = bs::systick::now();
        let mut y = 0;
        let mut x = 0;
        loop {
            now = await!(bs::systick::wait_until(now + 100)).unwrap();
            let mut display = await!(display_write.poll()).unwrap().finish(display_write);
            display.clear();
            let font = font::Font::EightByEight;
            {
                let string = await!(shared_value.lock()).unwrap();
                font.render_string(&string, gfx::Point::new(0, 0), &mut display).unwrap();
            }
            display.hline(0, y, 127).unwrap();
            display.vline(x, 0, 31).unwrap();
            y += 1;
            if y >= 32 {
                y = 0;
            }
            x += 1;
            if x >= 128 {
                x = 0;
            }
            display_write = display.commit();
        }
    };
    let mut adc_fn = || {
        let mut calibrated_adc = await!(uncalibrated_adc.poll()).unwrap().finish(uncalibrated_adc);
        loop {
            let mut conversion = calibrated_adc.single(heater_sense);
            let (adc, hs, value) = await!(conversion.poll()).unwrap().finish(conversion);
            {
                let mut string = await!(shared_value.lock()).unwrap();
                hex(value.into(), &mut string)
            }
            calibrated_adc = adc;
            heater_sense = hs;
        }
    };
    let mut incr_fn = || {
        let mut now = bs::systick::now();
        loop {
            now = await!(bs::systick::wait_until(now + 100)).unwrap();
            /*{
                let mut string = await!(shared_value.lock()).unwrap();
                unsafe {
                    let bytes = string.as_bytes_mut();
                    bytes[0] += 1;
                    if bytes[0] > b'Z' {
                        bytes[0] = b'A';
                    }
                }
            }*/
        }
    };
    loop {
        unsafe { ui_fn.resume() };
        unsafe { adc_fn.resume() };
        unsafe { incr_fn.resume() };
    }
}

fn main() {
    test()
}

// As we are not using interrupts, we just register a dummy catch all handler
/*#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];

extern "C" fn default_handler() {}*/

default_handler!(exception_handler);
fn exception_handler() {}
