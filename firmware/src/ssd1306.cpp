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
#include <cstring>
#include <algorithm>

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
    CMD_BYTES(SSD1306Command::DisplayAllOn_Resume), //change to the resume variant when done debugging
    CMD_BYTES(SSD1306Command::NormalDisplay),
    CMD_BYTES(SSD1306Command::DisplayOn)
};

static const uint8_t displayCommands[] = {
    CMD_BYTES(SSD1306Command::ColumnAddr),
    CMD_BYTES(0),
    CMD_BYTES(127),
    CMD_BYTES(SSD1306Command::PageAddr),
    CMD_BYTES(0),
    CMD_BYTES(3)
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
    if (!writeCommands(initialization, sizeof(initialization)))
        return false;
    return display();
}

TickType_t startDisp;
TickType_t endDisp;

bool SSD1306::display()
{
    uint8_t buffer[16];

    startDisp = xTaskGetTickCount();
    writeCommands(displayCommands, sizeof(displayCommands));

    for (uint32_t i = 0; i < m_buffer.size(); i += 16)
    {
        memcpy(buffer, &m_buffer.data()[i], 16);
        if (!m_i2c->write(static_cast<uint8_t>(m_address), 0x40, buffer, 16))
            return false;
    }
    endDisp = xTaskGetTickCount();

    return true;
}

void SSD1306::clear()
{
    memset(m_buffer.data(), 0, m_buffer.size());
}

void SSD1306::setPixel(uint8_t x, uint8_t y, uint8_t value)
{
    uint8_t shiftOffset = y % 8;
    if (x > 127)
        return;
    if (y > 31)
        return;
    if (value)
    {
        m_buffer[(y/8)*128 + x] |= 0x1 << shiftOffset;
    }
    else
    {
        m_buffer[(y/8)*128 + x] &= ~(0x1 << shiftOffset);
    }
}

void SSD1306::blit(uint8_t destX, uint8_t destY, uint8_t width, uint8_t height, const uint8_t *buffer)
{
    uint8_t bufferWidth = width / 8 + (width % 8 ? 1 : 0);

    for (uint8_t y = 0; y < height; y++)
    {
        uint16_t offset = y * bufferWidth;
        for (uint8_t x = 0; x < width; x++)
        {
            uint8_t mask = 0x1 << x % 8;
            uint8_t val = buffer[offset + x / 8] & mask ? 1 : 0;
            setPixel(destX + x, destY + y, val);
        }
    }
}

bool SSD1306::writeCommands(const uint8_t *buffer, uint32_t length)
{
    for (uint32_t i = 0; i < length; i += 2)
    {
        if (!m_i2c->write(static_cast<uint8_t>(m_address), buffer[i], &buffer[i+1], 1))
            return false;
    }
    return true;
}

