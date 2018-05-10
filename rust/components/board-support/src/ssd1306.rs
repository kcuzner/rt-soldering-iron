//! SSD1306 Module

use core::convert::From;

use nb;
use bare_take_mut as take_mut;
use embedded_hal::digital::OutputPin;
use stm32f031x_hal::gpio::gpiob;
use stm32f031x_hal::gpio::{Output, OpenDrainPullUp};
use stm32f031x_hal::i2c;

use systick;

// A word about implementation:
//
// I started out wanting to make myself some enums for the commands
// where I could ensure that the right arguments only were sent and
// that they were properly formatted. However, I realized that this
// would take up far more code space than some blobs. Therefore,
// the command sequences for initializing the display and
// transferring a page to be displayed are represented as binary blobs.

#[allow(dead_code)]
const CMD_SET_CONTRAST: u8 = 0x81;
#[allow(dead_code)]
const CMD_DISPLAY_ALL_ON: u8 = 0xa5;
#[allow(dead_code)]
const CMD_DISPLAY_ALL_ON_RESUME: u8 = 0xa4;
#[allow(dead_code)]
const CMD_NORMAL_DISPLAY: u8 = 0xa6;
#[allow(dead_code)]
const CMD_INVERT_DISPLAY: u8 = 0xa7;
#[allow(dead_code)]
const CMD_DISPLAY_OFF: u8 = 0xae;
#[allow(dead_code)]
const CMD_DISPLAY_ON: u8 = 0xaf;
#[allow(dead_code)]
const CMD_SET_DISPLAY_OFFSET: u8 = 0xd3;
#[allow(dead_code)]
const CMD_SET_COM_PINS: u8 = 0xda;
#[allow(dead_code)]
const CMD_SET_V_COM_DETECT: u8 = 0xdb;
#[allow(dead_code)]
const CMD_SET_DISPLAY_CLOCK_DIV: u8 = 0xd5;
#[allow(dead_code)]
const CMD_SET_PRECHARGE: u8 = 0xd9;
#[allow(dead_code)]
const CMD_SET_MULTIPLEX: u8 = 0xa8;
#[allow(dead_code)]
const CMD_SET_LOW_COLUMN: u8 = 0x00;
#[allow(dead_code)]
const CMD_SET_HIGH_COLUMN: u8 = 0x10;
#[allow(dead_code)]
const CMD_SET_START_LINE: u8 = 0x40;
#[allow(dead_code)]
const CMD_MEMORY_MODE: u8 = 0x20;
#[allow(dead_code)]
const CMD_COLUMN_ADDR: u8 = 0x21;
#[allow(dead_code)]
const CMD_PAGE_ADDR: u8 = 0x22;
#[allow(dead_code)]
const CMD_COM_SCAN_INC: u8 = 0xc0;
#[allow(dead_code)]
const CMD_COM_SCAN_DEC: u8 = 0xc8;
#[allow(dead_code)]
const CMD_SEG_REMAP: u8 = 0xa0;
#[allow(dead_code)]
const CMD_CHARGE_PUMP: u8 = 0x8d;

/// Command sequence for initializing the SSD1306
static INITIALIZATION_COMMANDS: [u8; 50] = [
    0x00, CMD_DISPLAY_OFF,
    0x00, CMD_SET_DISPLAY_CLOCK_DIV,
    0x00, 0x80,
    0x00, CMD_SET_MULTIPLEX,
    0x00, 31,
    0x00, CMD_SET_DISPLAY_OFFSET,
    0x00, 0,
    0x00, CMD_SET_START_LINE | 0x00,
    0x00, CMD_CHARGE_PUMP,
    0x00, 0x14, //we only use the charge pump
    0x00, CMD_MEMORY_MODE,
    0x00, 0,
    0x00, CMD_SEG_REMAP | 0x01,
    0x00, CMD_COM_SCAN_DEC,
    0x00, CMD_SET_COM_PINS,
    0x00, 0x02, //128x32
    0x00, CMD_SET_CONTRAST,
    0x00, 0x8F,
    0x00, CMD_SET_PRECHARGE,
    0x00, 0xF1,
    0x00, CMD_SET_V_COM_DETECT,
    0x00, 0x40,
    0x00, CMD_DISPLAY_ALL_ON_RESUME,
    0x00, CMD_NORMAL_DISPLAY,
    0x00, CMD_DISPLAY_ON,
];

