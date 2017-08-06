/**
 * RT Soldering Iron
 *
 * Buzzer component
 *
 * Kevin Cuzner
 */

#ifndef _BUZZER_H_
#define _BUZZER_H_

#include <stdint.h>
#include "FreeRTOS.h"
#include "semphr.h"

void buzzer_init();

void buzzer_beep(uint16_t duration_ms, uint16_t frequency_hz);

#endif //_BUZZER_H_

