/**
 * RT Soldering Iron
 *
 * Heater Control
 */

#include "isr.hpp"

#include <cstdint>
#include <array>

class Heater : IRQHandler
{
public:
    const uint32_t PWMFreq = 1E3;

    enum class Status { STANDBY, RAMP, STEADY };

    Heater();

    Status getStatus();
    std::uint16_t getTemperature();
    void setStandby(bool standby);
    void setTemperature(std::uint16_t temperature);

    void isr();
    std::uint16_t getAvgAdcValue();
private:

    bool m_standby;
    std::uint16_t m_rawCommand;
    std::array<std::uint16_t, 4> m_adcData;
};

