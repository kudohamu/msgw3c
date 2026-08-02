/// the Trait that returns a combination of factors for controlling the blending results
/// of the four subpixel regions formed by the overlap of graphic objects
/// with alpha channels or pixel coverage channels/values.
pub trait CompositeOperator {
  /// Returns a combination of factors for controlling the blending results
  /// of the four subpixel regions formed by the overlap of graphic objects
  /// with alpha channels or pixel coverage channels/values.
  fn fractions(&self, cs_a: f32, cb_a: f32) -> (f32, f32);
}

/// The Porter-Duff Compositing Operators.
/// There are 12 basic Porter Duff operators, satisfying all possible combinations of source and destination.
/// https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
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
  fn fractions(&self, cs_a: f32, cb_a: f32) -> (f32, f32) {
    match self {
      Self::Clear => (0., 0.),
      Self::Copy => (1., 0.),
      Self::Destination => (0., 1.),
      Self::SourceOver => (1., (1. - cs_a)),
      Self::DestinationOver => ((1. - cb_a), 1.),
      Self::SourceIn => (cb_a, 0.),
      Self::DestinationIn => (0., cs_a),
      Self::SourceOut => ((1. - cb_a), 0.),
      Self::DestinationOut => (0., (1. - cs_a)),
      Self::SourceAtop => (cb_a, (1. - cs_a)),
      Self::DestinationAtop => ((1. - cb_a), cs_a),
      Self::Xor => ((1. - cb_a), (1. - cs_a)),
      Self::Lighter => (1., 1.),
    }
  }
}
