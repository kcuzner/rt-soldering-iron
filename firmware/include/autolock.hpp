/**
 * RT Soldering Iron
 *
 * Autolock declaration
 *
 * Kevin Cuzner
 */

#pragma once

#include "FreeRTOS.h"
#include "semphr.h"

class Autolock
{
public:
    Autolock(SemaphoreHandle_t &semaphore) : m_semaphore(semaphore) { xSemaphoreTake(semaphore, portMAX_DELAY); }
    ~Autolock() { xSemaphoreGive(m_semaphore); }

private:
    SemaphoreHandle_t m_semaphore;
};

