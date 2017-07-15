/**
 * RT Soldering Iron
 *
 * Kevin Cuzner
 *
 * Bitmap Font
 */

#ifndef _FONT_H_
#define _FONT_H_

#include <stdint.h>

#ifdef __cplusplus
 extern "C" {
#endif /* __cplusplus */

typedef enum { FONT_16x32 } FontType;

/**
 * Gets a pointer to a character bitmap for the passed font and character
 *
 * font: Font to find the bitmap in
 * character: Character to find
 * width: Pointer to a word to fill with the width of the bitmap
 * height: Pointer to a word to fill with the height of the bitmap
 */
const uint8_t *font_get_character(FontType font, char character, uint16_t *width, uint16_t *height);

#ifdef __cplusplus
 }
#endif /* __cplusplus */

#endif //_FONT_H_

