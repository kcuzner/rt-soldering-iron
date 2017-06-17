/**
 * RT Soldering Iron
 *
 * Buzzer component
 *
 * Kevin Cuzner
 */

#include "buzzer.hpp"
#include "isr.hpp"

#include "stm32f0xx.h"

using namespace std;

Buzzer::Buzzer()
    : m_countdown(0)
{
    RCC->AHBENR |= RCC_AHBENR_GPIOAEN;
    RCC->APB2ENR |= RCC_APB2ENR_TIM1EN;

    TIM1->PSC = 0;
    TIM1->CCMR1 |= TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_1 |
        TIM_CCMR1_OC1PE;
    TIM1->CCER |= TIM_CCER_CC1E;
    TIM1->BDTR |= TIM_BDTR_MOE;
    TIM1->DIER |= TIM_DIER_UIE;

    GPIOA->AFR[1] |= 0x00000002;
    GPIOA->MODER |= GPIO_MODER_MODER8_1;

    NVIC_EnableIRQ(TIM1_BRK_UP_TRG_COM_IRQn);
    ISR::isr(TIM1_BRK_UP_TRG_COM_IRQn).attach(this);
}

void Buzzer::beep(uint16_t duration_ms, uint16_t frequency_hz)
{
    TIM1->ARR = SystemCoreClock / frequency_hz;
    TIM1->CCR1 = TIM1->ARR / 2;

    m_countdown = (uint32_t)duration_ms * (uint32_t)frequency_hz / 1000;

    TIM1->CR1 |= TIM_CR1_CEN;
    TIM1->EGR |= TIM_EGR_UG;
}

void Buzzer::isr()
{
    if (TIM1->SR & TIM_SR_UIF)
    {
        if (!m_countdown)
        {
            TIM1->CR1 &= ~TIM_CR1_CEN;
        }
        else
        {
            m_countdown--;
        }
    }
    TIM1->SR = 0;
}

