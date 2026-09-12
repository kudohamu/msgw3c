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
  let x = r.max(g).max(b);

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
  let c_b = b + d;

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
  let mut min = (r, 0);
  let mut mid = (g, 1);
  let mut max = (b, 2);

  if min.0 > mid.0 {
    std::mem::swap(&mut min, &mut mid);
  }
  if mid.0 > max.0 {
    std::mem::swap(&mut mid, &mut max);
  }
  if min.0 > mid.0 {
    std::mem::swap(&mut min, &mut mid);
  }

  let mut rgb = [0.; 3];
  if max.0 > min.0 {
    rgb[mid.1] = ((mid.0 - min.0) * s) / (max.0 - min.0);
    rgb[max.1] = s;
  }

  (rgb[0], rgb[1], rgb[2])
}

/// B(Cb, Cs) = Cs
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x cs / αs
///   = αb x cs
#[inline]
pub(crate) fn normal(_cb: f32, cs: f32, a_b: f32, _a_s: f32) -> f32 {
  a_b * cs
}

/// B(Cb, Cs) = Cb x Cs
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x cb / αb x cs / αs
///   = αs x αb x cb x cs / (αs x αb)
///   = cb x cs
#[inline]
pub(crate) fn multiply(cb: f32, cs: f32, _a_b: f32, _a_s: f32) -> f32 {
  cb * cs
}

/// B(Cb, Cs) = 1 - [(1 - Cb) x (1 - Cs)]
///           = Cb + Cs -(Cb x Cs)
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x (cb / αb + cs / αs - (cb / αb x cs / αs))
///   = αs x cb + αb x cs - cb x cs
#[inline]
pub(crate) fn screen(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  a_s * cb + a_b * cs - cb * cs
}

/// B(Cb, Cs) = HardLight(Cs, Cb)
#[inline]
pub(crate) fn overlay(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  hard_light(cs, cb, a_s, a_b)
}

/// B(Cb, Cs) = min(Cb, Cs)
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x min(cb / αb, cs / αs)
/// 　　= min(cb x αs, cs x αb)
#[inline]
pub(crate) fn darken(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  (cb * a_s).min(cs * a_b)
}

/// B(Cb, Cs) = max(Cb, Cs)
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x max(cb / αb, cs / αs)
///   = max(cb x αs, cs x αb)
#[inline]
pub(crate) fn lighten(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  (cb * a_s).max(cs * a_b)
}

/// if(Cb == 0)
///   B(Cb, Cs) = 0
/// else if(Cs == 1)
///   B(Cb, Cs) = 1
/// else
///   B(Cb, Cs) = min(1, Cb / (1 - Cs))
/// ⇒
/// if(cb / αb == 0)
///   B(Cb, Cs) = 0
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = 0
/// else if(cs / αs == 1)
///   B(Cb, Cs) = 1
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = αs x αb
/// else
///   B(Cb, Cs) = min(1, Cb / (1 - Cs))
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = αs x αb x min(1, cb / αb / (1 - cs / αs))
///   K = αs x αb x min(1, cb / αb x 1 / (1 - cs / αs))
///   K = αs x αb x min(1, cb / (αb x (1 - cs / αs)))
///   K = min(αs x αb, αs x αb x cb / (αb x (1 - cs / αs)))
///   K = min(αs x αb, αs x cb / (1 - cs / αs))
///   K = min(αs x αb, αs x αs x cb / (αs - cs))
#[inline]
pub(crate) fn color_dodge(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  if cb == 0. {
    0.
  } else if cs == a_s {
    a_s * a_b
  } else {
    (a_s * a_b).min((a_s * a_s * cb) / (a_s - cs))
  }
}

