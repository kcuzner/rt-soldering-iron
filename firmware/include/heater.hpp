/**
 * RT Soldering Iron
 *
 * Heater Control
 */

#include "isr.hpp"

#include "FreeRTOS.h"
#include "semphr.h"

#include <cstdint>
#include <array>

class Heater : IRQHandler
{
public:
    const std::uint16_t PWMPeriod = 0x1FF;

    Heater();

    void setDutyCycle(std::uint16_t dc);
    std::uint16_t getTemperature();

    void isr();
    std::uint16_t getAvgAdcValue();
private:

    StaticSemaphore_t m_mutex;
    SemaphoreHandle_t m_mutexHandle;
    StaticSemaphore_t m_binaryISRWait;
    SemaphoreHandle_t m_binaryISRWaitHandle;
    std::array<std::uint16_t, 4> m_adcData;
};

