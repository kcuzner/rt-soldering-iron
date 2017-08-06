/**
 * RT Soldering Iron
 *
 * SSD1306 Display Driver for the 128x32 display being used on this project
 *
 * Kevin Cuzner
 */

#ifndef _SSD1306_H_
#define _SSD1306_H_

#include <stdint.h>
#include <stdbool.h>

typedef enum { SSD1306_LOW = 0x78, SSD1306_HIGH = 0x7a } SSD1306Address;

bool ssd1306_init(SSD1306Address addr);

/**
 * Sends the current buffer to the display
 */
bool ssd1306_display();

/**
 * Clears the buffer
 */
void ssd1306_clear();
/**
 * Sets a single pixel in the buffer
 */
void ssd1306_setPixel(uint8_t x, uint8_t y, uint8_t value);
/**
 * Blits a bitmap to the buffer. The bitmap should be arranged with the bytes
 * as rows.
 */
void ssd1306_blit(uint8_t destX, uint8_t destY, uint8_t width, uint8_t height, const uint8_t *buffer);

void ssd1306_hline(uint8_t startX, uint8_t startY, uint8_t endX);

void ssd1306_vline(uint8_t startX, uint8_t startY, uint8_t endY);


#endif //_SSD1306_H_