static DISPLAY_COMMANDS: [u8; 12] = [
    0x00, CMD_COLUMN_ADDR,
    0x00, 0,
    0x00, 127,
    0x00, CMD_PAGE_ADDR,
    0x00, 0,
    0x00, 3,
];

#[derive(Copy, Clone)]
pub enum SSD1306Address {
    Low,
    High
}

impl From<SSD1306Address> for u8 {
    fn from(addr: SSD1306Address) -> u8 {
        match addr {
            SSD1306Address::Low => 0x78,
            SSD1306Address::High => 0x7a,
        }
    }
}

/// Writes SSD1306 command sequences
///
/// The SSD1306 listens to 2-byte command sequences which consist of a 0x00
/// followed by a single byte specifying a command or an argument for the
/// previous command. This struct will chunk a passed slice into 2-byte
/// segments.
struct CommandWrite {
    addr: SSD1306Address,
    write: i2c::MasterSliceWrite,
    data: &'static [u8],
    index: usize
}

impl CommandWrite {
    /// Creates a new commandwrite. The passed command slice must conform to the
    /// followinb standards:
    /// - It must have a length greater than or equal to 2
    /// - It must have an even length
    fn new(master: i2c::MasterI2c, address: SSD1306Address, commands: &'static [u8]) -> Self {
        if commands.len() < 2 || commands.len() % 2 != 0 {
            panic!();
        }
        CommandWrite {
            addr: address,
            write: i2c::MasterSliceWrite::new(master, address.into(), &commands[0..2]),
            data: commands,
            index: 0+2,
        }
    }

    /// Polls this command write to completion
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(_) => {
                if self.index < self.data.len() {
                    let next = &self.data[self.index..(self.index+2)];
                    let addr: u8 = self.addr.into();
                    take_mut::take(&mut self.write, |w| {
                        i2c::MasterSliceWrite::new(w.finish(), addr, next)
                    });
                    self.index += 2;
                    Err(nb::Error::WouldBlock)
                }
                else {
                    Ok(())
                }
            },
        }
    }

    /// Gets the address of this command
    ///
    /// Ugh my API is so inconsistent...
    fn get_addr(&self) -> SSD1306Address {
        self.addr
    }

    /// Finishes or aborts this write
    fn finish(self) -> i2c::MasterI2c {
        self.write.finish()
    }
}

/// Writes buffer pages
///
/// A page is written to the device by sending 0x40 followed by the 16 bytes
/// comprising the page. In order to avoid a copy, this will operate directly
/// on the DISPLAY_BUFFER and prepend 0x40 as the MasterWrite requests the
/// first byte.
struct PageWrite {
    addr: SSD1306Address,
    write: i2c::MasterWrite,
    data: &'static [u8],
    index: Option<usize>,
}

impl PageWrite {
    /// Creates a new pagewrite. The passed page is the page number to write
    fn new(master: i2c::MasterI2c, address: SSD1306Address, page: usize) -> Self {
        // Since there is only one MasterI2c, this functions as a proxy for DISPLAY_BUFFER
        let index = page*16;
        PageWrite {
            addr: address,
            write: master.begin_write(address.into(), 17), //all page writes have 17 bytes
            data: unsafe { &DISPLAY_BUFFER[index..(index+16)] },
            index: None,
        }
    }

