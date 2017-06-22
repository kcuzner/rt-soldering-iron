/**
 * RT Soldering Iron
 *
 * ISR Handler
 *
 * Kevin Cuzner
 */

#pragma once

#include "stm32f0xx.h"

#include <array>

class IRQHandler
{
public:
    virtual void isr() = 0;
};

/**
 * IRQHandler for calling arbitrary functions in arbitrary types in response to an ISR
 */
template <class T>
class IRQHandlerFn : public IRQHandler
{
public:
    IRQHandlerFn(T *parent, void (T::*fn)(void))
        : m_parent(parent), m_fn(fn)
    {
    }

    void isr() { (m_parent->*m_fn)(); }
private:
    T *m_parent;
    void (T::*m_fn)(void);
};

class ISR
{
public:
    constexpr ISR();
    void attach(IRQHandler *handler);
    void detach();
    void call();

    constexpr bool isValid() { return this == &s_invalidIsr; }

    /**
     * Gets the ISR for a particular IRQ number
     */
    static constexpr ISR& isr(IRQn_Type irqn)
    {
        return (std::uint32_t)(irqn+16) > s_isrs.size() ? s_invalidIsr : s_isrs[(std::uint32_t)(irqn+16)];
    }

private:
    IRQHandler *m_handler;

    static std::array<ISR, 48> s_isrs;
    static ISR s_invalidIsr;
};

