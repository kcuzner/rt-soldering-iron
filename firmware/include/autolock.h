/**
 * RT Soldering Iron
 *
 * Autolock declaration
 *
 * Kevin Cuzner
 */

#ifndef _AUTOLOCK_H_
#define _AUTOLOCK_H_

#include "FreeRTOS.h"
#include "semphr.h"
#include <stdint.h>

typedef struct {
    SemaphoreHandle_t s;
    uint8_t v;
} __AutolockType;

/**
 * Helper function which takes the passed semaphore
 */
static __inline__ SemaphoreHandle_t __semaphore_take(SemaphoreHandle_t s) { xSemaphoreTake(s, portMAX_DELAY); return s; }
/**
 * Helper function which gives the passed semaphore
 */
static __inline__ void __semaphore_give(__AutolockType *a) { xSemaphoreGive(a->s); }

/**
 * Takes a semaphore for the next statement and releases it afterwards
 *
 * Usage:
 * AUTOLOCK_TAKE(semaphoreHandle)
 * {
 *    ...Do you thing
 * }
 */
#define AUTOLOCK_TAKE(semaphore) for (__AutolockType __AUTOLOCK __attribute__((cleanup(__semaphore_give))) = { __semaphore_take(semaphore), 1 }; __AUTOLOCK.v; __AUTOLOCK.v = 0)

#endif //_AUTOLOCK_H_