    /// Polls this page write to completion
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(r) => match r {
                i2c::MasterWriteResult::Complete => Ok(()),
                i2c::MasterWriteResult::Advance(t) => {
                    match self.index {
                        None => {
                            take_mut::take(&mut self.write, |w| {
                                t.advance(w, 0x40)
                            });
                            self.index = Some(0);
                        },
                        Some(i) => {
                            let byte = self.data[i];
                            take_mut::take(&mut self.write, |w| {
                                t.advance(w, byte)
                            });
                            self.index = Some(i + 1);
                        },
                    };
                    Err(nb::Error::WouldBlock)
                }
            },
        }
    }

    /// Gets the address of this command
    ///
    /// Ugh my API is so inconsistent...
    fn get_addr(&self) -> SSD1306Address {
        self.addr
    }

    /// Finishes or aborts this write
    fn finish(self) -> i2c::MasterI2c {
        self.write.finish()
    }
}

// Another note about implementation:
//
// Since the current HAL only has one MasterI2c, the presence of an
// SSD1306 struct can be used as a proxy for it. This also means that
// it is a singleton and its presence implies safety when accessing
// static mutables.
//
// TODO: Changed this to accept any reset pin, not just PB3

/// Uninitialized SSD1306
pub struct Uninitialized {
    addr: SSD1306Address,
    master: i2c::MasterI2c,
    reset: gpiob::PB3<Output<OpenDrainPullUp>>,
}

impl Uninitialized {
    /// Creates a new unitialized SSD1306
    ///
    /// This does not cooperate well with multiple devices sharing
    /// the I2C bus, but we'll get to that someday.
    pub fn new<Mode>(master: i2c::MasterI2c, address: SSD1306Address, reset: gpiob::PB3<Mode>, gpiob: &mut gpiob::Regs) -> Self {
        let configured_reset = reset.into_output_open_drain_pull_up(gpiob);
        Uninitialized { master: master, addr: address, reset: configured_reset }
    }

    /// Resets the SSD1306 in preparation for initialization
    pub fn reset(self) -> Resetting {
        Resetting::new(self)
    }
}

/// SSD1306 being reset
pub struct Resetting {
    addr: SSD1306Address,
    master: i2c::MasterI2c,
    reset: gpiob::PB3<Output<OpenDrainPullUp>>,
    start_time: u32,
}

impl Resetting {
    fn new(mut ui: Uninitialized) -> Self {
        ui.reset.set_low();
        Resetting {
            addr: ui.addr,
            master: ui.master,
            reset: ui.reset,
            start_time: systick::now(),
        }
    }

    /// Polling function for reset completion
    pub fn poll(&mut self) -> nb::Result<ResetToken, !> {
        match systick::wait_until(self.start_time + 1) {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(_)) => unreachable!(),
            Ok(_) => {
                self.reset.set_high();
                Ok(ResetToken::new())
            }
        }
    }
}

/// Token produced only when reset has completed
pub struct ResetToken {
    _0: (),
}

impl ResetToken {
    /// Creates a new reset token. Only use in this mod.
    fn new() -> Self {
        ResetToken { _0: () }
    }

    /// Following a reset, this begins the initialization process
    pub fn initialize(self, r: Resetting) -> Initializing {
        Initializing::new(r)
    }
}

/// SSD1306 being initialized
pub struct Initializing {
    write: CommandWrite,
}

impl Initializing {
    fn new(r: Resetting) -> Self {
        // Note that r's reset pin gets dropped
        Initializing {
            write: CommandWrite::new(r.master, r.addr, &INITIALIZATION_COMMANDS),
        }
    }
    /// Polling function for waiting for initialization to be complete
    pub fn poll(&mut self) -> nb::Result<InitializedToken, i2c::MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(_) => Ok(InitializedToken::new()),
        }
    }
}

/// Token produced only when initialization has completed
pub struct InitializedToken {
    _0: ()
}

impl InitializedToken {
    /// Creates a new initialized token. Only use in this mod.
    fn new() -> Self {
        InitializedToken { _0: () }
    }

