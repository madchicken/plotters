/// The color type that is used by all the backend
#[derive(Clone, Copy)]
pub struct BackendColor {
    pub alpha: f64,
    pub rgb: (u8, u8, u8),
}

impl BackendColor {
    #[inline(always)]
    pub fn mix(&self, alpha: f64) -> Self {
        Self {
            alpha: self.alpha * alpha,
            rgb: self.rgb,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StrokeStyle {
    None, Dotted, Dashed, DashDotted
}

/// The style data for the backend drawing API
pub trait BackendStyle {
    /// Get the color of current style
    fn color(&self) -> BackendColor;

    /// Get the stroke width of current style
    fn stroke_width(&self) -> u32 {
        1
    }

    /// Get the current stroke style
    fn stroke_style(&self) -> StrokeStyle { StrokeStyle::None }
}

impl BackendStyle for BackendColor {
    fn color(&self) -> BackendColor {
        *self
    }
}
