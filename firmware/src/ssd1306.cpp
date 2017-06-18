/**
 * RT Soldering Iron
 *
 * SSD1306 Display Driver
 *
 * Kevin Cuzner
 */

#include "ssd1306.hpp"

#include "stm32f0xx.h"
#include "FreeRTOS.h"
#include "task.h"

#include <cstdint>

using namespace std;

enum class SSD1306Command : uint8_t
{
    SetContrast = 0x81,
    DisplayAllOn = 0xa5,
    DisplayAllOn_Resume = 0xa4,
    NormalDisplay = 0xa6,
    InvertDisplay = 0xa7,
    DisplayOff = 0xae,
    DisplayOn = 0xaf,
    SetDisplayOffset = 0xd3,
    SetComPins = 0xda,
    SetVComDetect = 0xdb,
    SetDisplayClockDiv = 0xd5,
    SetPrecharge = 0xd9,
    SetMultiplex = 0xa8,
    SetLowColumn = 0x00,
    SetHighColumn = 0x10,
    SetStartLine = 0x40,
    MemoryMode = 0x20,
    ColumnAddr = 0x21,
    PageAddr = 0x22,
    ComScanInc = 0xc0,
    ComScanDec = 0xc8,
    SegRemap = 0xa0,
    ChargePump = 0x8d,
};

#define CMD_BYTES(cmd) 0x00, static_cast<uint8_t>(cmd)

static const uint8_t initialization[] = {
    CMD_BYTES(SSD1306Command::DisplayOff),
    CMD_BYTES(SSD1306Command::SetDisplayClockDiv),
    CMD_BYTES(0x80),
    CMD_BYTES(SSD1306Command::SetMultiplex),
    CMD_BYTES(31),
    CMD_BYTES(SSD1306Command::SetDisplayOffset),
    CMD_BYTES(0),
    CMD_BYTES(static_cast<uint8_t>(SSD1306Command::SetStartLine) | 0x00),
    CMD_BYTES(SSD1306Command::ChargePump),
    CMD_BYTES(0x14), //we only use the charge pump
    CMD_BYTES(SSD1306Command::MemoryMode),
    CMD_BYTES(0),
    CMD_BYTES(static_cast<uint8_t>(SSD1306Command::SegRemap) | 0x1),
    CMD_BYTES(SSD1306Command::ComScanDec),
    CMD_BYTES(SSD1306Command::SetComPins),
    CMD_BYTES(0x02), //128x32
    CMD_BYTES(SSD1306Command::SetContrast),
    CMD_BYTES(0x8F),
    CMD_BYTES(SSD1306Command::SetPrecharge),
    CMD_BYTES(0xF1),
    CMD_BYTES(SSD1306Command::SetVComDetect),
    CMD_BYTES(0x40),
    CMD_BYTES(SSD1306Command::DisplayAllOn), //change to the resume variant when done debugging
    CMD_BYTES(SSD1306Command::NormalDisplay),
    CMD_BYTES(SSD1306Command::DisplayOn)
};

SSD1306::SSD1306(I2C *i2c, const Address &addr)
    : m_i2c(i2c), m_address(addr)
{
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;

    //set up reset pin to be open drain, pulled up
    GPIOB->OTYPER |= GPIO_OTYPER_OT_3;
    GPIOB->PUPDR &= ~(GPIO_PUPDR_PUPDR3_1);
    GPIOB->PUPDR |= GPIO_PUPDR_PUPDR3_0;
    GPIOB->MODER &= ~(GPIO_MODER_MODER3_1);
    GPIOB->MODER |= GPIO_MODER_MODER3_0;
}

bool SSD1306::initialize()
{
    //strobe reset for a little bit
    GPIOB->BSRR = GPIO_BSRR_BR_3;
    vTaskDelay(10);
    GPIOB->BSRR = GPIO_BSRR_BS_3;

    //write initialization sequence
    for (uint8_t i = 0; i < sizeof(initialization); i+= 2)
    {
        if (!m_i2c->write(static_cast<uint8_t>(m_address), initialization[i], &initialization[i+1], 1))
            return false;
    }
    return true;
}

