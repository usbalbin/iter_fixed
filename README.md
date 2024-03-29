# iter_fixed

[![crates.io](https://img.shields.io/crates/v/iter_fixed.svg)](https://crates.io/crates/iter_fixed)
[![docs.rs](https://docs.rs/iter_fixed/badge.svg)](https://docs.rs/iter_fixed/)
[![dependency status](https://deps.rs/crate/iter_fixed/0.3.0/status.svg)](https://deps.rs/crate/iter_fixed/0.3.0)

![Stable](https://github.com/usbalbin/iter_fixed/actions/workflows/stable.yml/badge.svg)
![Nightly](https://github.com/usbalbin/iter_fixed/actions/workflows/nightly.yml/badge.svg)
![Miri](https://github.com/usbalbin/iter_fixed/actions/workflows/miri.yml/badge.svg)

*This project is inspired by @leonardo-m 's idea https://github.com/rust-lang/rust/issues/80094#issuecomment-749260428*

**This code is currently very experimental, type names, function names, trait bounds etc. are all very much subject to change. 

Provides a type and traits for turning collections of fixed size, like arrays,
into `IteratorFixed` which can be used a bit like an ordinary `Iterator` but
with a compile time guaranteed length. This enables us to turn them back into
collections of fixed size without having to perform unnecessary checks during
run time.

Just like [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html), [`IteratorFixed`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html) provides methods like:

###### Works on stable rust
* [`map`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.map)
* [`inspect`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.inspect)
* [`enumerate`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.enumerate)
* [`zip`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.zip)
* [`rev`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.rev)
* [`copied`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.copied)
* [`cloned`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.cloned)

###### Requires nightly compiler
* [`skip`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.skip)
* [`step_by`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.step_by)
* [`chain`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.chain)
* [`take`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.take)
* [`flatten`](https://docs.rs/iter_fixed/latest/iter_fixed/struct.IteratorFixed.html#method.flatten)

however it does not and will never be able to support methods like `filter` or `take_while` which will affect the length during runtime.

## no_std

This crate should work without the full standard library

# Examples

## Toy example

```rust
// zip together two arrays of length 4, turn the elements wise sums of the
// two middle elements into an array of size 2
let res: [_; 2] = [1, 2, 3, 4]
    .into_iter_fixed()
    .zip([4, 3, 2, 1])
    .map(|(a, b)| a + b)
    .skip::<1>()
    .take::<2>()
    .collect();

assert_eq!(res, [5, 5]);
```

## Vector
see [examples/vector.rs](https://github.com/usbalbin/iter_fixed/blob/master/examples/vector.rs)

## Matrix

see [examples/matrix.rs](https://github.com/usbalbin/iter_fixed/blob/master/examples/matrix.rs)

# License
`iter_fixed` is distributed under the terms of both the MIT license and
the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in iter_fixed by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
