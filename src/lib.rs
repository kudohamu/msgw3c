#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
  nonstandard_style,
  rust_2018_idioms,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
  deprecated_in_future,
  missing_copy_implementations,
  missing_debug_implementations,
  unreachable_pub,
  unused_import_braces,
  unused_labels,
  unused_lifetimes,
  unused_qualifications,
  unused_results
)]

use crate::{
  blend::{
    BlendFormula, BlendMode, ColorBurnFormula, ColorDodgeFormula, ColorFormula, DarkenFormula,
    DifferenceFormula, ExclusionFormula, HardLightFormula, HueFormula, LightenFormula,
    LuminosityFormula, MultiplyFormula, NormalFormula, OverlayFormula, SaturationFormula,
    ScreenFormula, SoftLightFormula,
  },
  color::C,
  composite::{
    CompositeOperator, DestinationAtopOperator, DestinationInOperator, DestinationOutOperator,
    DestinationOverOperator, LighterOperator, PorterDuff, SourceAtopOperator, SourceInOperator,
    SourceOutOperator, SourceOverOperator, XorOperator,
  },
  error::Error,
};
pub mod blend;
pub mod color;
pub mod composite;
pub mod error;
mod formula;

/// A trait that represents blendable types.
/// Implementing this trait enables blending
/// that complies with the W3C specification.
pub trait Blend: Sized + Copy {
  /// Converts the color type [`C`] used internally by the Blend trait to the implemented type
  ///
  /// ```rust
  /// use msgw3c::{
  ///   Blend,
  ///   color::C,
  /// };
  ///
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
  ///   # fn to_color(&self) -> C {
  ///   #   C::new(
  ///   #     self.r as f32 / 255.,
  ///   #     self.g as f32 / 255.,
  ///   #     self.b as f32 / 255.,
  ///   #     self.a as f32 / 255.,
  ///   #  )
  ///   }
  /// }
  /// ```
  fn from_color(c: C) -> Self;
  /// Converts the implemented type to color type [`C`], which the Blend trait uses internally
  ///
  /// ```rust
  /// use msgw3c::{
  ///   Blend,
  ///   color::C,
  /// };
  ///
  /// // Your color type.
  /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// struct Rgba {
  ///   r: u8,
  ///   g: u8,
  ///   b: u8,
  ///   a: u8,
  /// }
  ///
  /// impl Blend for Rgba {
  ///   # fn from_color(color: C) -> Self {
  ///   #   Self {
  ///   #     r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  ///   #     g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  ///   #     b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  ///   #     a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  ///   #   }
  ///   # }
  ///
  ///   fn to_color(&self) -> C {
  ///     C::new(
  ///       self.r as f32 / 255.,
  ///       self.g as f32 / 255.,
  ///       self.b as f32 / 255.,
  ///       self.a as f32 / 255.,
  ///     )
  ///   }
  /// }
  /// ```
  fn to_color(&self) -> C;

  /// The utility function for `normal` blend. It is composited using `SourceOver`.
  /// Use [`Self::normal_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  ///   # fn from_color(color: C) -> Self {
  ///   #   let color = color.to_straight_alpha();
  ///   #
  ///   #   Self {
  ///   #     r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  ///   #     g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  ///   #     b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  ///   #     a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  ///   #   }
  ///   # }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.normal(&backdrop);
  /// ```
  #[inline]
  fn normal(&self, backdrop: &impl Blend) -> Self {
    self.normal_with(backdrop, PorterDuff::SourceOver)
  }

