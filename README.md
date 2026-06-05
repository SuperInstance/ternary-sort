# ternary-sort

Sorting algorithms for ternary and general data.

## Algorithms
- `ternary_counting_sort` — O(n) for arrays of trits in {-1, 0, +1}
- `ternary_quicksort` — 3-way Dutch National Flag partition; excellent for data with many duplicates
- `ternary_radix_sort` — LSD radix sort using base-3 digit decomposition for i32
- `is_sorted` — O(n) sortedness check

## Usage
```rust
let mut v = vec![3, 1, 3, 2, 3, 1];
ternary_quicksort(&mut v);
assert_eq!(v, vec![1, 1, 2, 3, 3, 3]);

let trits = vec![1i8, -1, 0, 1, -1];
let sorted = ternary_counting_sort(&trits);
assert_eq!(sorted, vec![-1, -1, 0, 1, 1]);
```
