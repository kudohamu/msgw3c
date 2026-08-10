#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
  LengthMismatch {
    source: usize,
    backdrop: usize,
    target: usize,
  },
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LengthMismatch {
        source,
        backdrop,
        target,
      } => write!(
        f,
        "the lengths of `source`, `backdrop` and `target` do not match (source: {}, backdrop: {}, target: {})",
        source, backdrop, target,
      ),
    }
  }
}

impl std::error::Error for Error {}
