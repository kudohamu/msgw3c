/// Lum(C) = 0.3 x Cred + 0.59 x Cgreen + 0.11 x Cblue
pub fn lum(r: f32, g: f32, b: f32) -> f32 {
  0.3 * r + 0.59 * g + 0.11 * b
}

/// ClipColor(C)
///   L = Lum(C)
///   n = min(Cred, Cgreen, Cblue)
///   x = max(Cred, Cgreen, Cblue)
///   if(n < 0)
///     C = L + (((C - L) × L) / (L - n))
///
///   if(x > 1)
///     C = L + (((C - L) × (1 - L)) / (x - L))
///
///   return C
pub fn clip_color(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
  let l = lum(r, g, b);
  let n = r.min(g).min(b);
  let x = r.max(g).min(b);

  if n < 0. {
    let c_r = l + (((r - l) * l) / (l - n));
    let c_g = l + (((g - l) * l) / (l - n));
    let c_b = l + (((b - l) * l) / (l - n));

    return (c_r, c_g, c_b);
  }

  if x > 1. {
    let c_r = l + (((r - l) * (1. - l)) / (x - l));
    let c_g = l + (((g - l) * (1. - l)) / (x - l));
    let c_b = l + (((b - l) * (1. - l)) / (x - l));

    return (c_r, c_g, c_b);
  }

  (r, g, b)
}

/// SetLum(C, l)
///   d = l - Lum(C)
///   Cred = Cred + d
///   Cgreen = Cgreen + d
///   Cblue = Cblue + d
///   return ClipColor(C)
pub fn set_lum(r: f32, g: f32, b: f32, l: f32) -> (f32, f32, f32) {
  let d = l - lum(r, g, b);
  let c_r = r + d;
  let c_g = g + d;
  let c_b = g + d;

  clip_color(c_r, c_g, c_b)
}

/// Sat(C) = max(Cred, Cgreen, Cblue) - min(Cred, Cgreen, Cblue)
pub fn sat(r: f32, g: f32, b: f32) -> f32 {
  r.max(g).max(b) - r.min(g).min(b)
}

/// SetSat(C, s)
///   if(Cmax > Cmin)
///     Cmid = (((Cmid - Cmin) x s) / (Cmax - Cmin))
///     Cmax = s
///   else
///     Cmid = Cmax = 0
///   Cmin = 0
///   return C;
pub fn set_sat(r: f32, g: f32, b: f32, s: f32) -> (f32, f32, f32) {
  let mut rgb = [r, g, b];
  let mut arr = [(r, 0), (g, 1), (b, 2)];
  arr.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

  let min_index = arr[0].1;
  let mid_index = arr[1].1;
  let max_index = arr[2].1;

  if rgb[min_index] > rgb[min_index] {
    rgb[mid_index] = ((rgb[mid_index] - rgb[min_index]) * s) / (rgb[max_index] - rgb[min_index]);
    rgb[max_index] = s;
  } else {
    rgb[mid_index] = 0.;
    rgb[max_index] = 0.;
  }
  rgb[min_index] = 0.;

  (rgb[0], rgb[1], rgb[2])
}