  /// `normal` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.normal_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn normal_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Normal, op)
  }

  /// The utility function for `multiply` blend. It is composited using `SourceOver`.
  /// Use [`Self::multiply_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.multiply(&backdrop);
  /// ```
  #[inline]
  fn multiply(&self, backdrop: &impl Blend) -> Self {
    self.multiply_with(backdrop, PorterDuff::SourceOver)
  }

  /// `multiply` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.multiply_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn multiply_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Multiply, op)
  }

  /// The utility function for `screen` blend. It is composited using `SourceOver`.
  /// Use [`Self::screen_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.screen(&backdrop);
  /// ```
  #[inline]
  fn screen(&self, backdrop: &impl Blend) -> Self {
    self.screen_with(backdrop, PorterDuff::SourceOver)
  }

  /// `screen` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.screen_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn screen_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Screen, op)
  }

  /// The utility function for `overlay` blend. It is composited using `SourceOver`.
  /// Use [`Self::overlay_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.overlay(&backdrop);
  /// ```
  #[inline]
  fn overlay(&self, backdrop: &impl Blend) -> Self {
    self.overlay_with(backdrop, PorterDuff::SourceOver)
  }

  /// `overlay` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.overlay_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn overlay_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Overlay, op)
  }

  /// The utility function for `darken` blend. It is composited using `SourceOver`.
  /// Use [`Self::darken_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.darken(&backdrop);
  /// ```
  #[inline]
  fn darken(&self, backdrop: &impl Blend) -> Self {
    self.darken_with(backdrop, PorterDuff::SourceOver)
  }

  /// `darken` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.darken_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn darken_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Darken, op)
  }

  /// The utility function for `lighten` blend. It is composited using `SourceOver`.
  /// Use [`Self::lighten_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.lighten(&backdrop);
  /// ```
  #[inline]
  fn lighten(&self, backdrop: &impl Blend) -> Self {
    self.lighten_with(backdrop, PorterDuff::SourceOver)
  }

  /// `lighten` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.lighten_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn lighten_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Lighten, op)
  }

  /// The utility function for `color_dodge` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_dodge_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color_dodge(&backdrop);
  /// ```
  #[inline]
  fn color_dodge(&self, backdrop: &impl Blend) -> Self {
    self.color_dodge_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_dodge` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color_dodge_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn color_dodge_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorDodge, op)
  }

  /// The utility function for `color_burn` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_burn_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color_burn(&backdrop);
  /// ```
  #[inline]
  fn color_burn(&self, backdrop: &impl Blend) -> Self {
    self.color_burn_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color_burn` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color_burn_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn color_burn_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::ColorBurn, op)
  }

  /// The utility function for `hard_light` blend. It is composited using `SourceOver`.
  /// Use [`Self::hard_light_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.hard_light(&backdrop);
  /// ```
  #[inline]
  fn hard_light(&self, backdrop: &impl Blend) -> Self {
    self.hard_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hard_light` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.hard_light_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn hard_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::HardLight, op)
  }

  /// The utility function for `soft_light` blend. It is composited using `SourceOver`.
  /// Use [`Self::soft_light_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.soft_light(&backdrop);
  /// ```
  #[inline]
  fn soft_light(&self, backdrop: &impl Blend) -> Self {
    self.soft_light_with(backdrop, PorterDuff::SourceOver)
  }

  /// `soft_light` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.soft_light_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn soft_light_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::SoftLight, op)
  }

  /// The utility function for `difference` blend. It is composited using `SourceOver`.
  /// Use [`Self::difference_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.difference(&backdrop);
  /// ```
  #[inline]
  fn difference(&self, backdrop: &impl Blend) -> Self {
    self.difference_with(backdrop, PorterDuff::SourceOver)
  }

  /// `difference` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.difference_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn difference_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Difference, op)
  }

  /// The utility function for `exclusion` blend. It is composited using `SourceOver`.
  /// Use [`Self::exclusion_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.exclusion(&backdrop);
  /// ```
  #[inline]
  fn exclusion(&self, backdrop: &impl Blend) -> Self {
    self.exclusion_with(backdrop, PorterDuff::SourceOver)
  }

  /// `exclusion` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.exclusion_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn exclusion_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Exclusion, op)
  }

  /// The utility function for `hue` blend. It is composited using `SourceOver`.
  /// Use [`Self::hue_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.hue(&backdrop);
  /// ```
  #[inline]
  fn hue(&self, backdrop: &impl Blend) -> Self {
    self.hue_with(backdrop, PorterDuff::SourceOver)
  }

  /// `hue` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.hue_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn hue_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Hue, op)
  }

  /// The utility function for `saturation` blend. It is composited using `SourceOver`.
  /// Use [`Self::saturation_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.saturation(&backdrop);
  /// ```
  #[inline]
  fn saturation(&self, backdrop: &impl Blend) -> Self {
    self.saturation_with(backdrop, PorterDuff::SourceOver)
  }

  /// `saturation` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.saturation_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn saturation_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Saturation, op)
  }

  /// The utility function for `color` blend. It is composited using `SourceOver`.
  /// Use [`Self::color_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color(&backdrop);
  /// ```
  #[inline]
  fn color(&self, backdrop: &impl Blend) -> Self {
    self.color_with(backdrop, PorterDuff::SourceOver)
  }

  /// `color` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.color_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn color_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Color, op)
  }

  /// The utility function for `luminosity` blend. It is composited using `SourceOver`.
  /// Use [`Self::luminosity_with()`], if you want to blend specifying the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.luminosity(&backdrop);
  /// ```
  #[inline]
  fn luminosity(&self, backdrop: &impl Blend) -> Self {
    self.luminosity_with(backdrop, PorterDuff::SourceOver)
  }

  /// `luminosity` blend function that allows color blending using the Porter-Duff composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.luminosity_with(&backdrop, PorterDuff::Destination);
  /// ```
  #[inline]
  fn luminosity_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    self.blend_with(backdrop, BlendMode::Luminosity, op)
  }

  /// Blend function that allows you to dynamically specify blend mode and composite operator.
  ///
  /// ```
  /// # use msgw3c::{Blend, blend::BlendMode, color::C, composite::PorterDuff};
  ///
  /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  /// # struct Rgba {
  /// #   r: u8,
  /// #   g: u8,
  /// #   b: u8,
  /// #   a: u8,
  /// # }
  ///
  /// # impl Blend for Rgba {
  /// #   fn from_color(color: C) -> Self {
  /// #     let color = color.to_straight_alpha();
  /// #
  /// #     Self {
  /// #       r: (color.r * 255.).clamp(0.0, 255.0) as u8,
  /// #       g: (color.g * 255.).clamp(0.0, 255.0) as u8,
  /// #       b: (color.b * 255.).clamp(0.0, 255.0) as u8,
  /// #       a: (color.a * 255.).clamp(0.0, 255.0) as u8,
  /// #     }
  /// #   }
  ///
  /// #   fn to_color(&self) -> C {
  /// #     C::from_straight_alpha(
  /// #       self.r as f32 / 255.,
  /// #       self.g as f32 / 255.,
  /// #       self.b as f32 / 255.,
  /// #       self.a as f32 / 255.,
  /// #     )
  /// #   }
  /// # }
  ///
  /// # let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
  /// # let source = Rgba { r: 200, g: 30, b: 10, a: 200 };
  ///
  /// let result = source.blend_with(&backdrop, BlendMode::Lighten, PorterDuff::SourceAtop);
  /// ```
  #[inline]
  fn blend_with(&self, backdrop: &impl Blend, mode: BlendMode, op: PorterDuff) -> Self {
    // Since “Normal Blend” and “SourceOver Composite” are a commonly used combination,
    // use the formula optimized for performance.
    if mode == BlendMode::Normal && op == PorterDuff::SourceOver {
      let cb = backdrop.to_color();
      let cs = self.to_color();

      // Cr = Cs + Cb x (1 - αs)
      // αr = αs + αb x (1 - αs)
      let inv_a = 1.0 - cs.a;
      let r = cs.r + cb.r * inv_a;
      let g = cs.g + cb.g * inv_a;
      let b = cs.b + cb.b * inv_a;
      let a = cs.a + cb.a * inv_a;

      return Self::from_color(C::new(r, g, b, a));
    }

    // These operators do not depend on the source or the blend formula.
    // Return before converting the source color or evaluating the blend mode.
    match op {
      PorterDuff::Clear => Self::from_color(C::TRANSPARENT),
      PorterDuff::Copy => Self::from_color(self.to_color()),
      PorterDuff::Destination => Self::from_color(backdrop.to_color()),
      _ => self.apply_blend_and_composite(backdrop, mode, op),
    }
  }

  /// Perform color blending and compositing using any calculation formula that
  /// implements `BlendFormula` and `CompositeOperator`.
  #[inline]
  fn apply_blend_and_composite<F: BlendFormula, Op: CompositeOperator>(
    &self,
    backdrop: &impl Blend,
    f: F,
    op: Op,
  ) -> Self {
    let cb = backdrop.to_color();
    let cs = self.to_color();

    let co = apply_blend_and_composite(cs, cb, &f, &op);

    Self::from_color(co)
  }
}

