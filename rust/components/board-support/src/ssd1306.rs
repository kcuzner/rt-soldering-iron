//! SSD1306 Module

use core::convert::Into;

use stm32f031x_hal::i2c;

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

#[derive(Copy, Clone)]
enum Ssd1306Command {
    SetContrast,
    DisplayAllOn,
    DisplayAllOnResume,
    NormalDisplay,
    InvertDisplay,
    DisplayOff,
    DisplayOn,
    SetDisplayOffset,
    SetComPins,
    SetVComDetect,
    SetDisplayClockDiv,
    SetPrecharge,
    SetMultiplex,
    SetLowColumn,
    SetHighColumn,
    SetStartLine(u8),
    MemoryMode,
    ColumnAddr,
    PageAddr,
    ComScanInc,
    ComScanDec,
    SegRemap(bool),
    ChargePump,
}

impl Into<[u8; 2]> for Ssd1306Command {
    fn into(self) -> [u8; 2] {
        match self {
            Ssd1306Command::SetContrast => [0, 0x81],
            Ssd1306Command::DisplayAllOn => [0, 0xa5],
            Ssd1306Command::DisplayAllOnResume => [0, 0xa4],
            Ssd1306Command::NormalDisplay => [0, 0xa6],
            Ssd1306Command::InvertDisplay => [0, 0xa7],
            Ssd1306Command::DisplayOff => [0, 0xae],
            Ssd1306Command::DisplayOn => [0, 0xaf],
            Ssd1306Command::SetDisplayOffset => [0, 0xd3],
            Ssd1306Command::SetComPins => [0, 0xda],
            Ssd1306Command::SetVComDetect => [0, 0xdb],
            Ssd1306Command::SetDisplayClockDiv => [0, 0xd5],
            Ssd1306Command::SetPrecharge => [0, 0xd9],
            Ssd1306Command::SetMultiplex => [0, 0xa8],
            Ssd1306Command::SetLowColumn => [0, 0x00],
            Ssd1306Command::SetHighColumn => [0, 0x10],
            Ssd1306Command::SetStartLine(line) => [0, 0x40 | line],
            Ssd1306Command::MemoryMode => [0, 0x20],
            Ssd1306Command::ColumnAddr => [0, 0x21],
            Ssd1306Command::PageAddr => [0, 0x22],
            Ssd1306Command::ComScanInc => [0, 0xc0],
            Ssd1306Command::ComScanDec => [0, 0xc8],
            Ssd1306Command::SegRemap(false) => [0, 0xa0],
            Ssd1306Command::SegRemap(true) => [0, 0xa1],
            Ssd1306Command::ChargePump => [0, 0x8d],
        }
    }
}

const INITIALIZATION: [u8; 50] = [
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

