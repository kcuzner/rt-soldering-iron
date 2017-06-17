/**
 * RT Soldering Iron
 *
 * ISR Handler
 *
 * Kevin Cuzner
 */

#include "isr.hpp"

using namespace std;

array<ISR, 48> ISR::s_isrs;
ISR ISR::s_invalidIsr;

ISR::ISR()
    : m_handler(nullptr)
{
}

void ISR::attach(IRQHandler *handler)
{
    m_handler = handler;
}

void ISR::detach()
{
    m_handler = nullptr;
}

void ISR::call()
{
    if (!m_handler)
    {
        //no ISR? Spin forever.
        while (1) { }
    }
    m_handler->isr();
}

bool ISR::isValid()
{
    return this == &s_invalidIsr;
}

ISR& ISR::isr(IRQn_Type irqn)
{
    uint8_t index = irqn + 16;
    if (index > s_isrs.size())
    {
        return s_invalidIsr;
    }

    return s_isrs[index];
}

extern "C"
{
    void NMI_Handler (void);
    void HardFault_Handler (void);
    void SVC_Handler (void);
    void PendSV_Handler (void);
    void SysTick_Handler (void);
    void WWDG_IRQHandler (void);                   /* Window WatchDog              */
    void PVD_IRQHandler (void);                    /* PVD through EXTI Line detect */
    void RTC_IRQHandler (void);                    /* RTC through the EXTI line    */
    void FLASH_IRQHandler (void);                  /* FLASH                        */
    void RCC_IRQHandler (void);                    /* RCC                          */
    void EXTI0_1_IRQHandler (void);                /* EXTI Line 0 and 1            */
    void EXTI2_3_IRQHandler (void);                /* EXTI Line 2 and 3            */
    void EXTI4_15_IRQHandler (void);               /* EXTI Line 4 to 15            */
    void DMA1_Channel1_IRQHandler (void);          /* DMA1 Channel 1               */
    void DMA1_Channel2_3_IRQHandler (void);        /* DMA1 Channel 2 and Channel 3 */
    void DMA1_Channel4_5_IRQHandler (void);        /* DMA1 Channel 4 and Channel 5 */
    void ADC1_IRQHandler (void);                   /* ADC1                         */
    void TIM1_BRK_UP_TRG_COM_IRQHandler (void);    /* TIM1 Break, Update, Trigger and Commutation */
    void TIM1_CC_IRQHandler (void);                /* TIM1 Capture Compare         */
    void TIM2_IRQHandler (void);                   /* TIM2                         */
    void TIM3_IRQHandler (void);                   /* TIM3                         */
    void TIM14_IRQHandler (void);                  /* TIM14                        */
    void TIM16_IRQHandler (void);                  /* TIM16                        */
    void TIM17_IRQHandler (void);                  /* TIM17                        */
    void I2C1_IRQHandler (void);                   /* I2C1                         */
    void SPI1_IRQHandler (void);                   /* SPI1                         */
    void USART1_IRQHandler (void);                 /* USART1                       */
};


void NMI_Handler (void)
{
    ISR::isr(NonMaskableInt_IRQn).call();
}

void HardFault_Handler (void)
{
    ISR::isr(HardFault_IRQn).call();
}

/* SVC, PendSV, and SysTick are implemented by FreeRTOS */

void WWDG_IRQHandler (void)                   /* Window WatchDog              */
{
    ISR::isr(WWDG_IRQn).call();
}

void RTC_IRQHandler (void)                    /* RTC through the EXTI line    */
{
    ISR::isr(RTC_IRQn).call();
}

void FLASH_IRQHandler (void)                  /* FLASH                        */
{
    ISR::isr(FLASH_IRQn).call();
}

void RCC_IRQHandler (void)                    /* RCC                          */
{
    ISR::isr(RCC_IRQn).call();
}

void EXTI0_1_IRQHandler (void)                /* EXTI Line 0 and 1            */
{
    ISR::isr(EXTI0_1_IRQn).call();
}

void EXTI2_3_IRQHandler (void)                /* EXTI Line 2 and 3            */
{
    ISR::isr(EXTI2_3_IRQn).call();
}

void EXTI4_15_IRQHandler (void)               /* EXTI Line 4 to 15            */
{
    ISR::isr(EXTI4_15_IRQn).call();
}

void DMA1_Channel1_IRQHandler (void)          /* DMA1 Channel 1               */
{
    ISR::isr(DMA1_Channel1_IRQn).call();
}

void DMA1_Channel2_3_IRQHandler (void)        /* DMA1 Channel 2 and Channel 3 */
{
    ISR::isr(DMA1_Channel2_3_IRQn).call();
}

void DMA1_Channel4_5_IRQHandler (void)        /* DMA1 Channel 4 and Channel 5 */
{
    ISR::isr(DMA1_Channel4_5_IRQn).call();
}

void ADC1_IRQHandler (void)                   /* ADC1                         */
{
    ISR::isr(ADC1_IRQn).call();
}

void TIM1_BRK_UP_TRG_COM_IRQHandler (void)    /* TIM1 Break, Update, Trigger and Commutation */
{
    ISR::isr(TIM1_BRK_UP_TRG_COM_IRQn).call();
}

void TIM1_CC_IRQHandler (void)                /* TIM1 Capture Compare         */
{
    ISR::isr(TIM1_CC_IRQn).call();
}

void TIM2_IRQHandler (void)                   /* TIM2                         */
{
    ISR::isr(TIM2_IRQn).call();
}

void TIM3_IRQHandler (void)                   /* TIM3                         */
{
    ISR::isr(TIM3_IRQn).call();
}

void TIM14_IRQHandler (void)                  /* TIM14                        */
{
    ISR::isr(TIM14_IRQn).call();
}

void TIM16_IRQHandler (void)                  /* TIM16                        */
{
    ISR::isr(TIM16_IRQn).call();
}

void TIM17_IRQHandler (void)                  /* TIM17                        */
{
    ISR::isr(TIM17_IRQn).call();
}

void I2C1_IRQHandler (void)                   /* I2C1                         */
{
    ISR::isr(I2C1_IRQn).call();
}

void SPI1_IRQHandler (void)                   /* SPI1                         */
{
    ISR::isr(SPI1_IRQn).call();
}

void USART1_IRQHandler (void)                 /* USART1                       */
{
    ISR::isr(USART1_IRQn).call();
}


