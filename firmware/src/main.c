/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 */

#include "stm32f0xx.h"
#include "FreeRTOS.h"
#include "task.h"

#include "buzzer.h"
#include "i2c.h"
#include "ssd1306.h"
#include "buttons.h"
#include "heater.h"

#include "font.h"

#define STACKSIZE(stk) (sizeof(stk)/sizeof(StackType_t))

void vApplicationTickHook(void);
void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName);
void vApplicationGetIdleTaskMemory( StaticTask_t **ppxIdleTaskTCBBuffer,
                                    StackType_t **ppxIdleTaskStackBuffer,
                                    uint32_t *pulIdleTaskStackSize );

StackType_t controlTaskStack[configMINIMAL_STACK_SIZE];
StaticTask_t controlTaskBuf;
static void controlTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    while (1)
    {
        vTaskDelayUntil(&nextWake, 10);
    }
}

StackType_t displayTaskStack[configMINIMAL_STACK_SIZE];
StaticTask_t displayTaskBuf;
static void displayTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    uint8_t buf[] = {0xae, 0xd5, 0x80};

    if (ssd1306_init(SSD1306_LOW))
        buzzer_beep(150, 440);

    uint8_t x = 0;
    uint8_t y = 0;
    while (1)
    {
        uint16_t hsize, vsize;
        ssd1306_clear();
        ssd1306_display();
        vTaskDelay(16);
        //buzzer.beep(10, 1000);
        //ssd1306.display();
        //if (i2c.write(0x3a, 0, &buf[0], 1))
        //    buzzer.beep(150, 440);
        //vTaskDelayUntil(&nextWake, 400);
    }
}

StackType_t mainTaskStack[configMINIMAL_STACK_SIZE];
StaticTask_t mainTaskBuf;
static void mainTask(void *pvParameters)
{
    while (1)
    {
        auto btn = buttons_getNextPress(portMAX_DELAY);
        if (btn != BTN_NONE)
        {
            buzzer_beep(100, 1000);
        }
        else
        {
            buzzer_beep(100, 2000);
        }
    }
}

int main(void)
{
    i2c_init();
    buzzer_init();
    buttons_init();
    heater_init();

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


