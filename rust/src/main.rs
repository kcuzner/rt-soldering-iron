//! Firmware for the rt soldering iron

#![feature(used, const_fn, asm, naked_functions, const_size_of, const_unsafe_cell_new, const_ptr_null_mut, never_type, generators, generator_trait)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate panic_abort;
extern crate embedded_hal;
extern crate stm32f031x;
extern crate stm32f031x_hal;
extern crate board_support as bs;
extern crate nb_sync;
#[macro_use(await)]
extern crate nb;
extern crate cast;
extern crate fpa;
extern crate numtoa;

extern crate bare_take_mut as take_mut;

use core::ops::Generator;
use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut};

use embedded_hal::PwmPin;

use numtoa::NumToA;
use cast::u16;
use fpa::I16F16;

use stm32f031x_hal::rcc::RccExt;
use stm32f031x_hal::time::{U32Ext};
use stm32f031x_hal::flash::{FlashExt};
use stm32f031x_hal::gpio::GpioExt;
use stm32f031x_hal::i2c::{I2CExt, IntoScl, IntoSda, I2cTiming, I2cTimingSetting};
use stm32f031x_hal::adc::{AdcExt, IntoAnalog};
use stm32f031x_hal::pwm::{PwmExt, IntoPwm};

pub use bs::{TIM1_BRK_UP_IRQ, SysTick};
pub use debug::{HARD_FAULT_STACK};

mod debug;
mod font;
mod gfx;
mod pid;

#[derive(Copy, Clone)]
struct DisplayData {
    adc: u32,
    pid: u32,
}

impl DisplayData {
    pub fn new(adc: u32, pid: u32) -> Self {
        DisplayData { adc: adc, pid: pid }
    }
}

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

fn test() -> ! {
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
    let tim14 = peripherals.TIM14;

    // Display queue
    let mut buf: [Option<DisplayData>; 4] = [None, None, None, None];
    let mut chan = nb_sync::fifo::Channel::new(&mut buf);
    let (mut receiver, mut sender) = chan.split();

    // Set up the clocks
    rcc.cfgr = rcc.cfgr.sysclk(48.mhz())
        .hclk(48.mhz())
        .pclk(8.mhz());
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Calibrate the system tick for 1 ms
    bs::systick::calibrate(&mut syst, clocks.clone());

    // Create the I2C master
    let i2c = peripherals.I2C1.constrain(&mut rcc.apb1)
        .bind(gpiob.pb6.into_scl(&mut gpiob.regs), gpiob.pb7.into_sda(&mut gpiob.regs))
        .master(I2cTiming::from(I2cTimingSetting::Fast));

    // Create the buzzer
    let mut buzzer = bs::Buzzer::new(tim1, &mut rcc.apb2, &mut nvic, gpioa.pa8, &mut gpioa.regs);

    // Create the heater sense line
    let mut heater_sense = gpioa.pa0.into_analog_input(&mut gpioa.regs);

    // Create the SSD1306
    let mut ssd1306_reset = bs::ssd1306::Uninitialized::new(i2c, bs::ssd1306::SSD1306Address::Low, gpiob.pb3, &mut gpiob.regs).reset();

    // Create the heater PWM
    let mut heater_pwm = tim14.constrain_pwm(&mut rcc.apb1);
    let mut heater_pin = heater_pwm.ch1(gpioa.pa7.into_pwm(&mut gpioa.regs));

    // Create the heater controller
    heater_pin.set_duty(0);
    heater_pwm.set_period(clocks.clone(), 1.khz());
    heater_pwm.enable();
    heater_pwm.commit();
    heater_pin.enable();
    let mut heater_pid = pid::PID::new(pid::Constants::new(0.2f64, 0.1f64, 0.001f64), heater_pin);

    // Build a setpoint using fixed point arithmetic
    //let c_per_bit = I16F16(0.053618f64).unwrap(); //based on seebeck and bad assumptions
    let c_per_bit = I16F16(0.13431f64).unwrap(); //based on 500C = 3.0V calculation
    //let bits_per_c = I16F16(18.650f64).unwrap();
    let bits_per_c = I16F16(7.4455f64).unwrap();
    let setpoint = u16(I16F16(300i16) * bits_per_c).unwrap();

    // UI task
    let mut ui_fn = || {
        let mut ssd1306_init = await!(ssd1306_reset.poll()).unwrap().initialize(ssd1306_reset);
        let mut display_write = await!(ssd1306_init.poll()).unwrap().commit(ssd1306_init);
        buzzer.beep(100, 1000.hz(), clocks.clone());
        let mut now = bs::systick::now();
        let mut y = 0;
        let mut x = 0;
        loop {
            now = await!(bs::systick::wait_until(now + 100)).unwrap();
            let display_data = await!(receiver.recv()).unwrap();
            let mut display = await!(display_write.poll()).unwrap().finish(display_write);
            display.clear();
            let font = font::Font::EightByEight;
            let mut arr: [u8; 8] = [0; 8];
            {
                let c = I16F16(display_data.adc).unwrap() * c_per_bit;
                let start_index = u16(c).unwrap().numtoa(10, &mut arr);
                let string = unsafe { from_utf8_unchecked(&arr[start_index..]) };
                font.render_string(&string, gfx::Point::new(0, 0), &mut display).unwrap();
            }
            {
                let mut string = unsafe { from_utf8_unchecked_mut(&mut arr) };
                hex(display_data.pid, &mut string);
                font.render_string(&string, gfx::Point::new(0, 12), &mut display).unwrap();
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

    // Control task
    let mut adc_fn = || {
        let mut calibrated_adc = await!(uncalibrated_adc.poll()).unwrap().finish(uncalibrated_adc);
        loop {
            let mut conversion = calibrated_adc.single(heater_sense);
            let (adc, hs, value) = await!(conversion.poll()).unwrap().finish(conversion);
            let pid_output = heater_pid.step(setpoint, value.raw_right() as u16);
            heater_pwm.commit();
            let data = DisplayData::new(value.raw_right(), pid_output.into());
            //yuck. Send inside a vanilla await! requires Copy.
            match sender.send(data) {
                _ => ()
            };
            await!(sender.send(data)).unwrap();
            calibrated_adc = adc;
            heater_sense = hs;
        }
    };

    // Dummy task for fun
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

entry!(main);

fn main() -> ! {
    test()
}

exception!(HardFault, debug::hard_fault_handler);
exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    panic!();
}

