//! General traits. Mostly implemented in arch/*.

/// A type used for color in the functions of [TextDisplay].
/// 
/// Type alias for (u8, bool). Boolean argument is whether to
/// change the value(i.e. for [COLOR_BLACK] and [COLOR_DEFAULT]).
pub type Color = (u8, bool);

/// Black-on-black.
pub const COLOR_BLACK: Color = (0, true);

/// Should be whatever color commonly used for status messages.
/// Generally should be white-on-black. Value is one.
pub const COLOR_DEFAULT: Color = (1, true);

/// Some form of display that can be written too with text.
pub trait TextDisplay {
    /// Writes a single character to the specified position.
    fn write_char(&self, pos: (u32, u32), char: u8, color: Color) -> Result<(), crate::Error<'static>>;
    /// Gets the size of the screen.
    fn get_size(&self) -> (u32, u32);
}

impl dyn TextDisplay + '_ {
    /// Clears the screen.
    pub fn clear_screen(&self, color: Color) {
        let (width, height) = self.get_size();
        for x in 0..width {
            for y in 0..height {
                self.write_char((x, y), b' ', color).unwrap();
            }
        }
    }

    /// Writes a &str to the screen.
    pub fn write_str(&self, pos: (u32, u32), str: &str, color: Color) -> Result<(u32, u32), crate::Error<'static>> {
        let (width, _) = self.get_size();
        let (mut x, mut y) = pos;
        for char in str.as_bytes() {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>width {
                x -= width;
                y += 1;
            }
        }
        Ok((x, y))
    }

    /// Writes a &\[u8] to the screen.
    pub fn write_bytes(&self, pos: (u32, u32), str: &[u8], color: Color) -> Result<(u32, u32), crate::Error<'static>> {
        let (width, _) = self.get_size();
        let (mut x, mut y) = pos;
        for char in str {
            self.write_char((x, y), *char, color)?;
            if *char == 0 {
                continue
            }
            x += 1;
            while x>width {
                x -= width;
                y += 1;
            }
        }
        Ok((x, y))
    }
}