/// The source and backdrop factors used by a compositing operator.
/// These factors determine how much of the source and backdrop contribute
/// to the resulting pixel during compositing.
#[derive(Debug, Clone, Copy)]
pub struct CompositeFactors {
  pub fa: f32,
  pub fb: f32,
}

impl CompositeFactors {
  #[inline]
  pub fn new(fa: f32, fb: f32) -> Self {
    Self { fa, fb }
  }
}

/// the Trait that returns a combination of factors for controlling the blending results
/// of the four subpixel regions formed by the overlap of graphic objects
/// with alpha channels or pixel coverage channels/values.
pub trait CompositeOperator {
  /// Returns a combination of factors for controlling the blending results
  /// of the four subpixel regions formed by the overlap of graphic objects
  /// with alpha channels or pixel coverage channels/values.
  fn fractions(&self, cs_a: f32, cb_a: f32) -> CompositeFactors;
}

/// The Porter-Duff Compositing Operators.
/// There are 12 basic Porter Duff operators, satisfying all possible combinations of source and destination.
/// <https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators>
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PorterDuff {
  Clear,
  Copy,
  Destination,
  #[default]
  SourceOver,
  DestinationOver,
  SourceIn,
  DestinationIn,
  SourceOut,
  DestinationOut,
  SourceAtop,
  DestinationAtop,
  Xor,
  Lighter,
}

impl CompositeOperator for PorterDuff {
  /// Returns the fractional terms Fa and Fb which defined for each operator and
  /// specify the fraction of the shapes that contribute to the final pixel value.
  #[inline]
  fn fractions(&self, cs_a: f32, cb_a: f32) -> CompositeFactors {
    match self {
      Self::Clear => ClearOperator.fractions(cs_a, cb_a),
      Self::Copy => CopyOperator.fractions(cs_a, cb_a),
      Self::Destination => DestinationOperator.fractions(cs_a, cb_a),
      Self::SourceOver => SourceOverOperator.fractions(cs_a, cb_a),
      Self::DestinationOver => DestinationOverOperator.fractions(cs_a, cb_a),
      Self::SourceIn => SourceInOperator.fractions(cs_a, cb_a),
      Self::DestinationIn => DestinationInOperator.fractions(cs_a, cb_a),
      Self::SourceOut => SourceOutOperator.fractions(cs_a, cb_a),
      Self::DestinationOut => DestinationOutOperator.fractions(cs_a, cb_a),
      Self::SourceAtop => SourceAtopOperator.fractions(cs_a, cb_a),
      Self::DestinationAtop => DestinationAtopOperator.fractions(cs_a, cb_a),
      Self::Xor => XorOperator.fractions(cs_a, cb_a),
      Self::Lighter => LighterOperator.fractions(cs_a, cb_a),
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClearOperator;

impl CompositeOperator for ClearOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(0., 0.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CopyOperator;

impl CompositeOperator for CopyOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1., 0.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DestinationOperator;

impl CompositeOperator for DestinationOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(0., 1.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceOverOperator;

impl CompositeOperator for SourceOverOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1., 1. - cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DestinationOverOperator;

impl CompositeOperator for DestinationOverOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1. - cb_a, 1.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceInOperator;

impl CompositeOperator for SourceInOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(cb_a, 0.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DestinationInOperator;

impl CompositeOperator for DestinationInOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(0., cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceOutOperator;

impl CompositeOperator for SourceOutOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1. - cb_a, 0.)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DestinationOutOperator;

impl CompositeOperator for DestinationOutOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(0., 1. - cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SourceAtopOperator;

impl CompositeOperator for SourceAtopOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(cb_a, 1. - cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DestinationAtopOperator;

impl CompositeOperator for DestinationAtopOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1. - cb_a, cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct XorOperator;

impl CompositeOperator for XorOperator {
  #[inline]
  fn fractions(&self, cs_a: f32, cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1. - cb_a, 1. - cs_a)
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LighterOperator;

impl CompositeOperator for LighterOperator {
  #[inline]
  fn fractions(&self, _cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(1., 1.)
  }
}
