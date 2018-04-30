//! SSD1306 Module

use core::convert::From;

use nb;
use bare_take_mut as take_mut;
use stm32f031x_hal::i2c;

// A word about implementation:
//
// I started out wanting to make myself some enums for the commands
// where I could ensure that the right arguments only were sent and
// that they were properly formatted. However, I realized that this
// would take up far more code space than some blobs. Therefore,
// the command sequences for initializing the display and 
// transferring a page to be displayed are represented as binary blobs.

const CMD_SET_CONTRAST: u8 = 0x81;
const CMD_DISPLAY_ALL_ON: u8 = 0xa5;
const CMD_DISPLAY_ALL_ON_RESUME: u8 = 0xa4;
const CMD_NORMAL_DISPLAY: u8 = 0xa6;
const CMD_INVERT_DISPLAY: u8 = 0xa7;
const CMD_DISPLAY_OFF: u8 = 0xae;
const CMD_DISPLAY_ON: u8 = 0xaf;
const CMD_SET_DISPLAY_OFFSET: u8 = 0xd3;
const CMD_SET_COM_PINS: u8 = 0xda;
const CMD_SET_V_COM_DETECT: u8 = 0xdb;
const CMD_SET_DISPLAY_CLOCK_DIV: u8 = 0xd5;
const CMD_SET_PRECHARGE: u8 = 0xd9;
const CMD_SET_MULTIPLEX: u8 = 0xa8;
const CMD_SET_LOW_COLUMN: u8 = 0x00;
const CMD_SET_HIGH_COLUMN: u8 = 0x10;
const CMD_SET_START_LINE: u8 = 0x40;
const CMD_MEMORY_MODE: u8 = 0x20;
const CMD_COLUMN_ADDR: u8 = 0x21;
const CMD_PAGE_ADDR: u8 = 0x22;
const CMD_COM_SCAN_INC: u8 = 0xc0;
const CMD_COM_SCAN_DEC: u8 = 0xc8;
const CMD_SEG_REMAP: u8 = 0xa0;
const CMD_CHARGE_PUMP: u8 = 0x8d;

/// Command sequence for initializing the SSD1306
const INITIALIZATION_COMMANDS: [u8; 50] = [
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

const DISPLAY_COMMANDS: [u8; 12] = [
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
            }
        }
    }

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

/// Uninitialized SSD1306
pub struct Uninitialized {
    addr: SSD1306Address,
    master: i2c::MasterI2c,
}

impl Uninitialized {
    /// Creates a new unitialized SSD1306
    ///
    /// This does not cooperate well with multiple devices sharing
    /// the I2C bus, but we'll get to that someday.
    pub fn new(master: i2c::MasterI2c, address: SSD1306Address) -> Self {
        Uninitialized { master: master, addr: address }
    }

    /// Kicks off the initialization process for the SSD1306
    pub fn initialize(self) -> Initializing {
        Initializing::new(self)
    }
}

pub struct Initializing {
    write: CommandWrite,
}

impl Initializing {
    fn new(ui: Uninitialized) -> Self {
        let addr = ui.addr.clone();
        Initializing {
            write: CommandWrite::new(ui.master, ui.addr, &INITIALIZATION_COMMANDS),
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

    pub fn finish(self, i: Initializing) -> Display {
        Display::new(i)
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
    fn new(i: Initializing) -> Self {
        Display { addr: i.write.get_addr(), i2c: i.write.finish() }
    }

    /// Clears the display buffer
    pub fn clear(&mut self) {
        // This is a proxy for DISPLAY_BUFFER
        unsafe { DISPLAY_BUFFER = [0; 128*32/8] };
    }

    /// Sets a single pixel in the buffer on or off
    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        let shiftOffset = y % 8;
        match (x, y) {
            (0...127, 0...31) => {
                let index = (y/8*128 + x) as usize;
                if on {
                    unsafe { DISPLAY_BUFFER[index] |= 0x1 << shiftOffset };
                }
                else {
                    unsafe { DISPLAY_BUFFER[index] &= !(0x1 << shiftOffset) };
                }
                Ok(())
            },
            _ => Err(DisplayError::OutOfRange),
        }
    }

    /// Quickly draws a horizontal line in the buffer
    pub fn hline(&mut self, start_x: u8, start_y: u8, end_x: u8) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        if start_x > end_x {
            return Err(DisplayError::OutOfRange);
        }
        match (start_x, start_y, end_x) {
            (0...127, 0...31, 0...127) => {
                let mask = 1 << (start_y % 8);
                for x in 0..(end_x - start_x) {
                    let index = (start_y/8*128 + start_x + x) as usize;
                    unsafe { DISPLAY_BUFFER[index] |= mask };
                }
                Ok(())
            },
            _ => Err(DisplayError::OutOfRange),
        }
    }

    /// Quickly draws a vertical line in the buffer
    pub fn vline(&mut self, start_x: u8, start_y: u8, end_y: u8) -> Result<(), DisplayError> {
        // This is a proxy for DISPLAY_BUFFER, so DISPLAY_BUFFER is safe
        if start_y > end_y {
            return Err(DisplayError::OutOfRange);
        }
        match (start_x, start_y, end_y) {
            (0...127, 0...31, 0...127) => {
                for y in 0..(end_y - start_y) {
                    let mask = 0xFF >> y % 8;
                    let index = ((start_y+y)/8*128 + start_x) as usize;
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

struct DisplayWriteStart {
    write: CommandWrite,
}

impl DisplayWriteStart {
    /// Start a write of the display contents. This performs the command
    /// sequence for setting the initial page.
    fn new(d: Display) -> Self {
        DisplayWriteStart {
            write: CommandWrite::new(d.i2c, d.addr, &DISPLAY_COMMANDS),
        }
    }

    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(_) => Ok(()),
        }
    }
}

struct DisplayWritePage {
    addr: SSD1306Address,
    write: i2c::MasterSliceWrite,
    page: usize,
}

impl DisplayWritePage {
    fn new(s: DisplayWriteStart) -> Self {
        // NOTE: This proxies DISPLAY_BUFFER and so the access is safe
        let addr = s.write.get_addr();
        DisplayWritePage {
            addr: addr,
            //NOTE: This needs to be fixed: It needs to precede the page with 0x40
            write: i2c::MasterSliceWrite::new(s.write.finish(), addr.into(), unsafe { &DISPLAY_BUFFER[0..16] }),
            page: 0,
        }
    }

    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        Err(nb::Error::WouldBlock)
    }
}

enum DisplayWriteState {
    Start(DisplayWriteStart),
    Page(DisplayWritePage),
}

impl DisplayWriteState {
    fn new(d: Display) -> Self {
        DisplayWriteState::Start(DisplayWriteStart::new(d))
    }

    fn next(self) -> Self {
        self
    }

    /// Polls
    fn poll(&mut self) -> nb::Result<(), i2c::MasterI2cError> {
        match self {
            &mut DisplayWriteState::Start(_) => Err(nb::Error::WouldBlock),
            &mut DisplayWriteState::Page(ref mut p) => p.poll(),
        }
    }
}

pub struct DisplayWrite {
    state: DisplayWriteState,
}

impl DisplayWrite {
    fn new (d: Display) -> Self {
        DisplayWrite { state: DisplayWriteState::Start(DisplayWriteStart::new(d)) }
    }
}

