/**
 * RT Soldering Iron
 *
 * Buttons
 *
 * Kevin Cuzner
 */

#ifndef _BUTTONS_H_
#define _BUTTONS_H_

#include <stdint.h>

#include "FreeRTOS.h"
#include "queue.h"
#include "semphr.h"

typedef enum { BTN_NONE = 0, BTN_UP = 1 << 0, BTN_LEFT = 1 << 1, BTN_DOWN = 1 << 2, BTN_RIGHT = 1 << 3, BTN_CENTER = 1 << 4 } ButtonPress;

void buttons_init();
ButtonPress buttons_getNextPress(const TickType_t timeout);
ButtonPress buttons_getCurrentState();

#endif //_BUTTONS_H_

