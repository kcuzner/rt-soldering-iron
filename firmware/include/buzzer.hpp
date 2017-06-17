/**
 * RT Soldering Iron
 *
 * Buzzer component
 *
 * Kevin Cuzner
 */

#ifndef _BUZZER_H_
#define _BUZZER_H_

#include <cstdint>
#include "FreeRTOS.h"
#include "semphr.h"

#include "isr.hpp"

class Buzzer : public IRQHandler
{
public:
    Buzzer();

    void beep(std::uint16_t duration_ms, std::uint16_t frequency_hz);

    void isr();
private:
    std::uint32_t m_countdown;
};

#endif //_BUZZER_H_

