pub const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub const fn sub_or_zero(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

pub const fn ceiling_div(x: usize, d: usize) -> usize {
    x / d + (x % d != 0) as usize
}
