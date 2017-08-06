/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 */

#include "buttons.h"

#include "stm32f0xx.h"
#include "task.h"

#include "autolock.h"

static StaticSemaphore_t m_semaphore;
static SemaphoreHandle_t m_semaphoreHandle;
static StaticQueue_t m_queue;
static QueueHandle_t m_queueHandle;
static uint8_t m_buttonStorage[sizeof(ButtonPress)];

void buttons_init()
{
    m_queueHandle = xQueueCreateStatic(1, sizeof(ButtonPress), m_buttonStorage, &m_queue);
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

    NVIC_EnableIRQ(EXTI2_3_IRQn);
    NVIC_EnableIRQ(EXTI4_15_IRQn);
}

ButtonPress buttons_getNextPress(const TickType_t timeout)
{
    ButtonPress btn = BTN_NONE;
    AUTOLOCK_TAKE(m_semaphoreHandle)
    {
        while (btn == BTN_NONE)
        {
            EXTI->PR |= EXTI_PR_PIF2 | EXTI_PR_PIF3 | EXTI_PR_PIF4 | EXTI_PR_PIF5 | EXTI_PR_PIF6;
            EXTI->FTSR |= EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6;
            if (!xQueueReceive(m_queueHandle, &btn, timeout))
            {
                EXTI->FTSR &= ~(EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6);
                return BTN_NONE;
            }
            vTaskDelay(100); //debounce delay
            uint32_t val = (GPIOA->IDR >> 2) & 0x1F;
            btn &= ~val;
        }
    }
    return btn;
}

ButtonPress buttons_getCurrentState()
{
    uint32_t val = (GPIOA->IDR >> 2) & 0x1F;
    return (~val) & 0x1F;
}

void __attribute__((alias("EXTI2_3_IRQHandler"))) EXTI4_15_IRQHandler();

void EXTI2_3_IRQHandler()
{
    BaseType_t xHigherPriorityTaskWoken;

    uint32_t val = ~(GPIOA->IDR >> 2);
    ButtonPress btn = val & 0x1F;
    xQueueOverwriteFromISR(m_queueHandle, &btn, &xHigherPriorityTaskWoken);

    EXTI->PR |= EXTI_PR_PIF2 | EXTI_PR_PIF3 | EXTI_PR_PIF4 | EXTI_PR_PIF5 | EXTI_PR_PIF6;
    EXTI->FTSR &= ~(EXTI_FTSR_FT2 | EXTI_FTSR_FT3 | EXTI_FTSR_FT4 | EXTI_FTSR_FT5 | EXTI_FTSR_FT6);
    portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

