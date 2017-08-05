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
#include "buttons.hpp"
#include "heater.hpp"

#include "font.h"

#define STACKSIZE(stk) (sizeof(stk)/sizeof(StackType_t))

extern "C" {
    void vApplicationTickHook(void);
    void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName);
    void vApplicationGetIdleTaskMemory( StaticTask_t **ppxIdleTaskTCBBuffer,
                                        StackType_t **ppxIdleTaskStackBuffer,
                                        uint32_t *pulIdleTaskStackSize );
}

I2C i2c;
Buzzer buzzer;
Buttons buttons;
Heater heater;

static StackType_t controlTaskStack[120];
static StaticTask_t controlTaskBuf;
static void controlTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    while (1)
    {
        vTaskDelayUntil(&nextWake, 10);
    }
}

static StackType_t displayTaskStack[120];
static StaticTask_t displayTaskBuf;
static void displayTask(void *pvParameters)
{
    SSD1306 ssd1306(&i2c, SSD1306::Address::LOW);
    TickType_t nextWake = xTaskGetTickCount();
    uint8_t buf[] = {0xae, 0xd5, 0x80};

    if (ssd1306.initialize())
        buzzer.beep(150, 440);

    uint8_t x = 0;
    uint8_t y = 0;
    while (1)
    {
        uint16_t hsize, vsize;
        ssd1306.clear();
        ssd1306.display();
        vTaskDelay(16);
        //buzzer.beep(10, 1000);
        //ssd1306.display();
        //if (i2c.write(0x3a, 0, &buf[0], 1))
        //    buzzer.beep(150, 440);
        //vTaskDelayUntil(&nextWake, 400);
    }
}

static StackType_t mainTaskStack[120];
static StaticTask_t mainTaskBuf;
static void mainTask(void *pvParameters)
{
    while (1)
    {
        auto btn = buttons.getNextPress();
        if (btn != Buttons::Button::NONE)
        {
            buzzer.beep(100, heater.getAvgAdcValue() + 1);
        }
        else
        {
            buzzer.beep(100, 2000);
        }
    }
}

int main(void)
{
    xTaskCreateStatic(mainTask, "M", STACKSIZE(mainTaskStack), NULL, 2, mainTaskStack, &mainTaskBuf);
    xTaskCreateStatic(displayTask, "D", STACKSIZE(displayTaskStack), NULL, 1, displayTaskStack, &displayTaskBuf);
    xTaskCreateStatic(controlTask, "C", STACKSIZE(controlTaskStack), NULL, 3, controlTaskStack, &controlTaskBuf);
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


