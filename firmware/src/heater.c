/**
 * RT Soldering Iron
 *
 * Heater Control
 *
 * Kevin Cuzner
 */

#include "heater.h"

#include "stm32f0xx.h"
#include "autolock.h"

static StaticSemaphore_t m_mutex;
static SemaphoreHandle_t m_mutexHandle;
static StaticSemaphore_t m_binaryISRWait;
static SemaphoreHandle_t m_binaryISRWaitHandle;
static uint16_t m_adcData[4];

void heater_init()
{
    RCC->AHBENR |= RCC_AHBENR_GPIOAEN | RCC_AHBENR_DMAEN;
    RCC->APB2ENR |= RCC_APB2ENR_ADCEN;
    RCC->APB1ENR |= RCC_APB1ENR_TIM14EN;

    //set up the mutexes
    m_mutexHandle = xSemaphoreCreateMutexStatic(&m_mutex);
    m_binaryISRWaitHandle = xSemaphoreCreateBinaryStatic(&m_binaryISRWait);

    //set up the heater PWM timer
    TIM14->PSC = 0;
    TIM14->ARR = HEATER_PWMPERIOD;
    TIM14->CCR1 = 0;
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

    //configure the DMA to do 4 conversions and stop
    DMA1_Channel1->CPAR = (uint32_t)(&(ADC1->DR));
    DMA1_Channel1->CMAR = (uint32_t)(m_adcData);
    DMA1_Channel1->CNDTR = sizeof(m_adcData)/sizeof(m_adcData[0]);
    DMA1_Channel1->CCR |= DMA_CCR_MINC | DMA_CCR_MSIZE_0 | DMA_CCR_PSIZE_0 | DMA_CCR_TCIE;
    DMA1_Channel1->CCR |= DMA_CCR_EN;

    //start conversions
    ADC1->CR |= ADC_CR_ADSTART;
    
    //heater sense is on PA1, heater control is on PA7
    GPIOA->AFR[0] &= 0x0FFFFFFF;
    GPIOA->AFR[0] |= 0x40000000;
    GPIOA->MODER &= ~GPIO_MODER_MODER7;
    GPIOA->MODER |= GPIO_MODER_MODER7_1 | GPIO_MODER_MODER1_0 | GPIO_MODER_MODER1_1;
    GPIOA->ODR &= ~GPIO_ODR_7;

    NVIC_EnableIRQ(DMA1_Channel1_IRQn);

    heater_setDutyCycle(0);
}

void heater_setDutyCycle(uint16_t dc)
{
    AUTOLOCK_TAKE(m_mutexHandle)
    {

        if (dc == 0)
        {
            TIM14->CR1 &= ~TIM_CR1_CEN;
        }
        else
        {
            if (dc > TIM14->ARR)
                dc = TIM14->ARR;
            TIM14->CCR1 = dc;
            TIM14->CR1 |= TIM_CR1_CEN;
        }
    }
}

uint16_t heater_getTemperature()
{
    AUTOLOCK_TAKE(m_mutexHandle)
    {
        GPIOA->MODER &= GPIO_MODER_MODER7;
        GPIOA->MODER |= GPIO_MODER_MODER7_0;

        GPIOA->MODER &= GPIO_MODER_MODER7;
        GPIOA->MODER |= GPIO_MODER_MODER7_1;
    }
    return 0;
}

static uint16_t heater_getAvgAdcValue()
{
    uint32_t acc = 0;
    for (uint16_t i = 0; i < sizeof(m_adcData)/sizeof(m_adcData[0]); i++)
    {
        acc += m_adcData[i];
    }

    return acc / sizeof(m_adcData)/sizeof(m_adcData[0]);
}

void DMA1_Channel1_IRQHandler() //DMA1 ISR for when all four samples are transferred
{
    static BaseType_t xHigherPriorityTaskWoken;

    DMA1->IFCR = DMA_IFCR_CGIF1;
    xSemaphoreGiveFromISR(m_binaryISRWaitHandle, &xHigherPriorityTaskWoken);
    portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