/// if(Cb == 1)
///   B(Cb, Cs) = 1
/// else if(Cs == 0)
///   B(Cb, Cs) = 0
/// else
///   B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
/// ⇒
/// if(cb / αb == 1)
///   B(Cb, Cs) = 1
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = αs x αb
/// else if(cs / αs == 0)
///   B(Cb, Cs) = 0
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = 0
/// else
///   B(Cb, Cs) = 1 - min(1, (1 - Cb) / Cs)
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = αs x αb x (1 - min(1, (1 - cb / αb) / (cs / αs)))
///   K = αs x αb x (1 - min(1, (1 - cb / αb) x (αs / cs)))
///   K = αs x αb x (1 - min(1, ((1 - cb / αb) x αs) / cs))
///   K = αs x αb x (1 - min(1, (αs - cb x αs / αb) / cs))
///   K = αs x αb x (1 - min(1, ((αs x αb - cb x αs) / αb) / cs))
///   K = αs x αb x (1 - min(1, (αs x αb - cb x αs) / (αb x cs))
/// 　　K = αs x αb - min(αs x αb, αs x αb x (αs x αb - cb x αs) / (αb x cs))
///   K = αs x αb - min(αs x αb, αs x (αs x αb - cb x αs) / cs)
///   K = αs x αb - min(αs x αb, (αs x αs x αb - cb x αs x αs) / cs)
///   K = max(0, αs x αb - (αs x αs x αb - cb x αs x αs) / cs)
#[inline]
pub(crate) fn color_burn(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  if cb == a_b {
    a_s * a_b
  } else if cs == 0. {
    0.
  } else {
    (a_s * a_b - (a_s * a_s * a_b - cb * a_s * a_s) / cs).max(0.)
  }
}

/// if(Cs <= 0.5)
///   B(Cb, Cs) = Multiply(Cb, 2 x Cs)
/// else
///   B(Cb, Cs) = Screen(Cb, 2 x Cs -1)
/// ⇒
/// if(cs / αs <= 0.5)
///   B(Cb, Cs) = Multiply(Cb, 2 x Cs)
///             = Multiply(Cb, 2 x cs / αs)
/// else
///   B(Cb, Cs) = Screen(Cb, 2 x Cs -1)
///             = Screen(Cb, 2 x cs / αs -1)
///             = Screen(Cb, (2 x cs - αs) / αs)
#[inline]
pub(crate) fn hard_light(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  if 2.0 * cs <= a_s {
    multiply(cb, 2. * cs, a_b, a_s)
  } else {
    screen(cb, 2. * cs - a_s, a_b, a_s)
  }
}

/// if(Cb <= 0.25)
///   D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
/// else
///   D(Cb) = sqrt(Cb)
/// ⇒
/// if(cb / αb <= 0.25)
///   D(Cb) = ((16 * Cb - 12) x Cb + 4) x Cb
///         = ((16 * cb / αb - 12) x cb / αb + 4) x cb / αb
/// else
///   D(Cb) = sqrt(Cb)
/// 　　　　　　　　= sqrt(cb / αb)
///
/// ===
///
/// if(Cs <= 0.5)
///   B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
/// else
///   B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
/// ⇒
/// if(cs / αs <= 0.5)
///   B(Cb, Cs) = Cb - (1 - 2 x Cs) x Cb x (1 - Cb)
///   K = αs x αb x B(cb / αb, cs / αs)
///     = αs x αb x (cb / αb - (1 - 2 x cs / αs) x cb / αb x (1 - cb / αb))
///     = (as x cb - (1 - 2 x cs / αs) x cb x αs x (1 - cb / αb))
///     = (as x cb - (αs - 2 x cs) / αs x cb x αs x (1 - cb / αb))
///     = (as x cb - (αs - 2 x cs) x cb x (1 - cb / αb))
/// else
///   B(Cb, Cs) = Cb + (2 x Cs - 1) x (D(Cb) - Cb)
///   K = αs x αb x B(cb / αb, cs / αs)
///   K = αs x αb x (cb / αb + (2 x cs / αs - 1) x (D(cb / αb) - cb / αb))
///   K = αs x αb x (cb / αb + (2 x cs - αs) / αs x (αb x D(cb / αb) - cb) / αb)
///   K = cb x αs + (2 x cs - αs) x (αb x D(cb / αb) - cb)
///   def: d = αb x D(Cb)
///   K = cb x αs + (2 x cs - αs) x (d - cb)
///
///   if(cb / αb <= 0.25)
///     d = αb x D(Cb)
///       = αb x ((16 * cb / αb - 12) x cb / αb + 4) x cb / αb
///       = ((16 * cb / αb - 12) x cb / αb + 4) x cb
///   else
///     d = αb x D(Cb)
///       = αb x sqrt(cb / αb)
///       = sqrt(αb x αb x cb / αb)
///       = sqrt(αb x cb)
#[inline]
pub(crate) fn soft_light(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  if a_b == 0.0 {
    return 0.0;
  }

  if 2.0 * cs <= a_s {
    a_s * cb - (a_s - 2. * cs) * cb * (1. - cb / a_b)
  } else {
    let d = if 4. * cb <= a_b {
      ((16. * cb / a_b - 12.) * cb / a_b + 4.) * cb
    } else {
      (a_b * cb).sqrt()
    };
    cb * a_s + (2. * cs - a_s) * (d - cb)
  }
}