/// Blends batches of colors using the `normal` blend mode and `SourceOver` compositing.
///
/// Use [`batch_normal_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_normal<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  check_colors_len(sources, backdrops, target)?;

  let target_len = target.len();
  assert!(sources.len() >= target_len);
  assert!(backdrops.len() >= target_len);

  for ((target, source), backdrop) in target.iter_mut().zip(sources).zip(backdrops) {
    let cs = source.to_color();
    let cb = backdrop.to_color();

    // Since “Normal Blend” and “SourceOver Composite” are a commonly used combination,
    // use the formula optimized for performance.

    // Cr = Cs + Cb x (1 - αs)
    // αr = αs + αb x (1 - αs)
    let inv_a = 1.0 - cs.a;
    let co_r = cs.r + cb.r * inv_a;
    let co_g = cs.g + cb.g * inv_a;
    let co_b = cs.b + cb.b * inv_a;
    let co_a = cs.a + cb.a * inv_a;

    *target = S::from_color(C::new(co_r, co_g, co_b, co_a));
  }

  Ok(())
}

/// Blends batches of colors in place using the `normal` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_normal_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_normal_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  check_colors_len_in_place(sources, backdrops)?;

  for (backdrop, source) in backdrops.iter_mut().zip(sources) {
    let cs = source.to_color();
    let cb = backdrop.to_color();

    // Since “Normal Blend” and “SourceOver Composite” are a commonly used combination,
    // use the formula optimized for performance.

    // Cr = Cs + Cb x (1 - αs)
    // αr = αs + αb x (1 - αs)
    let inv_a = 1.0 - cs.a;
    let co_r = cs.r + cb.r * inv_a;
    let co_g = cs.g + cb.g * inv_a;
    let co_b = cs.b + cb.b * inv_a;
    let co_a = cs.a + cb.a * inv_a;

    *backdrop = B::from_color(C::new(co_r, co_g, co_b, co_a));
  }

  Ok(())
}

/// Blends batches of colors using the `normal` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_normal_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  if op == PorterDuff::SourceOver {
    return batch_normal(sources, backdrops, target);
  }

  batch_apply_blend_with(sources, backdrops, target, NormalFormula, op)
}

/// Blends batches of colors in place using the `normal` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_normal_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  if op == PorterDuff::SourceOver {
    return batch_normal_in_place(sources, backdrops);
  }

  batch_apply_blend_with_in_place(sources, backdrops, NormalFormula, op)
}

/// Blends batches of colors using the `multiply` blend mode and `SourceOver` compositing.
///
/// Use [`batch_multiply_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_multiply<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    MultiplyFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `multiply` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_multiply_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_multiply_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, MultiplyFormula, SourceOverOperator)
}

