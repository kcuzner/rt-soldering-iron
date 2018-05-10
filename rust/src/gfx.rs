//! Small bitmap graphics system
//!
//! This interacts with the SSD1306

use bs::ssd1306;

/// Represents a point in cartesian coordinates
#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Creates a new point
    pub const fn new(x: usize, y: usize) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

/// Represents a cartesian rectangle
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    /// Creates a new Rect
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

impl<'a> From<Bitmap<'a>> for Rect {
    /// Gets a rectangle that encloses the entire bitmap
    fn from(b: Bitmap<'a>) -> Rect {
        Rect::new(0, 0, b.width(), b.height())
    }
}

/// Error originating from the gfx module
#[derive(Debug, Clone)]
pub enum GfxError {
    /// A bitmap point was out of range
    PointOutOfRange,
    /// An SSD1306 error occurred,
    DisplayError(ssd1306::DisplayError),
}

/// Renderable bitmap built from a byte slice
///
/// Bitmap data is stored elsewhere and many bitmaps can point to the same data.
#[derive(Clone)]
pub struct Bitmap<'a> {
    width: usize,
    height: usize,
    data: &'a [u8],
}

impl<'a> Bitmap<'a> {
    /// Creates a new bitmap
    ///
    /// `data` is a reference to a bitstream arranged with the LSB of the first
    /// byte being the pixel at 0,0, the LSB of the second byte at 7,0, and the
    /// MSB of the last byte being at (width-1, height-1). Unused bits in the
    /// final byte should are don't cares.
    pub const fn new(data: &'a[u8], width: usize, height: usize) -> Self {
        Bitmap {
            data: data,
            width: width,
            height: height,
        }
    }

    /// Gets the value of the buffer at some point
    pub fn at(&self, point: Point) -> Result<bool, GfxError> {
        let index = point.y * (self.width / 8) + point.x / 8;
        if index >= self.data.len() {
            return Err(GfxError::PointOutOfRange);
        }
        let mask = 0x01 << (point.x % 8);
        if self.data[index] & mask > 0 {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    /// Gets the width of this bitmap
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of this bitmap
    pub fn height(&self) -> usize {
        self.height
    }
}

/// A target for rendering a bitmap
pub trait RenderTarget {
    /// Renders the passed bitmap to this target
    fn render(&mut self, &Bitmap, src: Rect, dest: Point) -> Result<(), GfxError>;
}

impl RenderTarget for ssd1306::Display {
    /// Renders a bitmatp to this display
    fn render(&mut self, bitmap: &Bitmap, src: Rect, dest: Point) -> Result<(), GfxError> {
        for y in src.y..src.height+src.y {
            for x in src.x..src.width+src.x {
                match bitmap.at(Point::new(x, y)) {
                    Ok(v) => match self.set_pixel(x + dest.x, y + dest.y, v) {
                        Err(e) => Err(GfxError::DisplayError(e)),
                        _ => Ok(()),
                    },
                    Err(e) => Err(e),
                }?;
            }
        }
        Ok(())
    }
}