/// B(Cb, Cs) = | Cb - Cs |
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x | cb / αb - cs / αs |
///   = | cb x αs - cs x αb |
#[inline]
pub(crate) fn difference(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  (cb * a_s - cs * a_b).abs()
}

/// B(Cb, Cs) = Cb + Cs - 2 x Cb x Cs
/// K = αs x αb x B(cb / αb, cs / αs)
///   = αs x αb x (cb / αb + cs / αs - 2 x cb / αb x cs / αs)
///   = cb x αs + cs x αb - 2 x cb x cs
#[inline]
pub(crate) fn exclusion(cb: f32, cs: f32, a_b: f32, a_s: f32) -> f32 {
  cb * a_s + cs * a_b - 2. * cb * cs
}

/// B(Cb, Cs) = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
/// K = αs x αb x B(cb / αb, cs / αs)
///   = SetLum(SetSat(Cs, Sat(Cb)), Lum(Cb))
/// NOTE: args of cb and cS are straight alpha value.
#[inline]
pub(crate) fn hue(cb: (f32, f32, f32), cs: (f32, f32, f32), a_b: f32, a_s: f32) -> (f32, f32, f32) {
  let (r, g, b) = set_sat(cs.0, cs.1, cs.2, sat(cb.0, cb.1, cb.2));
  let (r, g, b) = set_lum(r, g, b, lum(cb.0, cb.1, cb.2));

  (a_s * a_b * r, a_s * a_b * g, a_s * a_b * b)
}

/// B(Cb, Cs) = SetLum(SetSat(Cb, Sat(Cs)), Lum(Cb))
/// K = αs x αb x B(cb / αb, cs / αs)
/// NOTE: args of cb and cS are straight alpha value.
#[inline]
pub(crate) fn saturation(
  cb: (f32, f32, f32),
  cs: (f32, f32, f32),
  a_b: f32,
  a_s: f32,
) -> (f32, f32, f32) {
  let (r, g, b) = set_sat(cb.0, cb.1, cb.2, sat(cs.0, cs.1, cs.2));
  let (r, g, b) = set_lum(r, g, b, lum(cb.0, cb.1, cb.2));

  (a_s * a_b * r, a_s * a_b * g, a_s * a_b * b)
}

/// B(Cb, Cs) = SetLum(Cs, Lum(Cb))
/// K = αs x αb x B(cb / αb, cs / αs)
/// NOTE: args of cb and cS are straight alpha value.
#[inline]
pub(crate) fn color(
  cb: (f32, f32, f32),
  cs: (f32, f32, f32),
  a_b: f32,
  a_s: f32,
) -> (f32, f32, f32) {
  let (r, g, b) = set_lum(cs.0, cs.1, cs.2, lum(cb.0, cb.1, cb.2));

  (a_s * a_b * r, a_s * a_b * g, a_s * a_b * b)
}

/// B(Cb, Cs) = SetLum(Cb, Lum(Cs))
/// K = αs x αb x B(cb / αb, cs / αs)
/// NOTE: args of cb and cS are straight alpha value.
#[inline]
pub(crate) fn luminosity(
  cb: (f32, f32, f32),
  cs: (f32, f32, f32),
  a_b: f32,
  a_s: f32,
) -> (f32, f32, f32) {
  let (r, g, b) = set_lum(cb.0, cb.1, cb.2, lum(cs.0, cs.1, cs.2));

  (a_s * a_b * r, a_s * a_b * g, a_s * a_b * b)
}
