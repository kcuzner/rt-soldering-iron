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

class ISR
{
public:
    ISR();
    void attach(IRQHandler *handler);
    void detach();
    void call();

    bool isValid();

    static ISR& isr(IRQn_Type irqn);

private:
    IRQHandler *m_handler;

    static std::array<ISR, 48> s_isrs;
    static ISR s_invalidIsr;
};

