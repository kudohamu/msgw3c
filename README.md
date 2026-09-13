# msgw3c

This crate provides color blending functionality based on the W3C's [Compositing and Blending Level 1](https://www.w3.org/TR/compositing-1/) specification.
You don't need to substitute with dedicated types. Simply implement the `Blend` trait for any color type you like, and you can access to a set of blend functions that available a set of blend functions that can use blend modes and composition operators from their type.

## features

- 16 blend modes
- 13 Porter-Duff compositing methods
- Support for custom blend and composite formulas
- Batch processing functions
- No dependency

## Usage

```toml
[dependencies]
msgw3c = "0.2.0"
```

Implement the `Blend` trait for any color type you like.

```rust
use msgw3c::{
  Blend,
  blend::BlendMode,
  color::C,
  composite::PorterDuff,
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
    if color.a == 0. {
      return Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      };
    }

    Self {
      r: (color.r * 255. / color.a).clamp(0.0, 255.0) as u8,
      g: (color.g * 255. / color.a).clamp(0.0, 255.0) as u8,
      b: (color.b * 255. / color.a).clamp(0.0, 255.0) as u8,
      a: (color.a * 255.).clamp(0.0, 255.0) as u8,
    }
  }

  fn to_color(&self) -> C {
    C::from_straight_alpha(
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
assert_eq!(
  result,
  Rgba { r: 83, g: 61, b: 51, a: 255 }
);

// `*_with` functions allow you to specify any Porter-Duff composition method.
let result = source.multiply_with(&backdrop, PorterDuff::DestinationOver);
assert_eq!(
  result,
  Rgba { r: 100, g: 200, b: 210, a: 255 }
);

// To select a blend mode at runtime, you can use the blend_with function.
let result = source.blend_with(&backdrop, BlendMode::Overlay, PorterDuff::SourceAtop);
assert_eq!(
  result,
  Rgba { r: 144, g: 167, b: 177, a: 255 }
);
```

### Custom blend / composite formulas

You can also use custom blend and composite formulas you like.

```rust,ignore
use msgw3c::{
    blend::BlendFormula,
    color::C,
    composite::{CompositeFactors, CompositeOperator},
};

#[derive(Debug, Clone, Copy)]
struct Subtract;

impl BlendFormula for Subtract {
  fn apply_k(&self, cb: C, cs: C) -> (f32, f32, f32) {
    cb.a - cs.a
  }
}

#[derive(Debug, Clone, Copy)]
struct SquaredAlphaOver;

impl CompositeOperator for SquaredAlphaOver {
  fn fractions(&self, cs_a: f32, _cb_a: f32) -> CompositeFactors {
    CompositeFactors::new(cs_a * cs_a, 1. - cs_a * cs_a)
  }
}

// apply original blend / composite formulas
let result = source.apply_blend_and_composite(&backdrop, Subtract, SquaredAlphaOver);
```

### batch blending

```rust
use msgw3c::{
  batch_normal,
  batch_multiply_with,
  batch_blend_with,
  batch_normal_in_place,
  batch_multiply_with_in_place,
  batch_blend_with_in_place,
  blend::BlendMode,
  Blend,
  color::C,
  composite::PorterDuff,
};

// Your color type.
#[derive(Debug, Clone, Copy)]
struct Rgba {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

impl Rgba {
  const TRANSPARENT: Self = Self { r: 0., g: 0., b: 0., a: 0. };
}

impl Blend for Rgba {
  fn from_color(color: C) -> Self {
    Self {
      r: color.r,
      g: color.g,
      b: color.b,
      a: color.a,
    }
  }

  fn to_color(&self) -> C {
    C::new(self.r, self.g, self.b, self.a)
  }
}

let backdrops = vec![Rgba { r: 0.1, g: 0.2, b: 0.25, a: 1.0 }, Rgba { r: 0.3, g: 0.22, b: 0.2, a: 0.8 }, Rgba { r: 0.6, g: 0.8, b: 1.0, a: 0.5 }];
let sources = vec![Rgba { r: 0.9, g: 0.3, b: 0.1, a: 0.2 }, Rgba { r: 0.45, g: 0.12, b: 0.66, a: 0.8 }, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0. }];
let mut target = vec![Rgba::TRANSPARENT; 3];

// batch blended
batch_normal(&sources, &backdrops, &mut target)?;
// custom operator blended
batch_multiply_with(&sources, &backdrops, &mut target, PorterDuff::SourceIn)?;
// runtime blend mode and operator blended
batch_blend_with(&sources, &backdrops, &mut target, BlendMode::Screen, PorterDuff::SourceIn)?;


let sources = vec![Rgba { r: 0.9, g: 0.3, b: 0.1, a: 0.2 }, Rgba { r: 0.45, g: 0.12, b: 0.66, a: 0.8 }, Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0. }];

// batch blended(in place ver)
let mut backdrops = vec![Rgba { r: 0.1, g: 0.2, b: 0.25, a: 1.0 }, Rgba { r: 0.3, g: 0.22, b: 0.2, a: 0.8 }, Rgba { r: 0.6, g: 0.8, b: 1.0, a: 0.5 }];
batch_normal_in_place(&sources, &mut backdrops)?;

// custom operator blended(in place ver)
let mut backdrops = vec![Rgba { r: 0.1, g: 0.2, b: 0.25, a: 1.0 }, Rgba { r: 0.3, g: 0.22, b: 0.2, a: 0.8 }, Rgba { r: 0.6, g: 0.8, b: 1.0, a: 0.5 }];
batch_multiply_with_in_place(&sources, &mut backdrops, PorterDuff::SourceIn)?;

// runtime blend mode and operator blended(in place ver)
let mut backdrops = vec![Rgba { r: 0.1, g: 0.2, b: 0.25, a: 1.0 }, Rgba { r: 0.3, g: 0.22, b: 0.2, a: 0.8 }, Rgba { r: 0.6, g: 0.8, b: 1.0, a: 0.5 }];
batch_blend_with_in_place(&sources, &mut backdrops, BlendMode::Screen, PorterDuff::SourceIn)?;
```

## Performance

The batch benchmark processes a 1920 × 1080 image in a single thread.

| Blend mode | Single pixel | 1080p batch |
|---|---:|---:|
| Normal | 1.014 ns | 1.010 ms |
| Multiply | 1.909 ns | 1.317 ms |
| Screen | 2.086 ns | 1.676 ms |
| Overlay | 2.193 ns | 2.135 ms |
| Darken | 1.946 ns | 1.490 ms |
| Lighten | 1.882 ns | 1.491 ms |
| ColorDodge | 2.750 ns | 2.177 ms |
| ColorBurn | 3.347 ns | 2.658 ms |
| HardLight | 2.366 ns | 2.127 ms |
| SoftLight | 4.532 ns | 27.731 ms |
| Difference | 1.878 ns | 1.491 ms |
| Exclusion | 2.076 ns | 1.775 ms |
| Hue | 6.019 ns | 26.694 ms |
| Saturation | 6.249 ns | 32.278 ms |
| Color | 4.375 ns | 18.100 ms |
| Luminosity | 4.552 ns | 18.063 ms |

These results use the `SourceOver` compositing operator. See the [detailed benchmark results](docs/benchmark.md) for all blend mode and Porter-Duff operator combinations and complete measurement conditions.

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