/// Blends batches of colors using the `multiply` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_multiply_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, MultiplyFormula, op)
}

/// Blends batches of colors in place using the `multiply` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_multiply_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, MultiplyFormula, op)
}

/// Blends batches of colors using the `screen` blend mode and `SourceOver` compositing.
///
/// Use [`batch_screen_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_screen<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    ScreenFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `screen` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_screen_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_screen_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, ScreenFormula, SourceOverOperator)
}

/// Blends batches of colors using the `screen` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_screen_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, ScreenFormula, op)
}

/// Blends batches of colors in place using the `screen` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_screen_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, ScreenFormula, op)
}

/// Blends batches of colors using the `overlay` blend mode and `SourceOver` compositing.
///
/// Use [`batch_overlay_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_overlay<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    OverlayFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `overlay` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_overlay_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_overlay_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, OverlayFormula, SourceOverOperator)
}

/// Blends batches of colors using the `overlay` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_overlay_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, OverlayFormula, op)
}

/// Blends batches of colors in place using the `overlay` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_overlay_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, OverlayFormula, op)
}

/// Blends batches of colors using the `darken` blend mode and `SourceOver` compositing.
///
/// Use [`batch_darken_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_darken<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    DarkenFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `darken` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_darken_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_darken_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, DarkenFormula, SourceOverOperator)
}

/// Blends batches of colors using the `darken` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_darken_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, DarkenFormula, op)
}

/// Blends batches of colors in place using the `darken` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_darken_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, DarkenFormula, op)
}

/// Blends batches of colors using the `lighten` blend mode and `SourceOver` compositing.
///
/// Use [`batch_lighten_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_lighten<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    LightenFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `lighten` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_lighten_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_lighten_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, LightenFormula, SourceOverOperator)
}

/// Blends batches of colors using the `lighten` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_lighten_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, LightenFormula, op)
}

/// Blends batches of colors in place using the `lighten` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_lighten_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, LightenFormula, op)
}

/// Blends batches of colors using the `color-dodge` blend mode and `SourceOver` compositing.
///
/// Use [`batch_color_dodge_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color_dodge<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    ColorDodgeFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `color-dodge` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_color_dodge_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color_dodge_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(
    sources,
    backdrops,
    ColorDodgeFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors using the `color-dodge` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_color_dodge_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, ColorDodgeFormula, op)
}

/// Blends batches of colors in place using the `color-dodge` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_color_dodge_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, ColorDodgeFormula, op)
}

/// Blends batches of colors using the `color-burn` blend mode and `SourceOver` compositing.
///
/// Use [`batch_color_burn_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color_burn<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    ColorBurnFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `color-burn` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_color_burn_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color_burn_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, ColorBurnFormula, SourceOverOperator)
}

/// Blends batches of colors using the `color-burn` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_color_burn_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, ColorBurnFormula, op)
}

/// Blends batches of colors in place using the `color-burn` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_color_burn_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, ColorBurnFormula, op)
}

/// Blends batches of colors using the `hard-light` blend mode and `SourceOver` compositing.
///
/// Use [`batch_hard_light_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_hard_light<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    HardLightFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `hard-light` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_hard_light_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_hard_light_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, HardLightFormula, SourceOverOperator)
}

/// Blends batches of colors using the `hard-light` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_hard_light_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, HardLightFormula, op)
}

/// Blends batches of colors in place using the `hard-light` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_hard_light_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, HardLightFormula, op)
}

/// Blends batches of colors using the `soft-light` blend mode and `SourceOver` compositing.
///
/// Use [`batch_soft_light_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_soft_light<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    SoftLightFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `soft-light` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_soft_light_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_soft_light_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, SoftLightFormula, SourceOverOperator)
}

/// Blends batches of colors using the `soft-light` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_soft_light_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, SoftLightFormula, op)
}

/// Blends batches of colors in place using the `soft-light` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_soft_light_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, SoftLightFormula, op)
}

/// Blends batches of colors using the `difference` blend mode and `SourceOver` compositing.
///
/// Use [`batch_difference_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_difference<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    DifferenceFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `difference` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_difference_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_difference_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(
    sources,
    backdrops,
    DifferenceFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors using the `difference` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_difference_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, DifferenceFormula, op)
}

/// Blends batches of colors in place using the `difference` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_difference_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, DifferenceFormula, op)
}

/// Blends batches of colors using the `exclusion` blend mode and `SourceOver` compositing.
///
/// Use [`batch_exclusion_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_exclusion<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    ExclusionFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `exclusion` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_exclusion_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_exclusion_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, ExclusionFormula, SourceOverOperator)
}

/// Blends batches of colors using the `exclusion` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_exclusion_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, ExclusionFormula, op)
}

/// Blends batches of colors in place using the `exclusion` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_exclusion_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, ExclusionFormula, op)
}

/// Blends batches of colors using the `hue` blend mode and `SourceOver` compositing.
///
/// Use [`batch_hue_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_hue<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(sources, backdrops, target, HueFormula, SourceOverOperator)
}

/// Blends batches of colors in place using the `hue` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_hue_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_hue_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, HueFormula, SourceOverOperator)
}

/// Blends batches of colors using the `hue` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_hue_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, HueFormula, op)
}