    pub fn commit(self, i: Initializing) -> DisplayWrite {
        DisplayWrite::new_init(i)
    }
}

#[derive(Debug, Clone)]
pub enum DisplayError {
    OutOfRange,
}

static mut DISPLAY_BUFFER: [u8; 128*32 / 8] = [0; 128*32/8];

pub struct Display {
    addr: SSD1306Address,
    i2c: i2c::MasterI2c,
}

impl Display {
    fn new(dw: DisplayWrite) -> Self {
        Display { addr: dw.write.get_addr(), i2c: dw.write.finish() }
    }

    /// Clears the display buffer
    pub fn clear(&mut self) {
        // This is a proxy for DISPLAY_BUFFER
        unsafe { DISPLAY_BUFFER = [0; 128*32/8] };
    }

    /// Sets a single pixel in the buffer on or off
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        let shift_offset = y % 8;
        match (x, y) {
            (0...127, 0...31) => {
                let index = (y/8*128 + x) as usize;
                if on {
                    unsafe { DISPLAY_BUFFER[index] |= 0x1 << shift_offset };
                }
                else {
                    unsafe { DISPLAY_BUFFER[index] &= !(0x1 << shift_offset) };
                }
                Ok(())
            },
            _ => Err(DisplayError::OutOfRange),
        }
    }

    /// Quickly draws a horizontal line in the buffer
    pub fn hline(&mut self, start_x: usize, start_y: usize, end_x: usize) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        if start_x > end_x {
            return Err(DisplayError::OutOfRange);
        }
        match (start_x, start_y, end_x) {
            (0...127, 0...31, 0...127) => {
                let mask = 1 << (start_y % 8);
                for x in 0..(end_x - start_x) {
                    let index = start_y/8*128 + start_x + x;
                    unsafe { DISPLAY_BUFFER[index] |= mask };
                }
                Ok(())
            },
            _ => Err(DisplayError::OutOfRange),
        }
    }

    /// Quickly draws a vertical line in the buffer
    pub fn vline(&mut self, start_x: usize, start_y: usize, end_y: usize) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        if start_y > end_y {
            return Err(DisplayError::OutOfRange);
        }
        match (start_x, start_y, end_y) {
            (0...127, 0...31, 0...127) => {
                for y in 0..(end_y - start_y) {
                    let mask = 0xFF >> y % 8;
                    let index = (start_y+y)/8*128 + start_x;
                    unsafe { DISPLAY_BUFFER[index] |= mask };
                }
                Ok(())
            },
            _ => Err(DisplayError::OutOfRange),
        }
    }

    /// Begins a write to display the current buffer
    pub fn commit(self) -> DisplayWrite {
        DisplayWrite::new(self)
    }
}

/// Writes the start command sequence for a display write
struct DisplayWriteStart {
    write: CommandWrite,
}

impl DisplayWriteStart {
    /// Start a write of the display contents for the first time.
    fn new_init(i: Initializing) -> Self {
        let addr = i.write.get_addr();
        DisplayWriteStart {
            write: CommandWrite::new(i.write.finish(), addr, &DISPLAY_COMMANDS),
        }
    }

    /// Start a write of the display contents. This performs the command
    /// sequence for setting the initial page.
    fn new(d: Display) -> Self {
        DisplayWriteStart {
            write: CommandWrite::new(d.i2c, d.addr, &DISPLAY_COMMANDS),
        }
    }

    /// Polls this write to completion
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        self.write.poll()
    }

    /// Gets the address of our write
    fn get_addr(&self) -> SSD1306Address {
        self.write.get_addr()
    }

    /// Finishes or aborts this write
    fn finish(self) -> i2c::MasterI2c {
        self.write.finish()
    }
}

/// Writes all the buffer pages to the SSD1306
struct DisplayWritePages {
    write: PageWrite,
    page: usize,
}

