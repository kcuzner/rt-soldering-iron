/**
 * RT Soldering Iron
 *
 * SSD1306 Display Driver
 *
 * Kevin Cuzner
 */

#include "ssd1306.h"

#include "stm32f0xx.h"
#include "FreeRTOS.h"
#include "task.h"
#include "i2c.h"

#include <string.h>

static SSD1306Address m_address;
static uint8_t m_buffer[128 * 32 / 8];

typedef enum
{
    CMD_SetContrast = 0x81,
    CMD_DisplayAllOn = 0xa5,
    CMD_DisplayAllOn_Resume = 0xa4,
    CMD_NormalDisplay = 0xa6,
    CMD_InvertDisplay = 0xa7,
    CMD_DisplayOff = 0xae,
    CMD_DisplayOn = 0xaf,
    CMD_SetDisplayOffset = 0xd3,
    CMD_SetComPins = 0xda,
    CMD_SetVComDetect = 0xdb,
    CMD_SetDisplayClockDiv = 0xd5,
    CMD_SetPrecharge = 0xd9,
    CMD_SetMultiplex = 0xa8,
    CMD_SetLowColumn = 0x00,
    CMD_SetHighColumn = 0x10,
    CMD_SetStartLine = 0x40,
    CMD_MemoryMode = 0x20,
    CMD_ColumnAddr = 0x21,
    CMD_PageAddr = 0x22,
    CMD_ComScanInc = 0xc0,
    CMD_ComScanDec = 0xc8,
    CMD_SegRemap = 0xa0,
    CMD_ChargePump = 0x8d,
} SSD1306Command;

#define CMD_BYTES(cmd) 0x00, (uint8_t)(cmd)

static const uint8_t initialization[] = {
    CMD_BYTES(CMD_DisplayOff),
    CMD_BYTES(CMD_SetDisplayClockDiv),
    CMD_BYTES(0x80),
    CMD_BYTES(CMD_SetMultiplex),
    CMD_BYTES(31),
    CMD_BYTES(CMD_SetDisplayOffset),
    CMD_BYTES(0),
    CMD_BYTES((uint8_t)(CMD_SetStartLine) | 0x00),
    CMD_BYTES(CMD_ChargePump),
    CMD_BYTES(0x14), //we only use the charge pump
    CMD_BYTES(CMD_MemoryMode),
    CMD_BYTES(0),
    CMD_BYTES((uint8_t)(CMD_SegRemap) | 0x1),
    CMD_BYTES(CMD_ComScanDec),
    CMD_BYTES(CMD_SetComPins),
    CMD_BYTES(0x02), //128x32
    CMD_BYTES(CMD_SetContrast),
    CMD_BYTES(0x8F),
    CMD_BYTES(CMD_SetPrecharge),
    CMD_BYTES(0xF1),
    CMD_BYTES(CMD_SetVComDetect),
    CMD_BYTES(0x40),
    CMD_BYTES(CMD_DisplayAllOn_Resume), //change to the resume variant when done debugging
    CMD_BYTES(CMD_NormalDisplay),
    CMD_BYTES(CMD_DisplayOn)
};

static const uint8_t displayCommands[] = {
    CMD_BYTES(CMD_ColumnAddr),
    CMD_BYTES(0),
    CMD_BYTES(127),
    CMD_BYTES(CMD_PageAddr),
    CMD_BYTES(0),
    CMD_BYTES(3)
};

static bool ssd1306_writeCommands(const uint8_t *buffer, uint32_t length)
{
    for (uint32_t i = 0; i < length; i += 2)
    {
        if (!i2c_write((uint8_t)(m_address), buffer[i], &buffer[i+1], 1))
            return false;
    }
    return true;
}

bool ssd1306_init(SSD1306Address addr)
{
    m_address = addr;
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;

    //set up reset pin to be open drain, pulled up
    GPIOB->OTYPER |= GPIO_OTYPER_OT_3;
    GPIOB->PUPDR &= ~(GPIO_PUPDR_PUPDR3_1);
    GPIOB->PUPDR |= GPIO_PUPDR_PUPDR3_0;
    GPIOB->MODER &= ~(GPIO_MODER_MODER3_1);

    //strobe reset for a little bit
    GPIOB->BSRR = GPIO_BSRR_BR_3;
    vTaskDelay(10);
    GPIOB->BSRR = GPIO_BSRR_BS_3;

    //write initialization sequence
    if (!ssd1306_writeCommands(initialization, sizeof(initialization)))
        return false;
    return ssd1306_display();
}

TickType_t startDisp;
TickType_t endDisp;

bool ssd1306_display()
{
    uint8_t buffer[16];

    startDisp = xTaskGetTickCount();
    ssd1306_writeCommands(displayCommands, sizeof(displayCommands));

    for (uint32_t i = 0; i < sizeof(m_buffer); i += 16)
    {
        memcpy(buffer, &m_buffer[i], 16);
        if (!i2c_write((uint8_t)(m_address), 0x40, buffer, 16))
            return false;
    }
    endDisp = xTaskGetTickCount();

    return true;
}

void ssd1306_clear()
{
    memset(m_buffer, 0, sizeof(m_buffer));
}

void ssd1306_setPixel(uint8_t x, uint8_t y, uint8_t value)
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

void ssd1306_blit(uint8_t destX, uint8_t destY, uint8_t width, uint8_t height, const uint8_t *buffer)
{
    uint8_t bufferWidth = width / 8 + (width % 8 ? 1 : 0);

    for (uint8_t y = 0; y < height; y++)
    {
        uint16_t offset = y * bufferWidth;
        for (uint8_t x = 0; x < width; x++)
        {
            uint8_t mask = 0x1 << x % 8;
            uint8_t val = buffer[offset + x / 8] & mask ? 1 : 0;
            ssd1306_setPixel(destX + x, destY + y, val);
        }
    }
}

void ssd1306_hline(uint8_t startX, uint8_t startY, uint8_t endX)
{
    uint8_t mask = 1 << (startY % 8);
    for (uint8_t x = 0; x < endX - startX; x++)
    {
        m_buffer[(startY/8)*128 + startX + x] |= mask;
    }
}

void ssd1306_vline(uint8_t startX, uint8_t startY, uint8_t endY)
{
    for (uint8_t y = 0; y < endY - startY; y += 8)
    {
        uint8_t mask = 0xFF >> (y % 8);
        m_buffer[((startY + y)/8)*128 + startX] |= mask;
    }
}