/// Blends batches of colors in place using the `hue` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_hue_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, HueFormula, op)
}

/// Blends batches of colors using the `saturation` blend mode and `SourceOver` compositing.
///
/// Use [`batch_saturation_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_saturation<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    SaturationFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `saturation` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_saturation_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_saturation_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(
    sources,
    backdrops,
    SaturationFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors using the `saturation` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_saturation_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, SaturationFormula, op)
}

/// Blends batches of colors in place using the `saturation` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_saturation_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, SaturationFormula, op)
}

/// Blends batches of colors using the `color` blend mode and `SourceOver` compositing.
///
/// Use [`batch_color_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(sources, backdrops, target, ColorFormula, SourceOverOperator)
}

/// Blends batches of colors in place using the `color` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_color_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_color_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(sources, backdrops, ColorFormula, SourceOverOperator)
}

/// Blends batches of colors using the `color` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_color_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, ColorFormula, op)
}

/// Blends batches of colors in place using the `color` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_color_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, ColorFormula, op)
}

/// Blends batches of colors using the `luminosity` blend mode and `SourceOver` compositing.
///
/// Use [`batch_luminosity_with()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_luminosity<S, B>(sources: &[S], backdrops: &[B], target: &mut [S]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite(
    sources,
    backdrops,
    target,
    LuminosityFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors in place using the `luminosity` blend mode and `SourceOver` compositing.
///
/// The results are stored in `backdrops`.
/// Use [`batch_luminosity_with_in_place()`] to specify a different Porter-Duff compositing operator.
#[inline]
pub fn batch_luminosity_in_place<S, B>(sources: &[S], backdrops: &mut [B]) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_and_composite_in_place(
    sources,
    backdrops,
    LuminosityFormula,
    SourceOverOperator,
  )
}

/// Blends batches of colors using the `luminosity` blend mode and the specified Porter-Duff compositing operator.
#[inline]
pub fn batch_luminosity_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with(sources, backdrops, target, LuminosityFormula, op)
}

/// Blends batches of colors in place using the `luminosity` blend mode and the specified Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_luminosity_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  batch_apply_blend_with_in_place(sources, backdrops, LuminosityFormula, op)
}

/// Blends batches of colors using the specified blend mode and Porter-Duff compositing operator.
#[inline]
pub fn batch_blend_with<S, B>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  mode: BlendMode,
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  match mode {
    BlendMode::Normal => batch_normal_with(sources, backdrops, target, op),
    BlendMode::Multiply => batch_multiply_with(sources, backdrops, target, op),
    BlendMode::Screen => batch_screen_with(sources, backdrops, target, op),
    BlendMode::Overlay => batch_overlay_with(sources, backdrops, target, op),
    BlendMode::Darken => batch_darken_with(sources, backdrops, target, op),
    BlendMode::Lighten => batch_lighten_with(sources, backdrops, target, op),
    BlendMode::ColorDodge => batch_color_dodge_with(sources, backdrops, target, op),
    BlendMode::ColorBurn => batch_color_burn_with(sources, backdrops, target, op),
    BlendMode::HardLight => batch_hard_light_with(sources, backdrops, target, op),
    BlendMode::SoftLight => batch_soft_light_with(sources, backdrops, target, op),
    BlendMode::Difference => batch_difference_with(sources, backdrops, target, op),
    BlendMode::Exclusion => batch_exclusion_with(sources, backdrops, target, op),
    BlendMode::Hue => batch_hue_with(sources, backdrops, target, op),
    BlendMode::Saturation => batch_saturation_with(sources, backdrops, target, op),
    BlendMode::Color => batch_color_with(sources, backdrops, target, op),
    BlendMode::Luminosity => batch_luminosity_with(sources, backdrops, target, op),
  }
}

/// Blends batches of colors in place using the specified blend mode and Porter-Duff compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_blend_with_in_place<S, B>(
  sources: &[S],
  backdrops: &mut [B],
  mode: BlendMode,
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  match mode {
    BlendMode::Normal => batch_normal_with_in_place(sources, backdrops, op),
    BlendMode::Multiply => batch_multiply_with_in_place(sources, backdrops, op),
    BlendMode::Screen => batch_screen_with_in_place(sources, backdrops, op),
    BlendMode::Overlay => batch_overlay_with_in_place(sources, backdrops, op),
    BlendMode::Darken => batch_darken_with_in_place(sources, backdrops, op),
    BlendMode::Lighten => batch_lighten_with_in_place(sources, backdrops, op),
    BlendMode::ColorDodge => batch_color_dodge_with_in_place(sources, backdrops, op),
    BlendMode::ColorBurn => batch_color_burn_with_in_place(sources, backdrops, op),
    BlendMode::HardLight => batch_hard_light_with_in_place(sources, backdrops, op),
    BlendMode::SoftLight => batch_soft_light_with_in_place(sources, backdrops, op),
    BlendMode::Difference => batch_difference_with_in_place(sources, backdrops, op),
    BlendMode::Exclusion => batch_exclusion_with_in_place(sources, backdrops, op),
    BlendMode::Hue => batch_hue_with_in_place(sources, backdrops, op),
    BlendMode::Saturation => batch_saturation_with_in_place(sources, backdrops, op),
    BlendMode::Color => batch_color_with_in_place(sources, backdrops, op),
    BlendMode::Luminosity => batch_luminosity_with_in_place(sources, backdrops, op),
  }
}

