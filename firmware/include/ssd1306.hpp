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

    bool initialize();

private:
    I2C *m_i2c;
    Address m_address;
    std::array<std::uint8_t, 128 * 32 / 8> m_buffer;
};

