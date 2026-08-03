use crate::{
  color::C,
  formula,
  porter_duff::{CompositeOperator, PorterDuff},
};

/// Blend modes defined in the following W3C specification.
/// https://drafts.csswg.org/compositing-1/#blending
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
  fn from_color(c: C) -> Self;
  fn to_color(&self) -> C;

  /// The utility function for `normal` blend. It is composited using `SourceOver`.
  /// Use `normal_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn normal(&self, backdrop: &impl Blend) -> Self {
    self.normal_with(backdrop, PorterDuff::SourceOver)
  }

  /// `normal` blend function that allows color blending using the Porter-Duff composite operator.
  fn normal_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Normal, op)
  }

  /// The utility function for `multiply` blend. It is composited using `SourceOver`.
  /// Use `multiply_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn multiply(&self, backdrop: &impl Blend) -> Self {
    self.multiply_with(backdrop, PorterDuff::SourceOver)
  }

  /// `multiply` blend function that allows color blending using the Porter-Duff composite operator.
  fn multiply_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Multiply, op)
  }

  /// The utility function for `screen` blend. It is composited using `SourceOver`.
  /// Use `screen_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn screen(&self, backdrop: &impl Blend) -> Self {
    self.screen_with(backdrop, PorterDuff::SourceOver)
  }

  /// `screen` blend function that allows color blending using the Porter-Duff composite operator.
  fn screen_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Screen, op)
  }

  /// The utility function for `overlay` blend. It is composited using `SourceOver`.
  /// Use `overlay_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn overlay(&self, backdrop: &impl Blend) -> Self {
    self.overlay_with(backdrop, PorterDuff::SourceOver)
  }

  /// `overlay` blend function that allows color blending using the Porter-Duff composite operator.
  fn overlay_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Overlay, op)
  }

  /// The utility function for `darken` blend. It is composited using `SourceOver`.
  /// Use `darken_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn darken(&self, backdrop: &impl Blend) -> Self {
    self.darken_with(backdrop, PorterDuff::SourceOver)
  }

  /// `darken` blend function that allows color blending using the Porter-Duff composite operator.
  fn darken_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Darken, op)
  }

  /// The utility function for `lighten` blend. It is composited using `SourceOver`.
  /// Use `lighten_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn lighten(&self, backdrop: &impl Blend) -> Self {
    self.lighten_with(backdrop, PorterDuff::SourceOver)
  }

  /// `lighten` blend function that allows color blending using the Porter-Duff composite operator.
  fn lighten_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Lighten, op)
  }

  /// The utility function for `color_dodge` blend. It is composited using `SourceOver`.
  /// Use `color_dodge_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn color_dodge(&self, backdrop: &impl Blend) -> Self {
    self.color_dodge_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_dodge` blend function that allows color blending using the Porter-Duff composite operator.
  fn color_dodge_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorDodge, op)
  }

  /// The utility function for `color_burn` blend. It is composited using `SourceOver`.
  /// Use `color_burn_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn color_burn(&self, backdrop: &impl Blend) -> Self {
    self.color_burn_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_burn` blend function that allows color blending using the Porter-Duff composite operator.
  fn color_burn_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorBurn, op)
  }

  /// The utility function for `hard_light` blend. It is composited using `SourceOver`.
  /// Use `hard_light_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn hard_light(&self, backdrop: &impl Blend) -> Self {
    self.hard_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hard_light` blend function that allows color blending using the Porter-Duff composite operator.
  fn hard_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::HardLight, op)
  }

  /// The utility function for `soft_light` blend. It is composited using `SourceOver`.
  /// Use `soft_light_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn soft_light(&self, backdrop: &impl Blend) -> Self {
    self.soft_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `soft_light` blend function that allows color blending using the Porter-Duff composite operator.
  fn soft_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::SoftLight, op)
  }

  /// The utility function for `difference` blend. It is composited using `SourceOver`.
  /// Use `difference_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn difference(&self, backdrop: &impl Blend) -> Self {
    self.difference_with(backdrop, PorterDuff::SourceOver)
  }

  /// `difference` blend function that allows color blending using the Porter-Duff composite operator.
  fn difference_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Difference, op)
  }

  /// The utility function for `exclusion` blend. It is composited using `SourceOver`.
  /// Use `exclusion_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn exclusion(&self, backdrop: &impl Blend) -> Self {
    self.exclusion_with(backdrop, PorterDuff::SourceOver)
  }

  /// `exclusion` blend function that allows color blending using the Porter-Duff composite operator.
  fn exclusion_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Exclusion, op)
  }

  /// The utility function for `hue` blend. It is composited using `SourceOver`.
  /// Use `hue_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn hue(&self, backdrop: &impl Blend) -> Self {
    self.hue_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hue` blend function that allows color blending using the Porter-Duff composite operator.
  fn hue_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Hue, op)
  }

  /// The utility function for `saturation` blend. It is composited using `SourceOver`.
  /// Use `saturation_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn saturation(&self, backdrop: &impl Blend) -> Self {
    self.saturation_with(backdrop, PorterDuff::SourceOver)
  }

  /// `saturation` blend function that allows color blending using the Porter-Duff composite operator.
  fn saturation_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Saturation, op)
  }

  /// The utility function for `color` blend. It is composited using `SourceOver`.
  /// Use `color`, if you want to blend specifying the Porter-Duff composite operator.
  fn color(&self, backdrop: &impl Blend) -> Self {
    self.color_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color` blend function that allows color blending using the Porter-Duff composite operator.
  fn color_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Color, op)
  }

  /// The utility function for `luminosity` blend. It is composited using `SourceOver`.
  /// Use `luminosity_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn luminosity(&self, backdrop: &impl Blend) -> Self {
    self.luminosity_with(backdrop, PorterDuff::SourceOver)
  }

  /// `luminosity` blend function that allows color blending using the Porter-Duff composite operator.
  fn luminosity_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Luminosity, op)
  }

  /// Blend function that allows you to dynamically specify blend mode and composite operator.
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
  /// https://drafts.csswg.org/compositing-1/#blendingseparable
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
      // https://drafts.csswg.org/compositing-1/#blending
      let blended_r = (1. - cb.a) * cs.r + cb.a * f(cb.r, cs.r);
      let blended_g = (1. - cb.a) * cs.g + cb.a * f(cb.g, cs.g);
      let blended_b = (1. - cb.a) * cs.b + cb.a * f(cb.b, cs.b);

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
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
  /// https://drafts.csswg.org/compositing-1/#blendingnonseparable
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
      // https://drafts.csswg.org/compositing-1/#blending
      let (b_r, b_g, b_b) = f((cb.r, cb.g, cb.b), (cs.r, cs.g, cs.b));
      let blended_r = (1. - cb.a) * cs.r + cb.a * b_r;
      let blended_g = (1. - cb.a) * cs.g + cb.a * b_g;
      let blended_b = (1. - cb.a) * cs.b + cb.a * b_b;

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
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
  pub struct Rgba {
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
    let bg = Rgba::new(1.0, 0.0, 0.0, 0.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::new(0.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn test_normal_opaque_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 1.0);

    assert_rgba(fg.normal(&bg), Rgba::new(0.0, 0.0, 1.0, 1.0));
  }

  #[test]
  fn test_normal_transparent_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::new(1.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.5);

    assert_rgba(fg.normal(&bg), Rgba::new(0.5, 0.0, 0.5, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 0.5);
    let fg = Rgba::new(0.0, 1.0, 0.0, 0.5);

    assert_rgba(fg.normal(&bg), Rgba::new(0.33333334, 0.6666667, 0.0, 0.75));
  }
}
