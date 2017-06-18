/**
 * RT Soldering Iron
 *
 * I2C
 *
 * Kevin Cuzner
 */

#pragma once

#include <cstdint>

#include "isr.hpp"
#include "FreeRTOS.h"
#include "semphr.h"

/**
 * I2C peripheral
 */
class I2C : public IRQHandler
{
public:
    /**
     * Initializes the I2C peripheral
     */
    I2C();

    /**
     * Writes bytes to an I2C device
     *
     * Do not use within an ISR
     *
     * address: I2C address, bits 7:1 are the slave address, 0 is don't care.
     * reg: Register address to write to
     * buffer: Pointer to a byte buffer to write from
     * len: Length of the buffer
     */
    bool write(std::uint8_t address, std::uint8_t reg, const std::uint8_t *buffer, std::uint8_t len);

    /**
     * Reads bytes from an I2C device
     *
     * Do not use within an ISR
     *
     * address: I2C address, bits 7:1 are the slave address, 0 is don't care
     * reg: Register address to read from
     * buffer: Pointer to a byte buffer to read from
     * len: Length of the buffer
     */
    bool read(std::uint8_t address, std::uint8_t reg, std::uint8_t *buffer, std::uint8_t len);

    /**
     * ISR Handler
     */
    void isr();

private:
    void setTiming();
    bool waitForMask(std::uint16_t mask);

    StaticSemaphore_t m_mutex;
    SemaphoreHandle_t m_mutexHandle;
    StaticSemaphore_t m_binaryISRWait;
    SemaphoreHandle_t m_binaryISRWaitHandle;
};

