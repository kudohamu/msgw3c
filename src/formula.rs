/// Lum(C) = 0.3 x Cred + 0.59 x Cgreen + 0.11 x Cblue
#[inline]
pub(crate) fn lum(r: f32, g: f32, b: f32) -> f32 {
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
#[inline]
pub(crate) fn clip_color(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
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
#[inline]
pub(crate) fn set_lum(r: f32, g: f32, b: f32, l: f32) -> (f32, f32, f32) {
  let d = l - lum(r, g, b);
  let c_r = r + d;
  let c_g = g + d;
  let c_b = g + d;

  clip_color(c_r, c_g, c_b)
}

/// Sat(C) = max(Cred, Cgreen, Cblue) - min(Cred, Cgreen, Cblue)
#[inline]
pub(crate) fn sat(r: f32, g: f32, b: f32) -> f32 {
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
#[inline]
pub(crate) fn set_sat(r: f32, g: f32, b: f32, s: f32) -> (f32, f32, f32) {
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

/// B(Cb, Cs) = Cs
#[inline]
pub(crate) fn normal(_cb: f32, cs: f32) -> f32 {
  cs
}

/// B(Cb, Cs) = Cb x Cs
#[inline]
pub(crate) fn multiply(cb: f32, cs: f32) -> f32 {
  cb * cs
}

/// B(Cb, Cs) = 1 - [(1 - Cb) x (1 - Cs)]
///           = Cb + Cs -(Cb x Cs)
#[inline]
pub(crate) fn screen(cb: f32, cs: f32) -> f32 {
  cb + cs - (cb * cs)
}

/// B(Cb, Cs) = HardLight(Cs, Cb)
#[inline]
pub(crate) fn overlay(cb: f32, cs: f32) -> f32 {
  hard_light(cs, cb)
}

/// B(Cb, Cs) = min(Cb, Cs)
#[inline]
pub(crate) fn darken(cb: f32, cs: f32) -> f32 {
  cb.min(cs)
}

/// B(Cb, Cs) = max(Cb, Cs)
#[inline]
pub(crate) fn lighten(cb: f32, cs: f32) -> f32 {
  cb.max(cs)
}

/// if(Cb == 0)
///   B(Cb, Cs) = 0
/// else if(Cs == 1)
///   B(Cb, Cs) = 1
/// else
///   B(Cb, Cs) = min(1, Cb / (1 - Cs))
#[inline]
pub(crate) fn color_dodge(cb: f32, cs: f32) -> f32 {
  if cb == 0. {
    0.
  } else if cs == 1. {
    1.
  } else {
    (cb / (1. - cs)).min(1.)
  }
}

/// if(Cb == 1)
///   B(Cb, Cs) = 1
/// else if(Cs == 0)
///   B(Cb, Cs) = 0
/// else
///   B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
#[inline]
pub(crate) fn color_burn(cb: f32, cs: f32) -> f32 {
  if cb == 1. {
    1.
  } else if cs == 0. {
    0.
  } else {
    1. - ((1. - cb) / cs).min(1.)
  }
}

/// if(Cs <= 0.5)
///   B(Cb, Cs) = Multiply(Cb, 2 x Cs)
/// else
///   B(Cb, Cs) = Screen(Cb, 2 x Cs -1)
#[inline]
pub(crate) fn hard_light(cb: f32, cs: f32) -> f32 {
  if cs <= 0.5 {
    multiply(cb, 2. * cs)
  } else {
    screen(cb, 2. * cs - 1.)
  }
}

///   if(Cs <= 0.5)
///     B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
///   else
///     B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
/// with
///   if(Cb <= 0.25)
///     D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
///   else
///     D(Cb) = sqrt(Cb)
#[inline]
pub(crate) fn soft_light_d(cb: f32) -> f32 {
  if cb <= 0.25 {
    ((16. * cb - 12.) * cb + 4.) * cb
  } else {
    cb.sqrt()
  }
}

#[inline]
pub(crate) fn soft_light(cb: f32, cs: f32) -> f32 {
  if cs <= 0.5 {
    cb - (1. - 2. * cs) * cb * (1. - cb)
  } else {
    cb + (2. * cs - 1.) * (soft_light_d(cb) - cb)
  }
}

/// B(Cb, Cs) = | Cb - Cs |
#[inline]
pub(crate) fn difference(cb: f32, cs: f32) -> f32 {
  (cb - cs).abs()
}

/// B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
#[inline]
pub(crate) fn exclusion(cb: f32, cs: f32) -> f32 {
  cb + cs - 2. * cb * cs
}

/// B(Cb, Cs) = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
#[inline]
pub(crate) fn hue(cb: (f32, f32, f32), cs: (f32, f32, f32)) -> (f32, f32, f32) {
  let (r, g, b) = set_sat(cs.0, cs.1, cs.2, sat(cb.0, cb.1, cb.2));
  set_lum(r, g, b, lum(cb.0, cb.1, cb.2))
}

/// B(Cb, Cs) = SetLum(SetSat(Cb, Sat(Cs)), Lum(Cb))
#[inline]
pub(crate) fn saturation(cb: (f32, f32, f32), cs: (f32, f32, f32)) -> (f32, f32, f32) {
  let (r, g, b) = set_sat(cb.0, cb.1, cb.2, sat(cs.0, cs.1, cs.2));
  set_lum(r, g, b, lum(cb.0, cb.1, cb.2))
}

/// B(Cb, Cs) = SetLum(Cs, Lum(Cb))
#[inline]
pub(crate) fn color(cb: (f32, f32, f32), cs: (f32, f32, f32)) -> (f32, f32, f32) {
  set_lum(cs.0, cs.1, cs.2, lum(cb.0, cb.1, cb.2))
}

/// B(Cb, Cs) = SetLum(Cb, Lum(Cs))
#[inline]
pub(crate) fn luminosity(cb: (f32, f32, f32), cs: (f32, f32, f32)) -> (f32, f32, f32) {
  set_lum(cb.0, cb.1, cb.2, lum(cs.0, cs.1, cs.2))
}
