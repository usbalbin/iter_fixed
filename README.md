![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

[![crates.io](https://img.shields.io/crates/v/iter_fixed.svg)](https://crates.io/crates/iter_fixed)
[![docs.rs](https://docs.rs/iter_fixed/badge.svg)](https://docs.rs/iter_fixed/)
[![dependency status](https://deps.rs/crate/iter_fixed/0.3.0/status.svg)](https://deps.rs/crate/iter_fixed/0.3.0)

![Stable](https://github.com/usbalbin/iter_fixed/actions/workflows/stable.yml/badge.svg)
![Nightly](https://github.com/usbalbin/iter_fixed/actions/workflows/nightly.yml/badge.svg)
![Miri](https://github.com/usbalbin/iter_fixed/actions/workflows/miri.yml/badge.svg)

# iter_fixed

Provides a type and traits for turning collections of fixed size, like arrays,
into [`IteratorFixed`] which can be used a bit like an ordinary [`Iterator`] but
with a compile time guaranteed length. This enables us to turn them back into
collections of fixed size without having to perform unnecessary checks during
run time.

[`IteratorFixed`] provides on stable methods like `map`, `inspect`, `enumerate`,
 `zip`, `rev`, `copied`, `cloned`, with nightly `skip`, `step_by`, `chain`, `take`,
 `flatten`.

However it does not and will never be able to support methods like
 `filter` or `take_while` which will affect the length during runtime.

## ⚠️ Experimental
*This code is currently very experimental, type names, function names, trait bounds etc. are all very much subject to change.*

## Origin
*This project is inspired by @leonardo-m 's idea <https://github.com/rust-lang/rust/issues/80094#issuecomment-749260428>*

## Examples:
```rust
// simply reverse an Array
let rev_array: [_; 4] = [1, 3, 2, 7]
    .into_iter_fixed()
    .rev()
    .collect();
assert_eq!(rev_array, [7, 2, 3, 1]);

// .. and compute sum with values of a second array multiplied by 10
let sum_array: [_; 4] = rev_array
    .into_iter_fixed()
    .zip([4,1,3,7])
    .map(|(a, b)| a + (b * 10))
    .collect();
assert_eq!(sum_array, [47, 12, 33, 71]);
```

You can also take a look at examples : [`matrix.rs`] and [`vector.rs`]

[`matrix.rs`]: source/examples/matrix.rs
[`vector.rs`]: source/examples/vector.rs


Current version: 0.4.0

## no_std

This crate should work without the full standard library

Some additional info here

# License : MIT OR Apache-2.0
`iter_fixed` is distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in iter_fixed by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
