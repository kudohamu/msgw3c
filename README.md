# msgw3c

This crate provides color blending functionality based on the W3C's [Compositing and Blending Level 1](https://www.w3.org/TR/compositing-1/) specification.
You don't need to substitute with dedicated types. Simply implement the `Blend` trait for any color type you like, and you can access to a set of blend functions that available a set of blend functions that can use blend modes and composition operators from their type.

## features

- 16 blend modes
- 13 Porter-Duff compositing methods
- No dependency

## Usage

```toml
[dependencies]
msgw3c = "0.1.0"
```

Implement the `Blend` trait for any color type you like.

```rust
use msgw3c::{
    blend::{Blend, BlendMode},
    color::C,
    porter_duff::PorterDuff,
};

// Your color type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// implement Blend trait
impl Blend for Rgba {
  fn from_color(color: C) -> Self {
    Self {
      r: (color.r * 255.).clamp(0.0, 255.0) as u8,
      g: (color.g * 255.).clamp(0.0, 255.0) as u8,
      b: (color.b * 255.).clamp(0.0, 255.0) as u8,
      a: (color.a * 255.).clamp(0.0, 255.0) as u8,
    }
  }

  fn to_color(&self) -> C {
    C::new(
      self.r as f32 / 255.,
      self.g as f32 / 255.,
      self.b as f32 / 255.,
      self.a as f32 / 255.,
    )
  }
}

let backdrop = Rgba { r: 100, g: 200, b: 210, a: 255 };
let source = Rgba { r: 200, g: 30, b: 10, a: 200 };

// Useful blending utility functions for many scenarios.
let result = source.multiply(&backdrop);
assert_eq!(result, Rgba { r: 83, g: 61, b: 51, a: 255 });

// `*_with` functions allow you to specify any Porter-Duff composition method.
let result = source.multiply_with(&backdrop, PorterDuff::DestinationOver);
assert_eq!(result, Rgba { r: 100, g: 200, b: 210, a: 255 });

// To select a blend mode at runtime, you can use the blend_with function.
let result = source.blend_with(&backdrop, BlendMode::Overlay, PorterDuff::SourceAtop);
assert_eq!(result, Rgba { r: 144, g: 167, b: 177, a: 255 });
```

## Blending

The `BlendMode` and its corresponding methods are as follows.
Each of these methods uses the `SourceOver` compositing, while the corresponding `*_with` methods allow you to specify compositing operator.

| blend mode | method |
| :-- | :-- |
| `Normal` | `normal` |
| `Multiply` | `multiply` |
| `Screen` | `screen` |
| `Overlay` | `overlay` |
| `Darken` | `darken` |
| `Lighten` | `lighten` |
| `ColorDodge` | `color_dodge` |
| `ColorBurn` | `color_burn` |
| `HardLight` | `hard_light` |
| `SoftLight` | `soft_light` |
| `Difference` | `difference` |
| `Exclusion` | `exclusion` |
| `Hue` | `hue` |
| `Saturation` | `saturation` |
| `Color` | `color` |
| `Luminosity` | `luminosity` |

## Porter-Duff Compositing Operators

`PorterDuff` has the following operators.

- Clear
- Copy
- Destination
- SourceOver
- DestinationOver
- SourceIn
- DestinationIn
- SourceOut
- DestinationOut
- SourceAtop
- DestinationAtop
- Xor
- Lighter
