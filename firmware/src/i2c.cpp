/**
 * RT Soldering Iron
 *
 * I2C
 *
 * Kevin Cuzner
 */

#include "i2c.hpp"

#include "stm32f0xx.h"
#include "system_stm32f0xx.h"
#include "autolock.hpp"

using namespace std;

I2C::I2C()
{
    //Intialize mutex and binary semaphore
    m_mutexHandle = xSemaphoreCreateMutexStatic(&m_mutex);
    m_binaryISRWaitHandle = xSemaphoreCreateBinaryStatic(&m_binaryISRWait);

    //Enable clocks
    RCC->APB1ENR |= RCC_APB1ENR_I2C1EN;
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;

    //Set PB6 and PB7 to Alternate Function, AF1
    GPIOB->AFR[0] &= 0x00FFFFFF;
    GPIOB->AFR[0] |= 0x11000000;
    GPIOB->MODER &= ~(GPIO_MODER_MODER6 | GPIO_MODER_MODER7);
    GPIOB->MODER |= GPIO_MODER_MODER6_1 | GPIO_MODER_MODER7_1;
    GPIOB->OTYPER |= GPIO_OTYPER_OT_6 | GPIO_OTYPER_OT_7;

    //Set up timing and add a callback to oscillator changes
    setTiming();
    //osc_add_callback(&i2c_set_timing);
    
    NVIC_EnableIRQ(I2C1_IRQn);
    ISR::isr(I2C1_IRQn).attach(this);
}

bool I2C::write(uint8_t address, uint8_t reg, const uint8_t *buffer, uint8_t len)
{
    Autolock l_lock(m_mutexHandle);

    I2C1->CR1 = I2C_CR1_PE; //enable peripheral

    //send address, register byte, and buffer data
    I2C1->CR2 = I2C_CR2_AUTOEND | ((len + 1) << I2C_CR2_NBYTES_Pos) |
        I2C_CR2_START | (address << I2C_CR2_SADD_Pos);

    I2C1->ISR |= I2C_ISR_TXE;
    I2C1->TXDR = reg;

    uint8_t i = 0;
    do
    {
        if (!waitForMask(I2C_ISR_STOPF | I2C_ISR_TXIS))
            return false;

        if (I2C1->ISR & I2C_ISR_TXIS)
            I2C1->TXDR = buffer[i++];
    }
    while (!(I2C1->ISR & I2C_ISR_STOPF));

    I2C1->CR1 = 0;

    return true;
}

bool I2C::read(uint8_t address, uint8_t reg, uint8_t *buffer, uint8_t len)
{
    Autolock l_lock(m_mutexHandle);

    I2C1->CR1 = I2C_CR1_PE; //enable peripheral

    //send address + register byte
    I2C1->CR2 = (1 << I2C_CR2_NBYTES_Pos) |
        I2C_CR2_START | (address << I2C_CR2_SADD_Pos);

    I2C1->ISR |= I2C_ISR_TXE;
    I2C1->TXDR = reg;

    if (!waitForMask(I2C_ISR_TC))
        return false;

    //restart, send address, read bytes
    I2C1->CR2 = I2C_CR2_AUTOEND | (len << I2C_CR2_NBYTES_Pos) |
        I2C_CR2_START | I2C_CR2_RD_WRN | (address << I2C_CR2_SADD_Pos);
 
    uint8_t i = 0;
    do
    {

        if (!waitForMask(I2C_ISR_STOPF | I2C_ISR_RXNE))
            return false;
        
        if (I2C1->ISR & I2C_ISR_RXNE)
        {
            buffer[i++] = I2C1->RXDR;
        }
    }
    while (!(I2C1->ISR  & I2C_ISR_STOPF));

    I2C1->CR1 = 0;

    return true;
}

void I2C::isr()
{
    static BaseType_t xHigherPriorityTaskWoken;

    //disable interrupts
    I2C1->CR1 &= ~(I2C_CR1_ERRIE | I2C_CR1_TCIE | I2C_CR1_STOPIE | I2C_CR1_NACKIE | I2C_CR1_ADDRIE | I2C_CR1_RXIE | I2C_CR1_TXIE);
    xSemaphoreGiveFromISR(m_binaryISRWaitHandle, &xHigherPriorityTaskWoken);
    portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

void I2C::setTiming(void)
{
    uint32_t prescaler = 1;// SystemCoreClock / 2000000;
    if (prescaler > 15)
        prescaler = 15;
    //Set up for master mode, 100KHz, assuming an 8MHz clock
    I2C1->TIMINGR = ((prescaler & 0xF) << I2C_TIMINGR_PRESC_Pos) |
        ((3 & 0xF) << I2C_TIMINGR_SCLDEL_Pos) |
        ((1 & 0xF) << I2C_TIMINGR_SDADEL_Pos) |
        ((3 & 0xFF) << I2C_TIMINGR_SCLH_Pos) |
        ((9 & 0xFF) << I2C_TIMINGR_SCLL_Pos);
}

/**
 * Waits for the I2C1 peripheral to reach a passed state
 *
 * mask: Mask to match and return true
 *
 * Returns true if any bit in the passed mask is matched or false if any error bit or nack bit is set
 */
bool I2C::waitForMask(std::uint16_t mask)
{
    while (true)
    {
        //interrupt on everything
        I2C1->CR1 |= I2C_CR1_ERRIE | I2C_CR1_TCIE | I2C_CR1_STOPIE | I2C_CR1_NACKIE | I2C_CR1_ADDRIE | I2C_CR1_RXIE | I2C_CR1_TXIE;
        //wait for the interrupt to finish (the interrupt turns off interrupts)
        xSemaphoreTake(m_binaryISRWaitHandle, portMAX_DELAY);

        if ((I2C1->ISR & I2C_ISR_ARLO) | (I2C1->ISR & I2C_ISR_BERR) | (I2C1->ISR & I2C_ISR_NACKF))
        {
            return false;
        }
        if (I2C1->ISR & mask)
        {
            return true;
        }
    }
}

