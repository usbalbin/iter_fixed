# iter_fixed
Provides a trait for turning collections of fixed size, like arrays, into `IteratorFixed` which can be used a bit
like an ordinary `Iterator` but with a compile time guaranteed length. This enables us to turn them back into collections
of fixed size without having to perform unnecessary checks during run time.

Just like `Iterator`, `IteratorFixed` provides adapters like:

* `map`
* `inspect`
* `skip`
* `step_by`
* `chain`
* `enumerate`
* `take`
* `zip`
* `rev`
* `copied`
* `cloned`

however it does not support adapter like `filter` or `take_while` which will affect the length during runtime.

# Examples

## Toy example

```
// zip together two arrays of length 4, turn the elements wise sums of the
// two middle elements into an array of size 2
let res: [_; 2] = [1u32, 2, 3, 4]
    .into_iter_fixed()
    .zip([4u32, 3, 2, 1])
    .map(|(a, b)| a + b)
    .skip::<1>()
    .take::<2>()
    .collect();

assert_eq!(res, [5, 5]);
```

## Vector
see examples/vector.rs

## Matrix

see examples/matrix.rs
