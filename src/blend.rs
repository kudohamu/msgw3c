use crate::{color::C, formula};

pub trait BlendFormula {
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32);
}

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

impl BlendFormula for BlendMode {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    match self {
      Self::Normal => NormalFormula.apply_k(cb, cs),
      Self::Multiply => MultiplyFormula.apply_k(cb, cs),
      Self::Screen => ScreenFormula.apply_k(cb, cs),
      Self::Overlay => OverlayFormula.apply_k(cb, cs),
      Self::Darken => DarkenFormula.apply_k(cb, cs),
      Self::Lighten => LightenFormula.apply_k(cb, cs),
      Self::ColorDodge => ColorDodgeFormula.apply_k(cb, cs),
      Self::ColorBurn => ColorBurnFormula.apply_k(cb, cs),
      Self::HardLight => HardLightFormula.apply_k(cb, cs),
      Self::SoftLight => SoftLightFormula.apply_k(cb, cs),
      Self::Difference => DifferenceFormula.apply_k(cb, cs),
      Self::Exclusion => ExclusionFormula.apply_k(cb, cs),
      Self::Hue => HueFormula.apply_k(cb, cs),
      Self::Saturation => SaturationFormula.apply_k(cb, cs),
      Self::Color => ColorFormula.apply_k(cb, cs),
      Self::Luminosity => LuminosityFormula.apply_k(cb, cs),
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NormalFormula;

impl BlendFormula for NormalFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::normal(cb.r, cs.r, cb.a, cs.a);
    let g = formula::normal(cb.g, cs.g, cb.a, cs.a);
    let b = formula::normal(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MultiplyFormula;

impl BlendFormula for MultiplyFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::multiply(cb.r, cs.r, cb.a, cs.a);
    let g = formula::multiply(cb.g, cs.g, cb.a, cs.a);
    let b = formula::multiply(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScreenFormula;

impl BlendFormula for ScreenFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::screen(cb.r, cs.r, cb.a, cs.a);
    let g = formula::screen(cb.g, cs.g, cb.a, cs.a);
    let b = formula::screen(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OverlayFormula;

impl BlendFormula for OverlayFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::overlay(cb.r, cs.r, cb.a, cs.a);
    let g = formula::overlay(cb.g, cs.g, cb.a, cs.a);
    let b = formula::overlay(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DarkenFormula;

impl BlendFormula for DarkenFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::darken(cb.r, cs.r, cb.a, cs.a);
    let g = formula::darken(cb.g, cs.g, cb.a, cs.a);
    let b = formula::darken(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LightenFormula;

impl BlendFormula for LightenFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::lighten(cb.r, cs.r, cb.a, cs.a);
    let g = formula::lighten(cb.g, cs.g, cb.a, cs.a);
    let b = formula::lighten(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ColorDodgeFormula;

impl BlendFormula for ColorDodgeFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::color_dodge(cb.r, cs.r, cb.a, cs.a);
    let g = formula::color_dodge(cb.g, cs.g, cb.a, cs.a);
    let b = formula::color_dodge(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ColorBurnFormula;

impl BlendFormula for ColorBurnFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::color_burn(cb.r, cs.r, cb.a, cs.a);
    let g = formula::color_burn(cb.g, cs.g, cb.a, cs.a);
    let b = formula::color_burn(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HardLightFormula;

impl BlendFormula for HardLightFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::hard_light(cb.r, cs.r, cb.a, cs.a);
    let g = formula::hard_light(cb.g, cs.g, cb.a, cs.a);
    let b = formula::hard_light(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SoftLightFormula;

impl BlendFormula for SoftLightFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::soft_light(cb.r, cs.r, cb.a, cs.a);
    let g = formula::soft_light(cb.g, cs.g, cb.a, cs.a);
    let b = formula::soft_light(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DifferenceFormula;

impl BlendFormula for DifferenceFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::difference(cb.r, cs.r, cb.a, cs.a);
    let g = formula::difference(cb.g, cs.g, cb.a, cs.a);
    let b = formula::difference(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExclusionFormula;

impl BlendFormula for ExclusionFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let r = formula::exclusion(cb.r, cs.r, cb.a, cs.a);
    let g = formula::exclusion(cb.g, cs.g, cb.a, cs.a);
    let b = formula::exclusion(cb.b, cs.b, cb.a, cs.a);

    (r, g, b)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HueFormula;

impl BlendFormula for HueFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let s_cb = cb.to_straight_alpha();
    let s_cs = cs.to_straight_alpha();

    formula::hue(
      (s_cb.r, s_cb.g, s_cb.b),
      (s_cs.r, s_cs.g, s_cs.b),
      cb.a,
      cs.a,
    )
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SaturationFormula;

impl BlendFormula for SaturationFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let s_cb = cb.to_straight_alpha();
    let s_cs = cs.to_straight_alpha();

    formula::saturation(
      (s_cb.r, s_cb.g, s_cb.b),
      (s_cs.r, s_cs.g, s_cs.b),
      cb.a,
      cs.a,
    )
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ColorFormula;

impl BlendFormula for ColorFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let s_cb = cb.to_straight_alpha();
    let s_cs = cs.to_straight_alpha();

    formula::color(
      (s_cb.r, s_cb.g, s_cb.b),
      (s_cs.r, s_cs.g, s_cs.b),
      cb.a,
      cs.a,
    )
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LuminosityFormula;

impl BlendFormula for LuminosityFormula {
  #[inline]
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    let s_cb = cb.to_straight_alpha();
    let s_cs = cs.to_straight_alpha();

    formula::luminosity(
      (s_cb.r, s_cb.g, s_cb.b),
      (s_cs.r, s_cs.g, s_cs.b),
      cb.a,
      cs.a,
    )
  }
}
