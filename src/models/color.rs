use std::fmt::Display;

/// Represents a color, most likely fetched from or to be used for Discord.
#[derive(Clone, Debug)]
pub struct Color {
    /// The integer value of this color.
    pub value: u32,
}

impl Color {
    /// Creates a new color with the given decimal value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rs_cord::Color;
    ///
    /// const RED = Color::new(0xFF0000);
    /// ```
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Creates a new color with the given RGB color.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: ((r as u32) << 16) + ((g as u32) << 8) + (b as u32),
        }
    }

    fn _get_rgb_byte(&self, b: u8) -> u8 {
        ((self.value >> (b * 8)) & 0xFF) as u8
    }

    /// Returns the decimal value of this color.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Returns this color as an RGB representation.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            self._get_rgb_byte(2),
            self._get_rgb_byte(1),
            self._get_rgb_byte(0),
        )
    }

    /// Returns this color as a lowercase hex string, which starts with `#`.
    pub fn to_hex_string(&self) -> String {
        format!("#{:06x}", self.value)
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self.value
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:06x}", self.value)
    }
}

pub type Colour = Color;
