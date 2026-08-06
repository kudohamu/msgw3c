use crate::{
  color::C,
  formula,
  porter_duff::{CompositeOperator, PorterDuff},
};

/// Blend modes defined in the following W3C specification.
/// <https://www.w3.org/TR/compositing-1/#blending>
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
  #[default]
  Normal,
  Multiply,
  Screen,
  Overlay,
  Darken,
  Lighten,
  ColorDodge,
  ColorBurn,
  HardLight,
  SoftLight,
  Difference,
  Exclusion,
  Hue,
  Saturation,
  Color,
  Luminosity,
}

/// A trait that represents blendable types.
/// Implementing this trait enables blending
/// that complies with the W3C specification.
pub trait Blend: Sized {
  /// Converts the color type [`C`] used internally by the Blend trait to the implemented type
  ///
  /// ```
  /// // Your color type.
  /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// struct Rgba {
  ///     r: u8,
  ///     g: u8,
  ///     b: u8,
  ///     a: u8,
  ///  }
  ///
  /// impl Blend for Rgba {
  ///   fn from_color(color: C) -> Self {
  ///     Self {
  ///       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  ///       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  ///       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  ///       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  ///     }
  ///   }
  ///
  ///   ...
  /// }
  /// ```
  fn from_color(c: C) -> Self;
  /// Converts the implemented type to color type [`C`], which the Blend trait uses internally
  ///
  /// ```
  /// // Your color type.
  /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// struct Rgba {
  ///     r: u8,
  ///     g: u8,
  ///     b: u8,
  ///     a: u8,
  ///  }
  ///
  /// impl Blend for Rgba {
  ///   fn to_color(&self) -> C {
  ///     C::new(
  ///       self.r as f32 / 255.,
  ///       self.g as f32 / 255.,
  ///       self.b as f32 / 255.,
  ///       self.a as f32 / 255.,
  ///     )
  ///   }
  ///
  ///   ...
  /// }
  /// ```
  fn to_color(&self) -> C;

  /// The utility function for `normal` blend. It is composited using `SourceOver`.
  /// Use [`Self::normal_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.normal(&backdrop);
  /// ```
  fn normal(&self, backdrop: &impl Blend) -> Self {
    self.normal_with(backdrop, PorterDuff::SourceOver)
  }

