/// The source and backdrop factors used by a compositing operator.
/// These factors determine how much of the source and backdrop contribute
/// to the resulting pixel during compositing.
#[derive(Debug, Clone, Copy)]
pub struct CompositeFactors {
  pub fa: f32,
  pub fb: f32,
}

impl CompositeFactors {
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
      Self::Clear => CompositeFactors::new(0., 0.),
      Self::Copy => CompositeFactors::new(1., 0.),
      Self::Destination => CompositeFactors::new(0., 1.),
      Self::SourceOver => CompositeFactors::new(1., 1. - cs_a),
      Self::DestinationOver => CompositeFactors::new(1. - cb_a, 1.),
      Self::SourceIn => CompositeFactors::new(cb_a, 0.),
      Self::DestinationIn => CompositeFactors::new(0., cs_a),
      Self::SourceOut => CompositeFactors::new(1. - cb_a, 0.),
      Self::DestinationOut => CompositeFactors::new(0., 1. - cs_a),
      Self::SourceAtop => CompositeFactors::new(cb_a, 1. - cs_a),
      Self::DestinationAtop => CompositeFactors::new(1. - cb_a, cs_a),
      Self::Xor => CompositeFactors::new(1. - cb_a, 1. - cs_a),
      Self::Lighter => CompositeFactors::new(1., 1.),
    }
  }
}
