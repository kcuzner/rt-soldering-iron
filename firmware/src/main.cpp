/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 */

#include "stm32f0xx.h"
#include "FreeRTOS.h"
#include "task.h"

#include "buzzer.hpp"
#include "i2c.hpp"
#include "ssd1306.hpp"

extern "C" {
    void vApplicationTickHook(void);
    void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName);
    void vApplicationGetIdleTaskMemory( StaticTask_t **ppxIdleTaskTCBBuffer,
                                        StackType_t **ppxIdleTaskStackBuffer,
                                        uint32_t *pulIdleTaskStackSize );
}

I2C i2c;
Buzzer buzzer;
SSD1306 ssd1306(&i2c, SSD1306::Address::LOW);

static StackType_t beepTaskStack[configMINIMAL_STACK_SIZE];
static StaticTask_t beepTaskBuf;
static void beepTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    while (1)
    {
        //buzzer.beep(100, 1000);
        vTaskDelayUntil(&nextWake, 1000);
    }
}

static StackType_t i2cTaskStack[120];
static StaticTask_t i2cTaskBuf;
static void i2cTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    uint8_t buf[] = {0xae, 0xd5, 0x80};

    buzzer.beep(100, 1200);
    if (ssd1306.initialize()) { }
        //buzzer.beep(150, 440);

    while (1)
    {
        //buzzer.beep(10, 1000);
        //ssd1306.display();
        //if (i2c.write(0x3a, 0, &buf[0], 1))
        //    buzzer.beep(150, 440);
        vTaskDelayUntil(&nextWake, 400);
    }
}

int main(void)
{
    xTaskCreateStatic(beepTask, "B", configMINIMAL_STACK_SIZE, NULL, 2, beepTaskStack, &beepTaskBuf);
    xTaskCreateStatic(i2cTask, "I", 120, NULL, 1, i2cTaskStack, &i2cTaskBuf);
    vTaskStartScheduler();

    while (1) { }
    return 0;
}

void vApplicationTickHook(void)
{
}

void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName)
{
}

void vApplicationGetIdleTaskMemory( StaticTask_t **ppxIdleTaskTCBBuffer,
                                    StackType_t **ppxIdleTaskStackBuffer,
                                    uint32_t *pulIdleTaskStackSize )
{
    static StackType_t idleStack[configMINIMAL_STACK_SIZE];
    static StaticTask_t idleTask;
    *ppxIdleTaskTCBBuffer = &idleTask;
    *ppxIdleTaskStackBuffer = idleStack;
    *pulIdleTaskStackSize = configMINIMAL_STACK_SIZE;
}


