/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 */

#include "buttons.hpp"
#include "autolock.hpp"

#include "stm32f0xx.h"
#include "task.h"

Buttons::Buttons()
{
    m_queueHandle = xQueueCreateStatic(1, sizeof(Buttons::Button), m_buttonStorage, &m_queue);
    m_semaphoreHandle = xSemaphoreCreateMutexStatic(&m_semaphore);

    RCC->AHBENR |= RCC_AHBENR_GPIOAEN;
    RCC->APB2ENR |= RCC_APB2ENR_SYSCFGCOMPEN;

    GPIOA->MODER &= ~(GPIO_MODER_MODER2 | GPIO_MODER_MODER3 | GPIO_MODER_MODER4 | GPIO_MODER_MODER5 | GPIO_MODER_MODER6);
    GPIOA->PUPDR &= ~(GPIO_PUPDR_PUPDR2 | GPIO_PUPDR_PUPDR3 | GPIO_PUPDR_PUPDR4 | GPIO_PUPDR_PUPDR5 | GPIO_PUPDR_PUPDR6);
    GPIOA->PUPDR |= GPIO_PUPDR_PUPDR2_0 | GPIO_PUPDR_PUPDR3_0 | GPIO_PUPDR_PUPDR4_0 | GPIO_PUPDR_PUPDR5_0 | GPIO_PUPDR_PUPDR6_0;

    SYSCFG->EXTICR[0] &= ~(SYSCFG_EXTICR1_EXTI3 | SYSCFG_EXTICR1_EXTI2);
    SYSCFG->EXTICR[0] |= SYSCFG_EXTICR1_EXTI3_PA | SYSCFG_EXTICR1_EXTI3_PA;
    SYSCFG->EXTICR[1] &= ~(SYSCFG_EXTICR2_EXTI4 | SYSCFG_EXTICR2_EXTI5 | SYSCFG_EXTICR2_EXTI6);
    SYSCFG->EXTICR[1] |= SYSCFG_EXTICR2_EXTI4_PA | SYSCFG_EXTICR2_EXTI5_PA | SYSCFG_EXTICR2_EXTI6_PA;

    EXTI->IMR |= EXTI_IMR_IM2 | EXTI_IMR_IM3 | EXTI_IMR_IM4 | EXTI_IMR_IM5 | EXTI_IMR_IM6;

    ISR::isr(EXTI2_3_IRQn).attach(this);
    ISR::isr(EXTI4_15_IRQn).attach(this);

    NVIC_EnableIRQ(EXTI2_3_IRQn);
    NVIC_EnableIRQ(EXTI4_15_IRQn);
}

Buttons::Button Buttons::getNextPress(const TickType_t &timeout)
{
    Autolock l_lock(m_semaphoreHandle);

    Buttons::Button btn = Buttons::Button::NONE;

    while (btn == Buttons::Button::NONE)
    {
        EXTI->PR |= EXTI_PR_PIF2 | EXTI_PR_PIF3 | EXTI_PR_PIF4 | EXTI_PR_PIF5 | EXTI_PR_PIF6;
        EXTI->FTSR |= EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6;
        if (!xQueueReceive(m_queueHandle, &btn, timeout))
        {
            EXTI->FTSR &= ~(EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6);
            return Buttons::Button::NONE;
        }
        vTaskDelay(100); //debounce delay
        uint32_t val = (GPIOA->IDR >> 2) & 0x1F;
        btn &= static_cast<Buttons::Button>(~val);
    }

    return btn;
}

Buttons::Button Buttons::getCurrentState()
{
    uint32_t val = (GPIOA->IDR >> 2) & 0x1F;
    return static_cast<Buttons::Button>((~val) & 0x1F);
}

void Buttons::isr()
{
    BaseType_t xHigherPriorityTaskWoken;

    uint32_t val = ~(GPIOA->IDR >> 2);
    val &= 0x1F;
    Buttons::Button btn = static_cast<Buttons::Button>(val);
    xQueueOverwriteFromISR(m_queueHandle, &btn, &xHigherPriorityTaskWoken);

    EXTI->PR |= EXTI_PR_PIF2 | EXTI_PR_PIF3 | EXTI_PR_PIF4 | EXTI_PR_PIF5 | EXTI_PR_PIF6;
    EXTI->FTSR &= ~(EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6);
    portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

