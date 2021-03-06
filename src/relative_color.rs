
use internal::{
    CanColor,
    Color,
    ColorComponent,
    HasColor,
};
use vecmath::{
    hsv,
};

/// Implemented by contexts that contains color.
pub trait RelativeColor<'a, T> {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> T;

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(&'a self, f: ColorComponent) -> T {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(&'a self, f: ColorComponent) -> T {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }
    
    /// Rotates hue by degrees.
    #[inline(always)]
    fn hue_deg(&'a self, angle: ColorComponent) -> T {
        let pi: ColorComponent = Float::pi();
        self.hue_rad(angle * pi / 180.0)
    }

    /// Rotates hue by radians.
    fn hue_rad(&'a self, angle: ColorComponent) -> T;
}

impl<
    'a,
    T: HasColor<'a, Color> + CanColor<'a, U, Color>,
    U
> RelativeColor<'a, U> for T {
    #[inline(always)]
    fn mul_rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> U {
        let color = self.get_color();
        self.color([color[0] * r, color[1] * g, color[2] * b, color[3] * a])
    }
    
    #[inline(always)]
    fn hue_rad(&'a self, angle: ColorComponent) -> U {
        self.color(hsv(*self.get_color(), angle, 1.0, 1.0))
    }
}

