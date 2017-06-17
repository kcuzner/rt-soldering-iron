/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 */

#include "stm32f0xx.h"
#include "FreeRTOS.h"
#include "task.h"

#include "buzzer.hpp"

extern "C" {
    void vApplicationMallocFailedHook(void);
    void vApplicationTickHook(void);
    void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName);
}

Buzzer buzzer;

static void beepTask(void *pvParameters)
{
    TickType_t nextWake = xTaskGetTickCount();
    while (1)
    {
        vTaskDelayUntil(&nextWake, 1000);
    }
}

int main(void)
{
    xTaskCreate(beepTask, "B", configMINIMAL_STACK_SIZE, NULL, 2, NULL);
    vTaskStartScheduler();

    while (1) { }
    return 0;
}

void vApplicationMallocFailedHook(void)
{
}

void vApplicationTickHook(void)
{
}

void vApplicationStackOverflowHook(TaskHandle_t task, char *taskName)
{
}

