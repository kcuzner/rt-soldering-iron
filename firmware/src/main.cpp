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
Buttons buttons;
Heater heater;

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

const uint8_t bitmap[] = {
0x00, 0x00, 0x0C, 0x0C, 0x0E, 0x12, 0x16, 0x1F, 0x31, 0x00, 0x00, 0x00, 
  0x00,
};

static char character = '0';

static StackType_t i2cTaskStack[120];
static StaticTask_t i2cTaskBuf;
static void i2cTask(void *pvParameters)
{
    Buttons::Button btn;
    TickType_t nextWake = xTaskGetTickCount();
    uint8_t buf[] = {0xae, 0xd5, 0x80};

    if (ssd1306.initialize())
        buzzer.beep(150, 440);

    heater.setStandby(false);
  
    ssd1306.blit(45, 0, 8, 4, bitmap);
    ssd1306.blit(53, 1, 8, 4, bitmap);
    ssd1306.blit(61, 2, 8, 4, bitmap);
    //ssd1306.blit(80, 8, 4, 8, bitmap);
    ssd1306.display();


    uint8_t x = 0;
    uint8_t y = 0;
    while (1)
    {
        uint16_t hsize, vsize;
        ssd1306.clear();
        ssd1306.blit(x, y, hsize, vsize, font_get_character(FONT_16x32, character, &hsize, &vsize));
        ssd1306.hline(5, 5, 100);
        ssd1306.vline(10, 10, 30);
        ssd1306.display();
        x += 1;
        y += 1;
        if (y > 31)
            y = 0;
        if (x > 127)
            x = 0;
        vTaskDelay(16);
        //buzzer.beep(10, 1000);
        //ssd1306.display();
        //if (i2c.write(0x3a, 0, &buf[0], 1))
        //    buzzer.beep(150, 440);
        //vTaskDelayUntil(&nextWake, 400);
    }
}

static StackType_t keyTaskStack[120];
static StaticTask_t keyTaskBuf;
static void keypressTask(void *pvParameters)
{
    while (1)
    {
        auto btn = buttons.getNextPress();
        if (btn != Buttons::Button::NONE)
        {
            if (btn == Buttons::Button::UP)
            {
                character++;
                if (character == ':')
                    character = 'A';
                if (character == '[')
                    character = '0';
            }
            else if (btn == Buttons::Button::DOWN)
            {
                character--;
                if (character == '/')
                    character = 'Z';
                if (character == '@')
                    character = '9';
            }
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
    xTaskCreateStatic(beepTask, "B", configMINIMAL_STACK_SIZE, NULL, 2, beepTaskStack, &beepTaskBuf);
    xTaskCreateStatic(i2cTask, "I", 120, NULL, 1, i2cTaskStack, &i2cTaskBuf);
    xTaskCreateStatic(keypressTask, "K", 120, NULL, 1, keyTaskStack, &keyTaskBuf);
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