/// Blends batches of colors using the specified blend formula and compositing operator.
#[inline]
pub fn batch_apply_blend_and_composite<S, B, F, Op>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  f: F,
  op: Op,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
  F: BlendFormula,
  Op: CompositeOperator,
{
  check_colors_len(sources, backdrops, target)?;

  let target_len = target.len();
  assert!(sources.len() >= target_len);
  assert!(backdrops.len() >= target_len);

  for ((target, source), backdrop) in target.iter_mut().zip(sources).zip(backdrops) {
    *target = S::from_color(apply_blend_and_composite(
      source.to_color(),
      backdrop.to_color(),
      &f,
      &op,
    ));
  }

  Ok(())
}

/// Blends batches of colors in place using the specified blend formula and compositing operator.
///
/// The results are stored in `backdrops`.
#[inline]
pub fn batch_apply_blend_and_composite_in_place<S, B, F, Op>(
  sources: &[S],
  backdrops: &mut [B],
  f: F,
  op: Op,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
  F: BlendFormula,
  Op: CompositeOperator,
{
  check_colors_len_in_place(sources, backdrops)?;

  for (backdrop, source) in backdrops.iter_mut().zip(sources) {
    *backdrop = B::from_color(apply_blend_and_composite(
      source.to_color(),
      backdrop.to_color(),
      &f,
      &op,
    ));
  }

  Ok(())
}

#[inline]
fn apply_blend_and_composite<F: BlendFormula, Op: CompositeOperator>(
  cs: C,
  cb: C,
  f: &F,
  op: &Op,
) -> C {
  // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
  // Compositing: co = αs x Fa x Cr + αb x Fb x Cb
  // https://www.w3.org/TR/compositing-1/#blending
  // https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators
  // co = αs x Fa x ((1 - αb) x Cs + αb x B(Cb, Cs)) + αb x Fb x Cb
  // co = αs x Fa x (1 - αb) x Cs + αs x Fa x αb x B(Cb, Cs) + αb x Fb x Cb
  // co = αs x Fa x (1 - αb) x cs / αs + αs x Fa x αb x B(cb / αb, cs / αs) + αb x Fb x cb / αb
  // co = Fa x (1 - αb) x cs + αs x Fa x αb x B(cb / αb, cs / αs) + Fb x cb
  // co = Fa x (1 - αb) x cs + Fa x αs x αb x B(cb / αb, cs / αs) + Fb x cb
  // def: K = αs x αb x B(cb / αb, cs / αs)
  //      K is blending formula for premultiplied alpha
  // co = Fa x (1 - αb) x cs + Fa x K + Fb x cb
  let (k_r, k_g, k_b) = f.apply_k(cb, cs);

  let factors = op.fractions(cs.a, cb.a);

  // αo = αs x Fa + αb x Fb
  let a0 = cs.a * factors.fa + cb.a * factors.fb;
  let inv_b_a = 1. - cb.a;

  let co_r = factors.fa * inv_b_a * cs.r + factors.fa * k_r + factors.fb * cb.r;
  let co_g = factors.fa * inv_b_a * cs.g + factors.fa * k_g + factors.fb * cb.g;
  let co_b = factors.fa * inv_b_a * cs.b + factors.fa * k_b + factors.fb * cb.b;

  C::new(co_r, co_g, co_b, a0)
}

#[inline]
fn batch_apply_blend_with<S, B, F: BlendFormula>(
  sources: &[S],
  backdrops: &[B],
  target: &mut [S],
  f: F,
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  match op {
    PorterDuff::Clear => {
      check_colors_len(sources, backdrops, target)?;

      let target_len = target.len();
      assert!(sources.len() >= target_len);
      assert!(backdrops.len() >= target_len);

      target[..target_len].fill(S::from_color(C::TRANSPARENT));
      Ok(())
    }
    PorterDuff::Destination => {
      check_colors_len(sources, backdrops, target)?;

      let target_len = target.len();
      assert!(sources.len() >= target_len);
      assert!(backdrops.len() >= target_len);

      for (out, b) in target[..target_len]
        .iter_mut()
        .zip(&backdrops[..target_len])
      {
        *out = S::from_color(b.to_color());
      }
      Ok(())
    }
    PorterDuff::Copy => {
      check_colors_len(sources, backdrops, target)?;

      let target_len = target.len();
      assert!(sources.len() >= target_len);
      assert!(backdrops.len() >= target_len);

      target[..target_len].copy_from_slice(&sources[..target_len]);
      Ok(())
    }
    PorterDuff::SourceOver => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, SourceOverOperator)
    }
    PorterDuff::DestinationOver => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, DestinationOverOperator)
    }
    PorterDuff::SourceIn => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, SourceInOperator)
    }
    PorterDuff::DestinationIn => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, DestinationInOperator)
    }
    PorterDuff::SourceOut => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, SourceOutOperator)
    }
    PorterDuff::DestinationOut => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, DestinationOutOperator)
    }
    PorterDuff::SourceAtop => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, SourceAtopOperator)
    }
    PorterDuff::DestinationAtop => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, DestinationAtopOperator)
    }
    PorterDuff::Xor => batch_apply_blend_and_composite(sources, backdrops, target, f, XorOperator),
    PorterDuff::Lighter => {
      batch_apply_blend_and_composite(sources, backdrops, target, f, LighterOperator)
    }
  }
}

