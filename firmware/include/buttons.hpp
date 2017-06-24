/**
 * RT Soldering Iron
 *
 * Buttons
 *
 * Kevin Cuzner
 */

#pragma once

#include <cstdint>
#include <array>

#include "isr.hpp"
#include "FreeRTOS.h"
#include "queue.h"
#include "semphr.h"

class Buttons : public IRQHandler
{
public:
    enum class Button : std::uint8_t { NONE = 0, UP = 1 << 0, LEFT = 1 << 1, DOWN = 1 << 2, RIGHT = 1 << 3, CENTER = 1 << 4 };
    Buttons();

    Button getNextPress(const TickType_t &timeout = portMAX_DELAY);
    Button getCurrentState();

    void isr();

private:
    void extiUpperIsr();
    void extiLowerIsr();
    void handleExti(BaseType_t *pxHigherPriorityTaskWoken);

    StaticSemaphore_t m_semaphore;
    SemaphoreHandle_t m_semaphoreHandle;
    StaticQueue_t m_queue;
    QueueHandle_t m_queueHandle;
    std::uint8_t m_buttonStorage[sizeof(Button)];
};

inline Buttons::Button operator | (Buttons::Button lh, Buttons::Button rh)
{
    using T = std::underlying_type<Buttons::Button>::type;
    return static_cast<Buttons::Button>(static_cast<T>(lh) | static_cast<T>(rh));
}

inline Buttons::Button operator & (Buttons::Button lh, Buttons::Button rh)
{
    using T = std::underlying_type<Buttons::Button>::type;
    return static_cast<Buttons::Button>(static_cast<T>(lh) & static_cast<T>(rh));
}

inline Buttons::Button& operator |= (Buttons::Button& lh, Buttons::Button rh)
{
    lh = lh | rh;
    return lh;
}

inline Buttons::Button& operator &= (Buttons::Button& lh, Buttons::Button rh)
{
    lh = lh & rh;
    return lh;
}

