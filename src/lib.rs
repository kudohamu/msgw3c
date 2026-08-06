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
pub mod blend;
pub mod color;
mod formula;
pub mod porter_duff;
