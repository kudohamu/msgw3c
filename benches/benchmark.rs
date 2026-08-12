use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use msgw3c::batch_blend_with;
use msgw3c::blend::BlendMode;
use msgw3c::composite::PorterDuff;
use msgw3c::{Blend, color::C};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::hint::black_box;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
struct Pixel(C);

impl Blend for Pixel {
  #[inline]
  fn from_color(color: C) -> Self {
    Self(color)
  }

  #[inline]
  fn to_color(&self) -> C {
    self.0
  }
}

#[derive(Debug, Clone, Copy)]
struct BlendAndComposite<'a> {
  pub sources: &'a [Pixel],
  pub backdrops: &'a [Pixel],
  pub mode: BlendMode,
  pub op: PorterDuff,
}

impl std::fmt::Display for BlendAndComposite<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} / {}", self.mode, self.op)?;

    Ok(())
  }
}

fn make_pixels(len: usize, seed: u64) -> Vec<Pixel> {
  let mut rng = StdRng::seed_from_u64(seed);

  (0..len)
    .map(|_| {
      let r: f32 = rng.random_range(0.0..=1.0);
      let g: f32 = rng.random_range(0.0..=1.0);
      let b: f32 = rng.random_range(0.0..=1.0);
      let a: f32 = rng.random_range(0.0..=1.0);

      Pixel(C::from_straight_alpha(r, g, b, a))
    })
    .collect()
}

fn bench_blend(c: &mut Criterion) {
  let mut group = c.benchmark_group("blend_with");

  let sources = make_pixels(1, 1000);
  let backdrops = make_pixels(1, 2000);

  for mode in [
    BlendMode::Normal,
    BlendMode::Multiply,
    BlendMode::Screen,
    BlendMode::Overlay,
    BlendMode::Darken,
    BlendMode::Lighten,
    BlendMode::ColorDodge,
    BlendMode::ColorBurn,
    BlendMode::HardLight,
    BlendMode::SoftLight,
    BlendMode::Difference,
    BlendMode::Exclusion,
    BlendMode::Hue,
    BlendMode::Saturation,
    BlendMode::Color,
    BlendMode::Luminosity,
  ]
  .iter()
  {
    for op in [
      PorterDuff::Clear,
      PorterDuff::Copy,
      PorterDuff::Destination,
      PorterDuff::SourceOver,
      PorterDuff::DestinationOver,
      PorterDuff::SourceIn,
      PorterDuff::DestinationIn,
      PorterDuff::SourceOut,
      PorterDuff::DestinationOut,
      PorterDuff::SourceAtop,
      PorterDuff::DestinationAtop,
      PorterDuff::Xor,
      PorterDuff::Lighter,
    ]
    .iter()
    {
      let param = BlendAndComposite {
        sources: &sources,
        backdrops: &backdrops,
        mode: *mode,
        op: *op,
      };

      group.bench_with_input(BenchmarkId::from_parameter(param), &param, |b, m| {
        b.iter(|| {
          black_box(m.sources[0]).blend_with(
            &black_box(m.backdrops[0]),
            black_box(m.mode),
            black_box(m.op),
          )
        });
      });
    }
  }

  group.finish();
}

fn bench_batch_blend(c: &mut Criterion) {
  let mut group = c.benchmark_group("batch_blend_with");
  group.measurement_time(Duration::from_secs(20));

  let len: usize = 1920 * 1080;
  let sources = make_pixels(len, 2525);
  let backdrops = make_pixels(len, 3939);

  for mode in [
    BlendMode::Normal,
    BlendMode::Multiply,
    BlendMode::Screen,
    BlendMode::Overlay,
    BlendMode::Darken,
    BlendMode::Lighten,
    BlendMode::ColorDodge,
    BlendMode::ColorBurn,
    BlendMode::HardLight,
    BlendMode::SoftLight,
    BlendMode::Difference,
    BlendMode::Exclusion,
    BlendMode::Hue,
    BlendMode::Saturation,
    BlendMode::Color,
    BlendMode::Luminosity,
  ]
  .iter()
  {
    for op in [
      PorterDuff::Clear,
      PorterDuff::Copy,
      PorterDuff::Destination,
      PorterDuff::SourceOver,
      PorterDuff::DestinationOver,
      PorterDuff::SourceIn,
      PorterDuff::DestinationIn,
      PorterDuff::SourceOut,
      PorterDuff::DestinationOut,
      PorterDuff::SourceAtop,
      PorterDuff::DestinationAtop,
      PorterDuff::Xor,
      PorterDuff::Lighter,
    ]
    .iter()
    {
      let param = BlendAndComposite {
        sources: &sources,
        backdrops: &backdrops,
        mode: *mode,
        op: *op,
      };
      group.throughput(Throughput::Elements(len as u64));

      group.bench_with_input(BenchmarkId::from_parameter(param), &param, |b, m| {
        b.iter_batched_ref(
          || vec![Pixel(C::TRANSPARENT); len],
          |target| {
            batch_blend_with(
              black_box(m.sources),
              black_box(m.backdrops),
              black_box(target.as_mut_slice()),
              black_box(m.mode),
              black_box(m.op),
            )
          },
          BatchSize::SmallInput,
        );
      });
    }
  }

  group.finish();
}

criterion_group!(benches, bench_blend, bench_batch_blend);
criterion_main!(benches);