#[inline]
fn batch_apply_blend_with_in_place<S, B, F: BlendFormula>(
  sources: &[S],
  backdrops: &mut [B],
  f: F,
  op: PorterDuff,
) -> Result<(), Error>
where
  S: Blend,
  B: Blend,
{
  match op {
    PorterDuff::Clear => {
      check_colors_len_in_place(sources, backdrops)?;
      backdrops.fill(B::from_color(C::TRANSPARENT));
      Ok(())
    }
    PorterDuff::Destination => check_colors_len_in_place(sources, backdrops),
    PorterDuff::Copy => {
      check_colors_len_in_place(sources, backdrops)?;

      for (backdrop, source) in backdrops.iter_mut().zip(sources) {
        *backdrop = B::from_color(source.to_color());
      }
      Ok(())
    }
    PorterDuff::SourceOver => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, SourceOverOperator)
    }
    PorterDuff::DestinationOver => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, DestinationOverOperator)
    }
    PorterDuff::SourceIn => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, SourceInOperator)
    }
    PorterDuff::DestinationIn => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, DestinationInOperator)
    }
    PorterDuff::SourceOut => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, SourceOutOperator)
    }
    PorterDuff::DestinationOut => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, DestinationOutOperator)
    }
    PorterDuff::SourceAtop => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, SourceAtopOperator)
    }
    PorterDuff::DestinationAtop => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, DestinationAtopOperator)
    }
    PorterDuff::Xor => batch_apply_blend_and_composite_in_place(sources, backdrops, f, XorOperator),
    PorterDuff::Lighter => {
      batch_apply_blend_and_composite_in_place(sources, backdrops, f, LighterOperator)
    }
  }
}

#[inline]
fn check_colors_len<S: Blend, B: Blend>(
  sources: &[S],
  backdrops: &[B],
  target: &[S],
) -> Result<(), Error> {
  let sources_len = sources.len();
  let backdrops_len = backdrops.len();
  let target_len = target.len();
  if sources_len < target_len || backdrops_len < target_len {
    return Err(Error::LengthMismatch {
      source: sources_len,
      backdrop: backdrops_len,
      target: target_len,
    });
  }

  Ok(())
}