impl DisplayWritePages {
    /// Begins writing the pages
    ///
    /// Note that this finishes the display write.
    fn new(s: DisplayWriteStart) -> Self {
        let addr = s.write.get_addr();
        DisplayWritePages {
            write: PageWrite::new(s.write.finish(), addr, 0),
            page: 0,
        }
    }

    /// Poll the page writing to completion
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(_) => {
                if self.page < unsafe { DISPLAY_BUFFER.len() / 16 - 1 } {
                    let page = self.page + 1;
                    take_mut::take(&mut self.write, |w| {
                        let addr = w.addr;
                        PageWrite::new(w.finish(), addr, page)
                    });
                    self.page = page;
                    Err(nb::Error::WouldBlock)
                }
                else {
                    Ok(())
                }
            }
        }
    }

    /// Gets the address of our write
    fn get_addr(&self) -> SSD1306Address {
        self.write.get_addr()
    }

    /// Finishes or aborts this write
    fn finish(self) -> i2c::MasterI2c {
        self.write.finish()
    }
}

/// State machine for a display write
enum DisplayWriteState {
    Start(DisplayWriteStart),
    Page(DisplayWritePages),
}

impl DisplayWriteState {
    fn get_addr(&self) -> SSD1306Address {
        match self {
            &DisplayWriteState::Start(ref s) => s.get_addr(),
            &DisplayWriteState::Page(ref p) => p.get_addr(),
        }
    }

    /// Change this state to the next state
    fn next(self) -> Self {
        match self {
            DisplayWriteState::Start(mut s) => match s.poll() {
                Ok(_) => DisplayWriteState::Page(DisplayWritePages::new(s)),
                _ => DisplayWriteState::Start(s),
            },
            DisplayWriteState::Page(p) => DisplayWriteState::Page(p),
        }
    }

    /// Polls the current state
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        // The actual movement of a state comes from next. This only polls again
        // and tells the caller whether or not this write can be finished.
        match self {
            &mut DisplayWriteState::Start(ref mut s) => match s.poll() {
                Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
                _ => Err(nb::Error::WouldBlock), // the start state always blocks because a page will follow
            },
            &mut DisplayWriteState::Page(ref mut p) => p.poll(),
        }
    }

    /// Finishes or aborts this display write
    fn finish(self) -> i2c::MasterI2c {
        match self {
            DisplayWriteState::Start(s) => s.finish(),
            DisplayWriteState::Page(p) => p.finish(),
        }
    }
}

/// Publicly facing display write
///
/// This encapsulates the series of steps required for a display write in a form
/// factor suitable for public consumption with a nonblocking API.
pub struct DisplayWrite {
    write: DisplayWriteState,
}

impl DisplayWrite {
    /// Initiates the first write of the display buffer to the SSD1306
    fn new_init(i: Initializing) -> Self {
        DisplayWrite { write: DisplayWriteState::Start(DisplayWriteStart::new_init(i)) }
    }
    /// Initiates a new write of the display buffer to the SSD1306
    fn new (d: Display) -> Self {
        DisplayWrite { write: DisplayWriteState::Start(DisplayWriteStart::new(d)) }
    }

    /// Polls this write to completion
    pub fn poll(&mut self) -> nb::Result<DisplayWriteCompleteToken, i2c::MasterI2cError> {
        take_mut::take(&mut self.write, |w| {
            w.next()
        });
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(_) => Ok(DisplayWriteCompleteToken::new()),
        }
    }
}

/// Token created when a display write is complete
pub struct DisplayWriteCompleteToken {
    _0: ()
}

impl DisplayWriteCompleteToken {
    /// Creates a new DisplayWriteCompleteToken
    fn new() -> Self {
        DisplayWriteCompleteToken { _0: () }
    }
    /// Finishes this write and produces a display interface.
    ///
    /// The display can be mutated as needed and then changed back into a
    /// DisplayWrite when it is ready to be committed.
    pub fn finish(self, dw: DisplayWrite) -> Display {
        Display::new(dw)
    }
}

