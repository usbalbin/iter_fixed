#[cfg(feature = "nightly_features")]
pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(feature = "nightly_features")]
pub const fn ceiling_div(x: usize, d: usize) -> usize {
    x / d + (x % d != 0) as usize
}
