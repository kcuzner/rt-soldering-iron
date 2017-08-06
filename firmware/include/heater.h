/**
 * RT Soldering Iron
 *
 * Heater Control
 */

#ifndef _HEATER_H_
#define _HEATER_H_

#include "FreeRTOS.h"
#include "semphr.h"

#include <stdint.h>

#define HEATER_PWMPERIOD 0x1FF

void heater_init();
void heater_setDutyCycle(uint16_t dc);
uint16_t heater_getTemperature();

#endif //_HEATER_H_

