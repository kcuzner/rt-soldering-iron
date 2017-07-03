/**
 * RT Soldering Iron
 *
 * SSD1306 Display Driver
 *
 * Kevin Cuzner
 */

#pragma once

#include "i2c.hpp"
#include <array>
#include <cstdint>

/**
 * SSD1306 driver specific to the 128x32 display being used on this project
 */
class SSD1306
{
public:
    enum class Address : std::uint8_t { LOW = 0x78, HIGH = 0x7a };
    SSD1306(I2C *i2c, const Address &addr);

    /**
     * Initializes the attached display
     */
    bool initialize();

    /**
     * Sends the current buffer to the display
     */
    bool display();

    /**
     * Clears the buffer
     */
    void clear();
    /**
     * Sets a single pixel in the buffer
     */
    void setPixel(std::uint8_t x, std::uint8_t y, std::uint8_t value);
    /**
     * Blits a bitmap to the buffer. The bitmap should be arranged with the bytes
     * as rows.
     */
    void blit(std::uint8_t destX, std::uint8_t destY, std::uint8_t width, std::uint8_t height, const std::uint8_t *buffer);

private:
    bool writeCommands(const std::uint8_t *buffer, std::uint32_t length);

    I2C *m_i2c;
    Address m_address;
    std::array<std::uint8_t, 128 * 32 / 8> m_buffer;
};

