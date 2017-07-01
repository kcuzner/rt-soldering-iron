/**
 * RT Soldering Iron
 *
 * Heater Control
 *
 * Kevin Cuzner
 */

#include "heater.hpp"

#include "stm32f0xx.h"

Heater::Heater()
    : m_standby(true)
{
    RCC->AHBENR |= RCC_AHBENR_GPIOAEN | RCC_AHBENR_DMAEN;
    RCC->APB2ENR |= RCC_APB2ENR_ADCEN;
    RCC->APB1ENR |= RCC_APB1ENR_TIM14EN;

    //set up the heater PWM timer
    TIM14->PSC = 0;
    TIM14->ARR = SystemCoreClock / 1000;//Heater::PWMFreq;
    TIM14->CCR1 = TIM14->ARR / 32;
    TIM14->CCMR1 |= TIM_CCMR1_OC1M_2 | TIM_CCMR1_OC1M_1 |
        TIM_CCMR1_OC1PE;
    TIM14->CCER |= TIM_CCER_CC1E;
    TIM14->BDTR |= TIM_BDTR_MOE;
    TIM14->DIER |= TIM_DIER_UIE;

    //calibrate the ADC
    ADC1->CR |= ADC_CR_ADCAL;
    while (ADC1->CR & ADC_CR_ADCAL) { }

    //enable the ADC
    ADC1->CR |= ADC_CR_ADEN;
    while (!(ADC1->ISR & ADC_ISR_ADRDY)) { }
    ADC1->ISR |= ADC_ISR_ADRDY;

    //start continuous conversions
    ADC1->CFGR1 = ADC_CFGR1_CONT | ADC_CFGR1_OVRMOD | ADC_CFGR1_DMAEN | ADC_CFGR1_DMACFG; //circular DMA, allow overruns, continuous conversion
    ADC1->CFGR2 = ADC_CFGR2_CKMODE_0 | ADC_CFGR2_CKMODE_1; //clock / 4; this should be slow enough for any clock
    ADC1->CHSELR = ADC_CHSELR_CHSEL1;

    //configure the DMA
    DMA1_Channel1->CPAR = (uint32_t)(&(ADC1->DR));
    DMA1_Channel1->CMAR = (uint32_t)(m_adcData.data());
    DMA1_Channel1->CNDTR = m_adcData.size();
    DMA1_Channel1->CCR |= DMA_CCR_MINC | DMA_CCR_MSIZE_0 | DMA_CCR_PSIZE_0 | DMA_CCR_CIRC;
    DMA1_Channel1->CCR |= DMA_CCR_EN;

    //start conversions
    ADC1->CR |= ADC_CR_ADSTART;
    
    //heater sense is on PA1, heater control is on PA7
    GPIOA->AFR[0] &= 0x0FFFFFFF;
    GPIOA->AFR[0] |= 0x40000000;
    GPIOA->MODER &= ~GPIO_MODER_MODER7;
    GPIOA->MODER |= GPIO_MODER_MODER7_1 | GPIO_MODER_MODER1_0 | GPIO_MODER_MODER1_1;

    ISR::isr(TIM14_IRQn).attach(this);
    NVIC_EnableIRQ(TIM14_IRQn);
}

void Heater::setStandby(bool standby)
{
    if (standby)
    {
        TIM14->CR1 &= ~TIM_CR1_CEN;
    }
    else
    {
        TIM14->CR1 |= TIM_CR1_CEN;
    }
    m_standby = standby;
}

uint16_t Heater::getAvgAdcValue()
{
    uint32_t acc = 0;
    for (uint16_t i : m_adcData)
        acc += i;

    return acc / m_adcData.size();
}

void Heater::isr() //TIM14 ISR, as its period is slower than the ADC
{
    TIM14->SR = 0;
    if (m_standby)
        return;
}