#[inline]
fn check_colors_len_in_place<S: Blend, B: Blend>(
  sources: &[S],
  backdrops: &[B],
) -> Result<(), Error> {
  let sources_len = sources.len();
  let backdrops_len = backdrops.len();
  if sources_len < backdrops_len {
    return Err(Error::LengthMismatch {
      source: sources_len,
      backdrop: backdrops_len,
      target: backdrops_len,
    });
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone, Copy)]
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

    fn from_straight(r: f32, g: f32, b: f32, a: f32) -> Self {
      Self::new(r * a, g * a, b * a, a)
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
    let bg = Rgba::from_straight(1., 0., 0., 0.);
    let fg = Rgba::from_straight(0., 0., 1., 0.);

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
      assert_rgba(result, Rgba::from_straight(0., 0., 0., 0.));
    }
  }

  #[test]
  fn test_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.);

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
      assert_rgba(result, Rgba::from_straight(1., 0., 0., 1.));
    }
  }

  #[test]
  fn test_normal_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 1.) + 1. x 0. x (1., 0., 0.) = (0., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. x 0. = 1.
    assert_rgba(
      fg.normal(&bg),
      Rgba::from_straight(0. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_normal_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 1.) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.5, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 0.5 + 0.5 = 1.0
    assert_rgba(
      fg.normal(&bg),
      Rgba::from_straight(0.5 / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_normal_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // normal: B(Cb, Cs) = Cs = (0., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 1.) = (0., 0., 0.5) + (0., 0., 0.5) = (0., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.5) + (0.25, 0., 0.) = (0.25, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.normal(&bg),
      Rgba::from_straight(0.25 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_multiply_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 0.) + 1. x 0. x (1., 0., 0.) = (0., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.multiply(&bg),
      Rgba::from_straight(0. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_multiply_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0., 0., 0.) + (0.5, 0., 0.) = (0.5, 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.multiply(&bg),
      Rgba::from_straight(0.5 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_multiply_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // multiply: B(Cb, Cs) = Cb x Cs = (1., 0., 0.) x (0., 0., 1.) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 0.) = (0., 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.25) + (0.25, 0., 0.) = (0.25, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.multiply(&bg),
      Rgba::from_straight(0.25 / 0.75, 0., 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_screen_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.screen(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_screen_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.screen(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_screen_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // screen: B(Cb, Cs) = Cb + Cs - (Cb x Cs) = (1., 0., 0.) + (0., 0., 1.) - ((1., 0., 0.) x (0., 0., 1.)) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.screen(&bg),
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_overlay_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
    assert_rgba(
      fg.overlay(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_overlay_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
    assert_rgba(
      fg.overlay(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_overlay_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_darken_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (0., 0., 0.) + 1. x 0. x (1., 0., 0.) = (0., 0., 0.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.darken(&bg),
      Rgba::from_straight(0. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_darken_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (0., 0., 0.) = (0., 0., 0.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.darken(&bg),
      Rgba::from_straight(0.5 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_darken_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // darken: B(Cb, Cs) = min(Cb, Cs) = (0., 0., 0.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (0., 0., 0.) = (0., 0., 0.5)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0., 0., 0.5) + 0.5 x 0.5 x (1., 0., 0.) = (0., 0., 0.25) + (0.25, 0., 0.) = (0.25, 0., 0.25)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.darken(&bg),
      Rgba::from_straight(0.25 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_lighten_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.lighten(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_lighten_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.lighten(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_lighten_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // lighten: B(Cb, Cs) = max(Cb, Cs) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.lighten(&bg),
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_dodge_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_dodge_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_dodge_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_burn_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
    assert_rgba(
      fg.color_burn(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_burn_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
    assert_rgba(
      fg.color_burn(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_color_burn_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_hard_light_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
    assert_rgba(
      fg.hard_light(&bg),
      Rgba::from_straight(0. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_hard_light_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_hard_light_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.25 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_soft_light_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
    assert_rgba(
      fg.soft_light(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_soft_light_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
    assert_rgba(
      fg.soft_light(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_soft_light_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_difference_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.difference(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_difference_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.difference(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_difference_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // difference: B(Cb, Cs) = | Cb - Cs |
    //                       = |(1., 0., 0.) - (0., 0., 1.)| = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.difference(&bg),
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_exclusion_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 1. x 1. x (1., 0., 1.) + 1. x 0. x (1., 0., 0.) = (1., 0., 1.)
    // αo = αs x Fa + αb x Fb = 1. x 1. + 1. 0. = 1.
    assert_rgba(
      fg.exclusion(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_exclusion_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 1.) x (0., 0., 1.) + 1. x (1., 0., 1.) = (1., 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (1., 0., 1.) + 1. x 0.5 x (1., 0., 0.) = (0.5, 0., 0.5) + (0.5, 0., 0.) = (1., 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 1. x 0.5 = 1.
    assert_rgba(
      fg.exclusion(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_exclusion_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

    // SourceOver: Fa = 1; Fb = 1 – αs
    //           : Fa = 1.; Fb = 0.5
    // exclusion: B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
    //                      = (1., 0., 0.) + (0., 0., 1.) - 2. x (1., 0., 0.) x (0., 0., 1.) = (1., 0., 1.) - (0., 0., 0.) = (1., 0., 1.)
    // Cr = (1 - αb) x Cs + αb x B(Cb, Cs) = (1. - 0.5) x (0., 0., 1.) + 0.5 x (1., 0., 1.) = (0., 0., 0.5) + (0.5, 0., 0.5) = (0.5, 0., 1.)
    // Co = αs x Fa x Cr + αb x Fb x Cb = 0.5 x 1. x (0.5, 0., 1.) + 0.5 x 0.5 x (1., 0., 0.) = (0.25, 0., 0.5) + (0.25, 0., 0.) = (0.5, 0., 0.5)
    // αo = αs x Fa + αb x Fb = 0.5 x 1. + 0.5 x 0.5 = 0.75
    assert_rgba(
      fg.exclusion(&bg),
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_hue_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
      Rgba::from_straight(0.19 / 0.89 / 1., 0.19 / 0.89 / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_hue_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.54 / 0.89 / 1., 0.095 / 0.89 / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_hue_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.27 / 0.89 / 0.75, 0.0475 / 0.89 / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_saturation_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
    assert_rgba(
      fg.saturation(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_saturation_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
    assert_rgba(
      fg.saturation(&bg),
      Rgba::from_straight(1. / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_saturation_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.5 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_color_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
      Rgba::from_straight(0.19 / 0.89 / 1., 0.19 / 0.89 / 1., 1. / 1., 1.),
    );
  }

  #[test]
  fn test_color_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.54 / 0.89 / 1., 0.095 / 0.89 / 1., 0.5 / 1., 1.),
    );
  }

  #[test]
  fn test_color_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.27 / 0.89 / 0.75, 0.0475 / 0.89 / 0.75, 0.5 / 0.75, 0.75),
    );
  }

  #[test]
  fn test_luminosity_opaque_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 1.);

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
      Rgba::from_straight(0.11 / 0.3 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_luminosity_semi_transparent_over_opaque() {
    let bg = Rgba::from_straight(1., 0., 0., 1.);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.205 / 0.3 / 1., 0. / 1., 0. / 1., 1.),
    );
  }

  #[test]
  fn test_luminosity_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_straight(1., 0., 0., 0.5);
    let fg = Rgba::from_straight(0., 0., 1., 0.5);

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
      Rgba::from_straight(0.1025 / 0.3 / 0.75, 0. / 0.75, 0.25 / 0.75, 0.75),
    );
  }
}
