/**
 * RT Soldering Iron
 *
 * Buzzer component
 *
 * Kevin Cuzner
 */

#include "buzzer.h"
#include "autolock.h"

#include "stm32f0xx.h"

static uint32_t m_countdown = 0;

static StaticSemaphore_t m_mutex;
static SemaphoreHandle_t m_mutexHandle;

void buzzer_init()
{
    m_mutexHandle = xSemaphoreCreateMutexStatic(&m_mutex);

    RCC->AHBENR |= RCC_AHBENR_GPIOAEN;
    RCC->APB2ENR |= RCC_APB2ENR_TIM1EN;

    TIM1->PSC = 0;
    TIM1->CCMR1 |= TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_1 |
        TIM_CCMR1_OC1PE;
    TIM1->CCER |= TIM_CCER_CC1E;
    TIM1->BDTR |= TIM_BDTR_MOE;
    TIM1->DIER |= TIM_DIER_UIE;

    GPIOA->AFR[1] &= 0xFFFFFFF0;
    GPIOA->AFR[1] |= 0x00000002;
    GPIOA->MODER &= ~GPIO_MODER_MODER8;
    GPIOA->MODER |= GPIO_MODER_MODER8_1;

    NVIC_EnableIRQ(TIM1_BRK_UP_TRG_COM_IRQn);
}

void buzzer_beep(uint16_t duration_ms, uint16_t frequency_hz)
{
    AUTOLOCK_TAKE(m_mutexHandle)
    {
        TIM1->ARR = SystemCoreClock / frequency_hz;
        TIM1->CCR1 = TIM1->ARR / 2;

        m_countdown = (uint32_t)duration_ms * (uint32_t)frequency_hz / 1000;

        TIM1->CR1 |= TIM_CR1_CEN;
        TIM1->EGR |= TIM_EGR_UG;
    }
}

void TIM1_BRK_UP_TRG_COM_IRQHandler()
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