  /// `normal` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.normal_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn normal_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Normal, op)
  }

  /// The utility function for `multiply` blend. It is composited using `SourceOver`.
  /// Use [`Self::multiply_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.multiply(&backdrop);
  /// ```
  fn multiply(&self, backdrop: &impl Blend) -> Self {
    self.multiply_with(backdrop, PorterDuff::SourceOver)
  }

  /// `multiply` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.multiply_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn multiply_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Multiply, op)
  }

  /// The utility function for `screen` blend. It is composited using `SourceOver`.
  /// Use [`Self::screen_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.screen(&backdrop);
  /// ```
  fn screen(&self, backdrop: &impl Blend) -> Self {
    self.screen_with(backdrop, PorterDuff::SourceOver)
  }

  /// `screen` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.screen_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn screen_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Screen, op)
  }

  /// The utility function for `overlay` blend. It is composited using `SourceOver`.
  /// Use [`Self::overlay_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.overlay(&backdrop);
  /// ```
  fn overlay(&self, backdrop: &impl Blend) -> Self {
    self.overlay_with(backdrop, PorterDuff::SourceOver)
  }

  /// `overlay` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.overlay_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn overlay_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Overlay, op)
  }

  /// The utility function for `darken` blend. It is composited using `SourceOver`.
  /// Use [`Self::darken_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.darken(&backdrop);
  /// ```
  fn darken(&self, backdrop: &impl Blend) -> Self {
    self.darken_with(backdrop, PorterDuff::SourceOver)
  }

  /// `darken` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.darken_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn darken_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Darken, op)
  }

  /// The utility function for `lighten` blend. It is composited using `SourceOver`.
  /// Use [`Self::lighten_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.lighten(&backdrop);
  /// ```
  fn lighten(&self, backdrop: &impl Blend) -> Self {
    self.lighten_with(backdrop, PorterDuff::SourceOver)
  }

  /// `lighten` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.lighten_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn lighten_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Lighten, op)
  }

  /// The utility function for `color_dodge` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_dodge_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color_dodge(&backdrop);
  /// ```
  fn color_dodge(&self, backdrop: &impl Blend) -> Self {
    self.color_dodge_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_dodge` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color_dodge_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn color_dodge_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorDodge, op)
  }

  /// The utility function for `color_burn` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_burn_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color_burn(&backdrop);
  /// ```
  fn color_burn(&self, backdrop: &impl Blend) -> Self {
    self.color_burn_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_burn` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color_burn_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn color_burn_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorBurn, op)
  }

  /// The utility function for `hard_light` blend. It is composited using `SourceOver`.
  /// Use [`Self::hard_light_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.hard_light(&backdrop);
  /// ```
  fn hard_light(&self, backdrop: &impl Blend) -> Self {
    self.hard_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hard_light` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.hard_light_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn hard_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::HardLight, op)
  }

  /// The utility function for `soft_light` blend. It is composited using `SourceOver`.
  /// Use [`Self::soft_light_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.soft_light(&backdrop);
  /// ```
  fn soft_light(&self, backdrop: &impl Blend) -> Self {
    self.soft_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `soft_light` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.soft_light_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn soft_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::SoftLight, op)
  }

  /// The utility function for `difference` blend. It is composited using `SourceOver`.
  /// Use [`Self::difference_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.difference(&backdrop);
  /// ```
  fn difference(&self, backdrop: &impl Blend) -> Self {
    self.difference_with(backdrop, PorterDuff::SourceOver)
  }

  /// `difference` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.difference_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn difference_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Difference, op)
  }

  /// The utility function for `exclusion` blend. It is composited using `SourceOver`.
  /// Use [`Self::exclusion_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.exclusion(&backdrop);
  /// ```
  fn exclusion(&self, backdrop: &impl Blend) -> Self {
    self.exclusion_with(backdrop, PorterDuff::SourceOver)
  }

  /// `exclusion` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.exclusion_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn exclusion_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Exclusion, op)
  }

  /// The utility function for `hue` blend. It is composited using `SourceOver`.
  /// Use [`Self::hue_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.hue(&backdrop);
  /// ```
  fn hue(&self, backdrop: &impl Blend) -> Self {
    self.hue_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hue` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.hue_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn hue_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Hue, op)
  }

  /// The utility function for `saturation` blend. It is composited using `SourceOver`.
  /// Use [`Self::saturation_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.saturation(&backdrop);
  /// ```
  fn saturation(&self, backdrop: &impl Blend) -> Self {
    self.saturation_with(backdrop, PorterDuff::SourceOver)
  }

  /// `saturation` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.saturation_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn saturation_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Saturation, op)
  }

  /// The utility function for `color` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color(&backdrop);
  /// ```
  fn color(&self, backdrop: &impl Blend) -> Self {
    self.color_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.color_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn color_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Color, op)
  }

  /// The utility function for `luminosity` blend. It is composited using `SourceOver`.
  /// Use [`Self::luminosity_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.luminosity(&backdrop);
  /// ```
  fn luminosity(&self, backdrop: &impl Blend) -> Self {
    self.luminosity_with(backdrop, PorterDuff::SourceOver)
  }

  /// `luminosity` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// let result = source.luminosity_with(&backdrop, PorterDuff::Destination)
  /// ```
  fn luminosity_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Luminosity, op)
  }

  /// Blend function that allows you to dynamically specify blend mode and composite operator.
  ///
  /// ```
  /// let result = source.blend_with(&backdrop, BlendMode::Lighten, PorterDuff::SourceAtop)
  /// ```
  fn blend_with(&self, backdrop: &impl Blend, mode: BlendMode, op: PorterDuff) -> Self {
    match mode {
      BlendMode::Normal => Self::composite_separable(formula::normal, op)(backdrop, self),
      BlendMode::Multiply => Self::composite_separable(formula::multiply, op)(backdrop, self),
      BlendMode::Screen => Self::composite_separable(formula::screen, op)(backdrop, self),
      BlendMode::Overlay => Self::composite_separable(formula::overlay, op)(backdrop, self),
      BlendMode::Darken => Self::composite_separable(formula::darken, op)(backdrop, self),
      BlendMode::Lighten => Self::composite_separable(formula::lighten, op)(backdrop, self),
      BlendMode::ColorDodge => Self::composite_separable(formula::color_dodge, op)(backdrop, self),
      BlendMode::ColorBurn => Self::composite_separable(formula::color_burn, op)(backdrop, self),
      BlendMode::HardLight => Self::composite_separable(formula::hard_light, op)(backdrop, self),
      BlendMode::SoftLight => Self::composite_separable(formula::soft_light, op)(backdrop, self),
      BlendMode::Difference => Self::composite_separable(formula::difference, op)(backdrop, self),
      BlendMode::Exclusion => Self::composite_separable(formula::exclusion, op)(backdrop, self),
      BlendMode::Hue => Self::composite_non_separable(formula::hue, op)(backdrop, self),
      BlendMode::Saturation => {
        Self::composite_non_separable(formula::saturation, op)(backdrop, self)
      }
      BlendMode::Color => Self::composite_non_separable(formula::color, op)(backdrop, self),
      BlendMode::Luminosity => {
        Self::composite_non_separable(formula::luminosity, op)(backdrop, self)
      }
    }
  }

  /// General composite function for each component of the result color is completely determined by
  /// the corresponding components of the constituent backdrop and source colors.
  /// <https://www.w3.org/TR/compositing-1/#blendingseparable>
  fn composite_separable<B: Blend, F, Op: CompositeOperator>(
    f: F,
    op: Op,
  ) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn(f32, f32) -> f32,
  {
    move |backdrop, src| -> Self {
      let cb = backdrop.to_color();
      let cs = src.to_color();

      if cs.a == 0. && cb.a == 0. {
        return Self::from_color(C::ZERO);
      }

      // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
      // https://www.w3.org/TR/compositing-1/#blending
      let blended_r = (1. - cb.a) * cs.r + cb.a * f(cb.r, cs.r);
      let blended_g = (1. - cb.a) * cs.g + cb.a * f(cb.g, cs.g);
      let blended_b = (1. - cb.a) * cs.b + cb.a * f(cb.b, cs.b);

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators
      let (fa, fb) = op.fractions(cs.a, cb.a);
      let pm_r = cs.a * fa * blended_r + cb.a * fb * cb.r;
      let pm_g = cs.a * fa * blended_g + cb.a * fb * cb.g;
      let pm_b = cs.a * fa * blended_b + cb.a * fb * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * fa + cb.a * fb;

      if a0 == 0. {
        return Self::from_color(C::ZERO);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_color(C::new(r, g, b, a0))
    }
  }

  /// General composite function for all color components in combination.
  /// <https://www.w3.org/TR/compositing-1/#blendingnonseparable>
  fn composite_non_separable<B: Blend, F, Op: CompositeOperator>(
    f: F,
    op: Op,
  ) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn((f32, f32, f32), (f32, f32, f32)) -> (f32, f32, f32),
  {
    move |backdrop, src| -> Self {
      let cb = backdrop.to_color();
      let cs = src.to_color();

      if cs.a == 0. && cb.a == 0. {
        return Self::from_color(C::ZERO);
      }

      // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
      // https://www.w3.org/TR/compositing-1/#blending
      let (b_r, b_g, b_b) = f((cb.r, cb.g, cb.b), (cs.r, cs.g, cs.b));
      let blended_r = (1. - cb.a) * cs.r + cb.a * b_r;
      let blended_g = (1. - cb.a) * cs.g + cb.a * b_g;
      let blended_b = (1. - cb.a) * cs.b + cb.a * b_b;

      // Composite: Co = αs x Fa x Cr + αb x Fb x Cb
      // https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators
      let (fa, fb) = op.fractions(cs.a, cb.a);
      let pm_r = cs.a * fa * blended_r + cb.a * fb * cb.r;
      let pm_g = cs.a * fa * blended_g + cb.a * fb * cb.g;
      let pm_b = cs.a * fa * blended_b + cb.a * fb * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * fa + cb.a * fb;

      if a0 == 0. {
        return Self::from_color(C::ZERO);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_color(C::new(r, g, b, a0))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
  }

  impl Rgba {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
      Self { r, g, b, a }
    }
  }

  impl Blend for Rgba {
    fn from_color(c: C) -> Self {
      Rgba {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
      }
    }

    fn to_color(&self) -> C {
      C::new(self.r, self.g, self.b, self.a)
    }
  }

  fn assert_rgba(actual: Rgba, expected: Rgba) {
    let eps = 1e-4;

    assert!(
      (actual.r - expected.r).abs() < eps
        && (actual.g - expected.g).abs() < eps
        && (actual.b - expected.b).abs() < eps
        && (actual.a - expected.a).abs() < eps,
      "expected: {:?}, actual: {:?}",
      expected,
      actual
    );
  }

  #[test]
  fn test_transparent_over_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.);
    let fg = Rgba::new(0., 0., 1., 0.);

    let results = vec![
      fg.normal(&bg),
      fg.multiply(&bg),
      fg.screen(&bg),
      fg.overlay(&bg),
      fg.darken(&bg),
      fg.lighten(&bg),
      fg.color_dodge(&bg),
      fg.color_burn(&bg),
      fg.hard_light(&bg),
      fg.soft_light(&bg),
      fg.difference(&bg),
      fg.exclusion(&bg),
      fg.hue(&bg),
      fg.saturation(&bg),
      fg.color(&bg),
      fg.luminosity(&bg),
    ];

    for result in results {
      assert_rgba(result, Rgba::new(0., 0., 0., 0.));
    }
  }

  #[test]
  fn test_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.);

    let results = vec![
      fg.normal(&bg),
      fg.multiply(&bg),
      fg.screen(&bg),
      fg.overlay(&bg),
      fg.darken(&bg),
      fg.lighten(&bg),
      fg.color_dodge(&bg),
      fg.color_burn(&bg),
      fg.hard_light(&bg),
      fg.soft_light(&bg),
      fg.difference(&bg),
      fg.exclusion(&bg),
      fg.hue(&bg),
      fg.saturation(&bg),
      fg.color(&bg),
      fg.luminosity(&bg),
    ];

    for result in results {
      assert_rgba(result, Rgba::new(1., 0., 0., 1.));
    }
  }

  #[test]
  fn test_normal_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 1.) + 1. x 0. x (1., 0., 0.) = (0., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. x 0. = 1.
    assert_rgba(fg.normal(&bg), Rgba::new(0. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_normal_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 0.5 + 0.5 = 1.0
    assert_rgba(fg.normal(&bg), Rgba::new(0.5 / 1., 0. / 1., 0.5 / 1., 1.));
  }

  #[test]
  fn test_normal_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 1.) = (0., 0., 0.5) + (0., 0., 0.5) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.25, 0., 0.) = (0.25, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.normal(&bg),
      Rgba::new(0.25 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_multiply_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 0.) + 1. x 0. x (1., 0., 0.) = (0., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.multiply(&bg), Rgba::new(0. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_multiply_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0., 0., 0.) + (0.5, 0., 0.) = (0.5, 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.multiply(&bg), Rgba::new(0.5 / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_multiply_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 0.) = (0., 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.25) + (0.25, 0., 0.) = (0.25, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.multiply(&bg),
      Rgba::new(0.25 / 0.75, 0., 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_screen_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.screen(&bg), Rgba::new(1. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_screen_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.screen(&bg), Rgba::new(1. / 1., 0. / 1., 0.5 / 1., 1.));
  }

  #[test]
  fn test_screen_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.screen(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_overlay_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // overlay: B(Cb, Cs) = HardLight(Cs, Cb)
    //                    = if(Cb <= 0.5)
    //                        B(Cs, Cb) = Multiply(Cs, 2 x Cb) = Cs x 2 x Cb
    //                      else
    //                        B(Cs, Cb) = Screen(Cs, 2 x Cb - 1) = Cs + (2 x Cb - 1) - (Cs x (2 x Cb - 1))
    //                    = r: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //                    = g: 0. x 2. x 0. = 0.
    //                    = b: 1. x 2. x 0. = 0.
    //                    = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 0.) + 1. x 0. x (1., 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.overlay(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_overlay_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // overlay: B(Cb, Cs) = HardLight(Cs, Cb)
    //                    = if(Cb <= 0.5)
    //                        B(Cs, Cb) = Multiply(Cs, 2 x Cb) = Cs x 2 x Cb
    //                      else
    //                        B(Cs, Cb) = Screen(Cs, 2 x Cb - 1) = Cs + (2 x Cb - 1) - (Cs x (2 x Cb - 1))
    //                    = r: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //                    = g: 0. x 2. x 0. = 0.
    //                    = b: 1. x 2. x 0. = 0.
    //                    = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.) + (0.5, 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.overlay(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_overlay_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // overlay: B(Cb, Cs) = HardLight(Cs, Cb)
    //                    = if(Cb <= 0.5)
    //                        B(Cs, Cb) = Multiply(Cs, 2 x Cb) = Cs x 2 x Cb
    //                      else
    //                        B(Cs, Cb) = Screen(Cs, 2 x Cb - 1) = Cs + (2 x Cb - 1) - (Cs x (2 x Cb - 1))
    //                    = r: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //                    = g: 0. x 2. x 0. = 0.
    //                    = b: 1. x 2. x 0. = 0.
    //                    = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.25) + (0.25, 0., 0.) = (0.5, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.overlay(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_darken_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 0.) + 1. x 0. x (1., 0., 0.) = (0., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.darken(&bg), Rgba::new(0. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_darken_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.darken(&bg), Rgba::new(0.5 / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_darken_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 0.) = (0., 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.25) + (0.25, 0., 0.) = (0.25, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.darken(&bg),
      Rgba::new(0.25 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_lighten_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.lighten(&bg), Rgba::new(1. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_lighten_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.lighten(&bg), Rgba::new(1. / 1., 0. / 1., 0.5 / 1., 1.));
  }

  #[test]
  fn test_lighten_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.lighten(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_dodge_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // color_dodge: if(Cb == 0)
    //                B(Cb, Cs) = 0
    //              else if(Cs == 1)
    //                B(Cb, Cs) = 1
    //              else
    //                B(Cb, Cs) = min(1, Cb / (1 - Cs))
    //              = r: min(1., 1. / (1. - 0.)) = 1.
    //              = g: 0.
    //              = b: 0.
    //              = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. (1., 0., 0.) + 1. x 0. x (1., 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.color_dodge(&bg),
      Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_dodge_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color_dodge: if(Cb == 0)
    //                B(Cb, Cs) = 0
    //              else if(Cs == 1)
    //                B(Cb, Cs) = 1
    //              else
    //                B(Cb, Cs) = min(1, Cb / (1 - Cs))
    //              = r: min(1., 1. / (1. - 0.)) = 1.
    //              = g: 0.
    //              = b: 0.
    //              = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.) + (0.5, 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.color_dodge(&bg),
      Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_dodge_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color_dodge: if(Cb == 0)
    //                B(Cb, Cs) = 0
    //              else if(Cs == 1)
    //                B(Cb, Cs) = 1
    //              else
    //                B(Cb, Cs) = min(1, Cb / (1 - Cs))
    //              = r: min(1., 1. / (1. - 0.)) = 1.
    //              = g: 0.
    //              = b: 0.
    //              = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.25) + (0.25, 0., 0.) = (0.5, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.color_dodge(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_burn_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // color_burn: if(Cb == 1)
    //               B(Cb, Cs) = 1
    //             else if(Cs == 0)
    //               B(Cb, Cs) = 0
    //             else
    //               B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
    //             = r: 1.
    //             = g: 0.
    //             = b: 1. - min(1., (1. - 0.) / 1.) = 1. - 1. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 0.) + 1. x 0. x (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.color_burn(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_color_burn_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color_burn: if(Cb == 1)
    //               B(Cb, Cs) = 1
    //             else if(Cs == 0)
    //               B(Cb, Cs) = 0
    //             else
    //               B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
    //             = r: 1.
    //             = g: 0.
    //             = b: 1. - min(1., (1. - 0.) / 1.) = 1. - 1. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.) + (0.5, 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.color_burn(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_color_burn_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color_burn: if(Cb == 1)
    //               B(Cb, Cs) = 1
    //             else if(Cs == 0)
    //               B(Cb, Cs) = 0
    //             else
    //               B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
    //             = r: 1.
    //             = g: 0.
    //             = b: 1. - min(1., (1. - 0.) / 1.) = 1. - 1. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.25) + (0.25, 0., 0.) = (0.5, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.color_burn(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_hard_light_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // hard_light: if(Cs <= 0.5)
    //               B(Cb, Cs) = Multiply(Cb, 2 x Cs) = Cb x (2 x Cs)
    //             else
    //               B(Cb, Cs) = Screen(Cb, 2 x Cs -1) = Cb + (2 x Cs - 1) - (Cb x (2 x Cs - 1))
    //             = r: 1. x (2. x 0.) = 0.
    //             = g: 0. x (2. x 0.) = 0.
    //             = b: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //             = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 1.) + 1. x 0. x (1., 0., 0.) = (0., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.hard_light(&bg), Rgba::new(0. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_hard_light_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // hard_light: if(Cs <= 0.5)
    //               B(Cb, Cs) = Multiply(Cb, 2 x Cs) = Cb x (2 x Cs)
    //             else
    //               B(Cb, Cs) = Screen(Cb, 2 x Cs -1) = Cb + (2 x Cs - 1) - (Cb x (2 x Cs - 1))
    //             = r: 1. x (2. x 0.) = 0.
    //             = g: 0. x (2. x 0.) = 0.
    //             = b: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //             = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.hard_light(&bg),
      Rgba::new(0.5 / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_hard_light_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // hard_light: if(Cs <= 0.5)
    //               B(Cb, Cs) = Multiply(Cb, 2 x Cs) = Cb x (2 x Cs)
    //             else
    //               B(Cb, Cs) = Screen(Cb, 2 x Cs -1) = Cb + (2 x Cs - 1) - (Cb x (2 x Cs - 1))
    //             = r: 1. x (2. x 0.) = 0.
    //             = g: 0. x (2. x 0.) = 0.
    //             = b: 0. + (2. x 1. - 1.) - (0. x (2. x 1. - 1.)) = 0. + 1. - 0. = 1.
    //             = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 1.) = (0., 0., 0.5) + (0., 0., 0.5) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.25, 0., 0.) = (0.25, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.hard_light(&bg),
      Rgba::new(0.25 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_soft_light_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // soft_light: B(Cb, Cs) = if(Cs <= 0.5)
    //                 B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
    //               else
    //                 B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
    //             with
    //               if(Cb <= 0.25)
    //                 D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
    //               else
    //                 D(Cb) = sqrt(Cb)
    //             = r: 1. - (1. - 2. x 0.) x 1. x (1. - 1.) = 1. - 1. x 1. x 0. = 1.
    //             = g: 0. - (1. - 2. x 0.) x 0. x (1. - 0.) = 0. - 1. x 0. x 0. = 0.
    //             = b: 0. + (2. x 1. - 1.) x (((16. x 0. - 12.) x 0. + 4.) x 0. - 0.) = 0. + 1. x 0. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 0.) + 1. x 0. x (1., 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.soft_light(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_soft_light_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // soft_light:   if(Cs <= 0.5)
    //                 B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
    //               else
    //                 B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
    //             with
    //               if(Cb <= 0.25)
    //                 D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
    //               else
    //                 D(Cb) = sqrt(Cb)
    //             = r: 1. - (1. - 2. x 0.) x 1. x (1. - 1.) = 1. - 1. x 1. x 0. = 1.
    //             = g: 0. - (1. - 2. x 0.) x 0. x (1. - 0.) = 0. - 1. x 0. x 0. = 0.
    //             = b: 0. + (2. x 1. - 1.) x (((16. x 0. - 12.) x 0. + 4.) x 0. - 0.) = 0. + 1. x 0. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.) + (0.5, 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.soft_light(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_soft_light_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // soft_light:   if(Cs <= 0.5)
    //                 B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
    //               else
    //                 B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
    //             with
    //               if(Cb <= 0.25)
    //                 D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
    //               else
    //                 D(Cb) = sqrt(Cb)
    //             = r: 1. - (1. - 2. x 0.) x 1. x (1. - 1.) = 1. - 1. x 1. x 0. = 1.
    //             = g: 0. - (1. - 2. x 0.) x 0. x (1. - 0.) = 0. - 1. x 0. x 0. = 0.
    //             = b: 0. + (2. x 1. - 1.) x (((16. x 0. - 12.) x 0. + 4.) x 0. - 0.) = 0. + 1. x 0. = 0.
    //             = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 0.) = (0, 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.25) + (0.25, 0., 0.) = (0.5, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.soft_light(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_difference_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.difference(&bg), Rgba::new(1. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_difference_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.difference(&bg),
      Rgba::new(1. / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_difference_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.difference(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_exclusion_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.exclusion(&bg), Rgba::new(1. / 1., 0. / 1., 1. / 1., 1.));
  }

  #[test]
  fn test_exclusion_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.exclusion(&bg), Rgba::new(1. / 1., 0. / 1., 0.5 / 1., 1.));
  }

  #[test]
  fn test_exclusion_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.exclusion(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_hue_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // hue: B(Cb, Cs) = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(Cbred, Cbgreen, Cbblue) - min(Cbred, Cbgreen, Cbblue))), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(1., 0., 0.) - min(1., 0., 0.))), Lum(Cb))
    //                = SetLum(SetSat(Cs, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                = SetLum(SetSat(Cs, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                = SetLum(SetSat(Cs, 1.), 0.3)
    //                = SetLum(SetSat((0., 0., 1.), 1.), 0.3)
    //                = SetLum((0., 0., 1.), 0.3)
    //                ※ d = l - Lum(C) = 0.3 - Lum((0., 0., 1.)) = 0.3 - (0.3 x 0. + 0.59 x 0. + 0.11 x 1.) = 0.3 - 0.11 = 0.19
    //                = ClipColor((0.19, 0.19, 1.19))
    //                ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                = L + (((C - L) × (1 - L)) / (x - L))
    //                = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) + 1. x 0. x (1., 0., 0.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.hue(&bg),
      Rgba::new(0.19 / 0.89 / 1., 0.19 / 0.89 / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_hue_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // hue: B(Cb, Cs) = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(Cbred, Cbgreen, Cbblue) - min(Cbred, Cbgreen, Cbblue))), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(1., 0., 0.) - min(1., 0., 0.))), Lum(Cb))
    //                = SetLum(SetSat(Cs, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                = SetLum(SetSat(Cs, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                = SetLum(SetSat(Cs, 1.), 0.3)
    //                = SetLum(SetSat((0., 0., 1.), 1.), 0.3)
    //                = SetLum((0., 0., 1.), 0.3)
    //                ※ d = l - Lum(C) = 0.3 - Lum((0., 0., 1.)) = 0.3 - (0.3 x 0. + 0.59 x 0. + 0.11 x 1.) = 0.3 - 0.11 = 0.19
    //                = ClipColor((0.19, 0.19, 1.19))
    //                ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                = L + (((C - L) × (1 - L)) / (x - L))
    //                = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) + 1. x 0.5 x (1., 0., 0.) = (0.095 / 0.89, 0.095 / 0.89, 0.5) + (0.5, 0., 0.) = (0.54 / 0.89, 0.095 / 0.89, 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.hue(&bg),
      Rgba::new(0.54 / 0.89 / 1., 0.095 / 0.89 / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_hue_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // hue: B(Cb, Cs) = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(Cbred, Cbgreen, Cbblue) - min(Cbred, Cbgreen, Cbblue))), Lum(Cb))
    //                = SetLum(SetSat(Cs, (max(1., 0., 0.) - min(1., 0., 0.))), Lum(Cb))
    //                = SetLum(SetSat(Cs, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                = SetLum(SetSat(Cs, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                = SetLum(SetSat(Cs, 1.), 0.3)
    //                = SetLum(SetSat((0., 0., 1.), 1.), 0.3)
    //                = SetLum((0., 0., 1.), 0.3)
    //                ※ d = l - Lum(C) = 0.3 - Lum((0., 0., 1.)) = 0.3 - (0.3 x 0. + 0.59 x 0. + 0.11 x 1.) = 0.3 - 0.11 = 0.19
    //                = ClipColor((0.19, 0.19, 1.19))
    //                ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                = L + (((C - L) × (1 - L)) / (x - L))
    //                = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0., 0., 0.5) + (0.095 / 0.89, 0.095 / 0.89, 0.5) = (0.095 / 0.89, 0.095 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.095 / 0.89, 0.095 / 0.89, 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.0475 / 0.89, 0.0475 / 0.89, 0.5) + (0.25, 0., 0.) = (0.27 / 0.89, 0.0475 / 0.89, 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.hue(&bg),
      Rgba::new(0.27 / 0.89 / 0.75, 0.0475 / 0.89 / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_saturation_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // saturation: B(Cb, Cs) = SetLum(SetSat(Cb, Sat(Cs)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(Csred, Csgreen, Csblue) - min(Csred, Csgreen, Csblue)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(0., 0., 1.) - min(0., 0., 1.)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                       = SetLum(SetSat(Cb, 1.), 0.3)
    //                       = SetLum(SetSat((1., 0., 0.), 1.), 0.3)
    //                       = SetLum((1., 0., 0.), 0.3)
    //                       ※ d = l - Lum(C) = 0.3 - Lum((1., 0., 0.)) = 0.3 - (0.3 x 1. + 0.59 x 0. + 0.11 x 0.) = 0.3 - 0.3 = 0.
    //                       = ClipColor((1., 0., 0.))
    //                       L = Lum(C) = Lum((1., 0., 0.)) = 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.3
    //                       n = min(Cred, Cgreen, Cblue) = 0.
    //                       x = max(Cred, Cgreen, Cblue) = 1.
    //                       = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.) = (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 0.) + 1. x 0. x (1., 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(fg.saturation(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_saturation_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // saturation: B(Cb, Cs) = SetLum(SetSat(Cb, Sat(Cs)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(Csred, Csgreen, Csblue) - min(Csred, Csgreen, Csblue)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(0., 0., 1.) - min(0., 0., 1.)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                       = SetLum(SetSat(Cb, 1.), 0.3)
    //                       = SetLum(SetSat((1., 0., 0.), 1.), 0.3)
    //                       = SetLum((1., 0., 0.), 0.3)
    //                       ※ d = l - Lum(C) = 0.3 - Lum((1., 0., 0.)) = 0.3 - (0.3 x 1. + 0.59 x 0. + 0.11 x 0.) = 0.3 - 0.3 = 0.
    //                       = ClipColor((1., 0., 0.))
    //                       L = Lum(C) = Lum((1., 0., 0.)) = 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.3
    //                       n = min(Cred, Cgreen, Cblue) = 0.
    //                       x = max(Cred, Cgreen, Cblue) = 1.
    //                       = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.) + (0.5, 0., 0.) = (1., 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(fg.saturation(&bg), Rgba::new(1. / 1., 0. / 1., 0. / 1., 1.));
  }

  #[test]
  fn test_saturation_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // saturation: B(Cb, Cs) = SetLum(SetSat(Cb, Sat(Cs)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(Csred, Csgreen, Csblue) - min(Csred, Csgreen, Csblue)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, max(0., 0., 1.) - min(0., 0., 1.)), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), Lum(Cb))
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                       = SetLum(SetSat(Cb, 1.), 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                       = SetLum(SetSat(Cb, 1.), 0.3)
    //                       = SetLum(SetSat((1., 0., 0.), 1.), 0.3)
    //                       = SetLum((1., 0., 0.), 0.3)
    //                       ※ d = l - Lum(C) = 0.3 - Lum((1., 0., 0.)) = 0.3 - (0.3 x 1. + 0.59 x 0. + 0.11 x 0.) = 0.3 - 0.3 = 0.
    //                       = ClipColor((1., 0., 0.))
    //                       L = Lum(C) = Lum((1., 0., 0.)) = 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.3
    //                       n = min(Cred, Cgreen, Cblue) = 0.
    //                       x = max(Cred, Cgreen, Cblue) = 1.
    //                       = (1., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.25) + (0.25, 0., 0.) = (0.5, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.saturation(&bg),
      Rgba::new(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // color: B(Cb, Cs) = SetLum(Cs, Lum(Cb))
    //                  = SetLum(Cs, 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                  = SetLum(Cs, 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                  = SetLum(Cs, 0.3)
    //                  = SetLum((0., 0., 1.), 0.3)
    //                  ※ d = l - Lum(C) = 0.3 -0.3 x 0. + 0.59 x 0. + 0.11 x 1. = 0.3 - 0.11 = 0.19
    //                  = ClipColor((0.19, 0.19, 1.19))
    //                  ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                  ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                  ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                  = L + (((C - L) × (1 - L)) / (x - L))
    //                  = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                  = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                  = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                  = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) + 1. x 0. x (1., 0., 0.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.color(&bg),
      Rgba::new(0.19 / 0.89 / 1., 0.19 / 0.89 / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_color_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color: B(Cb, Cs) = SetLum(Cs, Lum(Cb))
    //                  = SetLum(Cs, 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                  = SetLum(Cs, 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                  = SetLum(Cs, 0.3)
    //                  = SetLum((0., 0., 1.), 0.3)
    //                  ※ d = l - Lum(C) = 0.3 -0.3 x 0. + 0.59 x 0. + 0.11 x 1. = 0.3 - 0.11 = 0.19
    //                  = ClipColor((0.19, 0.19, 1.19))
    //                  ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                  ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                  ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                  = L + (((C - L) × (1 - L)) / (x - L))
    //                  = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                  = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                  = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                  = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.19 / 0.89, 0.19 / 0.89, 1.) + 1. x 0.5 x (1., 0., 0.) = (0.095 / 0.89, 0.095 / 0.89, 0.5) + (0.5, 0., 0.) = (0.54 / 0.89, 0.095 / 0.89, 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.color(&bg),
      Rgba::new(0.54 / 0.89 / 1., 0.095 / 0.89 / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_color_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // color: B(Cb, Cs) = SetLum(Cs, Lum(Cb))
    //                  = SetLum(Cs, 0.3 x Cbred + 0.59 x Cbgreen + 0.11 x Cbblue)
    //                  = SetLum(Cs, 0.3 x 1. + 0.59 x 0. + 0.11 x 0.)
    //                  = SetLum(Cs, 0.3)
    //                  = SetLum((0., 0., 1.), 0.3)
    //                  ※ d = l - Lum(C) = 0.3 -0.3 x 0. + 0.59 x 0. + 0.11 x 1. = 0.3 - 0.11 = 0.19
    //                  = ClipColor((0.19, 0.19, 1.19))
    //                  ※ L = Lum(C) = Lum((0.19, 0.19, 1.19)) = 0.3 x 0.19 + 0.59 x 0.19 + 0.11 x 1.19 = 0.3
    //                  ※ n = min(Cred, Cgreen, Cblue) = min(0.19, 0.19, 1.19) = 0.19
    //                  ※ x = max(Cred, Cgreen, Cblue) = max(0.19, 0.19, 1.19) = 1.19
    //                  = L + (((C - L) × (1 - L)) / (x - L))
    //                  = 0.3 + ((((0.19, 0.19, 1.19) - 0.3) x (1 - 0.3)) / (1.19 - 0.3))
    //                  = 0.3 + (((-0.11, -0.11, 0.89) x 0.7) / 0.89)
    //                  = 0.3 + ((-0.077, -0.077, 0.623) / 0.89)
    //                  = 0.3 + (-0.077 / 0.89, -0.077 / 0.89, 0.623 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 0.89 / 0.89)
    //                  = (0.19 / 0.89, 0.19 / 0.89, 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0.19 / 0.89, 0.19 / 0.89, 1.) = (0., 0., 0.5) + (0.095 / 0.89, 0.095 / 0.89, 0.5) = (0.095 / 0.89, 0.095 / 0.89, 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.095 / 0.89, 0.095 / 0.89, 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.0475 / 0.89, 0.0475 / 0.89, 0.5) + (0.25, 0., 0.) = (0.27 / 0.89, 0.0475 / 0.89, 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.color(&bg),
      Rgba::new(0.27 / 0.89 / 0.75, 0.0475 / 0.89 / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_luminosity_opaque_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // luminosity: B(Cb, Cs) = SetLum(Cb, Lum(Cs))
    //                       = SetLum(Cb, 0.3 x Csred + 0.59 x Csgreen + 0.11 x Csblue)
    //                       = SetLum(Cb, 0.3 x 0. + 0.59 x 0. + 0.11 x 1.)
    //                       = SetLum(Cb, 0.11)
    //                       = SetLum((1., 0., 0.), 0.11)
    //                       ※ d = l - Lum(C) = 0.11 - 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.11 - 0.3 = -0.19
    //                       = ClipColor((0.81, -0.19, -0.19))
    //                       ※ L = Lum(C) = 0.3 x 0.81 + 0.59 x -0.19 + 0.11 x -0.19 = 0.11
    //                       ※ n = min(Cred, Cgreen, Cblue) = -0.19
    //                       ※ x = max(Cred, Cgreen, Cblue) = 0.81
    //                       = L + (((C - L) × L) / (L - n))
    //                       = 0.11 + ((((0.81, -0.19, -0.19) - 0.11) x 0.11) / (0.11 - (-0.19)))
    //                       = 0.11 + (((0.7, -0.3, -0.3) x 0.11) / 0.3)
    //                       = 0.11 + ((0.077, -0.033, -0.033) / 0.3)
    //                       = 0.11 + (0.077 / 0.3, -0.11, -0.11)
    //                       = (0.11 / 0.3, 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.11 / 0.3, 0., 0.) = (0.11 / 0.3, 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0.11 / 0.3, 0., 0.) + 1. x 0. x (1., 0., 0.) = (0.11 / 0.3, 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.luminosity(&bg),
      Rgba::new(0.11 / 0.3 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_luminosity_semi_transparent_over_opaque() {
    let bg = Rgba::new(1., 0., 0., 1.);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // luminosity: B(Cb, Cs) = SetLum(Cb, Lum(Cs))
    //                       = SetLum(Cb, 0.3 x Csred + 0.59 x Csgreen + 0.11 x Csblue)
    //                       = SetLum(Cb, 0.3 x 0. + 0.59 x 0. + 0.11 x 1.)
    //                       = SetLum(Cb, 0.11)
    //                       = SetLum((1., 0., 0.), 0.11)
    //                       ※ d = l - Lum(C) = 0.11 - 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.11 - 0.3 = -0.19
    //                       = ClipColor((0.81, -0.19, -0.19))
    //                       ※ L = Lum(C) = 0.3 x 0.81 + 0.59 x -0.19 + 0.11 x -0.19 = 0.11
    //                       ※ n = min(Cred, Cgreen, Cblue) = -0.19
    //                       ※ x = max(Cred, Cgreen, Cblue) = 0.81
    //                       = L + (((C - L) × L) / (L - n))
    //                       = 0.11 + ((((0.81, -0.19, -0.19) - 0.11) x 0.11) / (0.11 - (-0.19)))
    //                       = 0.11 + (((0.7, -0.3, -0.3) x 0.11) / 0.3)
    //                       = 0.11 + ((0.077, -0.033, -0.033) / 0.3)
    //                       = 0.11 + (0.077 / 0.3, -0.11, -0.11)
    //                       = (0.11 / 0.3, 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0.11 / 0.3, 0., 0.) = (0.11 / 0.3, 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.11 / 0.3, 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.055 / 0.3, 0., 0.) + (0.5, 0., 0.) = (0.205 / 0.3, 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.luminosity(&bg),
      Rgba::new(0.205 / 0.3 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_luminosity_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1., 0., 0., 0.5);
    let fg = Rgba::new(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // luminosity: B(Cb, Cs) = SetLum(Cb, Lum(Cs))
    //                       = SetLum(Cb, 0.3 x Csred + 0.59 x Csgreen + 0.11 x Csblue)
    //                       = SetLum(Cb, 0.3 x 0. + 0.59 x 0. + 0.11 x 1.)
    //                       = SetLum(Cb, 0.11)
    //                       = SetLum((1., 0., 0.), 0.11)
    //                       ※ d = l - Lum(C) = 0.11 - 0.3 x 1. + 0.59 x 0. + 0.11 x 0. = 0.11 - 0.3 = -0.19
    //                       = ClipColor((0.81, -0.19, -0.19))
    //                       ※ L = Lum(C) = 0.3 x 0.81 + 0.59 x -0.19 + 0.11 x -0.19 = 0.11
    //                       ※ n = min(Cred, Cgreen, Cblue) = -0.19
    //                       ※ x = max(Cred, Cgreen, Cblue) = 0.81
    //                       = L + (((C - L) × L) / (L - n))
    //                       = 0.11 + ((((0.81, -0.19, -0.19) - 0.11) x 0.11) / (0.11 - (-0.19)))
    //                       = 0.11 + (((0.7, -0.3, -0.3) x 0.11) / 0.3)
    //                       = 0.11 + ((0.077, -0.033, -0.033) / 0.3)
    //                       = 0.11 + (0.077 / 0.3, -0.11, -0.11)
    //                       = (0.11 / 0.3, 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0.11 / 0.3, 0., 0.) = (0., 0., 0.5) + (0.055 / 0.3, 0., 0.) = (0.055 / 0.3, 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.055 / 0.3, 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0.0275 / 0.3, 0., 0.25) + (0.25, 0., 0.) = (0.1025 / 0.3, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.luminosity(&bg),
      Rgba::new(0.1025 / 0.3 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }
}
