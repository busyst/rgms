/// Represents a color in the RGB color space.
/// Each component (red, green, blue) is stored as an 8-bit unsigned integer (0-255).
#[derive(Debug)]
pub struct Rgb {
   /// Red
   r: u8,
   /// Green
   g: u8,
   /// Blue
   b: u8,
}

impl Rgb {
    /// Creates a new RGB color with the specified components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// let purple = Rgb::new(128, 0, 128);
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    /// Adds two RGB colors together using saturating addition.
    /// If any component would exceed 255, it is capped at 255.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The RGB color to add to this one
    ///
    /// # Examples
    ///
    /// ```
    /// let color1 = Rgb::new(100, 150, 200);
    /// let color2 = Rgb::new(50, 75, 100);
    /// let combined = color1 + color2; // Results in Rgb { r: 150, g: 225, b: 255 }
    fn add(&self, rhs: Rgb) -> Rgb {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
    /// Linearly interpolates between two colors.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Ending color
    /// * `x` - Interpolation factor (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// let red = Rgb::new(255, 0, 0);
    /// let blue = Rgb::new(0, 0, 255);
    /// let purple = Rgb::lerp(red, blue, 0.5); // Results in Rgb { r: 128, g: 0, b: 128 }
    /// ```
    pub fn lerp(&self, rhs: Rgb, x: f32) -> Rgb {
        let x = x.clamp(0.0, 1.0);
        Self {
            r: ((self.r as f32) * (1.0 - x) + (rhs.r as f32) * x) as u8,
            g: ((self.g as f32) * (1.0 - x) + (rhs.g as f32) * x) as u8,
            b: ((self.b as f32) * (1.0 - x) + (rhs.b as f32) * x) as u8,
        }
    }
    /// Performs additive color blending.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Second color
    ///
    /// # Examples
    ///
    /// ```
    /// let red = Rgb::new(255, 0, 0);
    /// let blue = Rgb::new(0, 0, 255);
    /// let magenta = Rgb::blend(red, blue); // Additive mixing
    /// ```
    pub fn blend(&self, rhs: Rgb) -> Rgb {
        Self {
            r: (((self.r as u16) + (rhs.r as u16)) / 2) as u8,
            g: (((self.g as u16) + (rhs.g as u16)) / 2) as u8,
            b: (((self.b as u16) + (rhs.b as u16)) / 2) as u8,
        }
    }


}

impl Default for Rgb {
   /// Returns a black color (RGB: 0, 0, 0)
   ///
   /// # Examples
   ///
   /// ```
   /// let black = Rgb::default();
   /// ```
   fn default() -> Self {
       Self { r: 0, g: 0, b: 0 }
   }
}

impl Clone for Rgb {
   /// Creates a copy of the RGB color
   fn clone(&self) -> Self {
       Self { r: self.r, g: self.g, b: self.b }
   }
}