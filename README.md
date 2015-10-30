Use Arbitrarily dimensioned arrays in rust.

Note: Although I've tried to keep implementations as fast as possible, constructors are O(d^2) and
indexing is O(d) where d is the number of dimensions, and n is the sum of the magnitudes of all
dimensions.

Usage
=====


```rust
// Create a 4d array of u32s.
// where the dimensioning is 5x3x9x2
let mut my_array: NArray<u32> = NArray::new(4, &[5,3,9,2]);

// Set some arbitrary point (getting is the same, but no `= 9`)
my_array[&[3, 1, 5, 1]] = 9;

// Create a 3d array of u32s where each index
// value is the sum of it's index coordinates
// with dimension 5x8x11.
let my_fancy_array: NArray<u32> = NArray::from_function(3, &[5, 8, 11], |n: &[usize]| {
    n.to_vec().iter().fold(1, |acc, &item| acc*item as u32)
});
assert_eq!(my_fancy_array[&[3, 4, 9]], 108);

```
