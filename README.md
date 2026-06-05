# ternary-sort

**Sorting algorithms optimized for ternary {-1, 0, +1} data: Dutch National Flag, counting sort, and radix sort on trits.**

When your data only has 3 distinct values, you don't need general-purpose sorting. **Dutch National Flag** (Dijkstra) partitions an array into {-1 | 0 | +1} in a single O(n) pass with O(1) extra space. It's provably optimal — you cannot do better than looking at each element once.

---

## Algorithms

### Dutch National Flag (ternary_quicksort)

Dijkstra's 3-way partition: maintain four regions [<-1 | ?unknown | 0 | >+1] with three pointers:

```
[  -1's  |  unknown  |  0's  |  +1's  ]
   ^         ^^         ^        ^
   lo       mid,i      mid      hi
```

Each step: look at element[i]. If -1, swap with lo. If +1, swap with hi. If 0, skip. O(n) total, O(1) space.

### Ternary Counting Sort

For arrays of pure {-1, 0, +1}: count occurrences of each value, then emit them in order. O(n) time, O(1) space (only 3 counters).

### Ternary Radix Sort

Sort integers by their balanced ternary digits (trits), least-significant first. Each pass is a 3-way stable partition. Total passes = log₃(max_value), which is 37% fewer passes than binary radix sort.

---

## Architecture

- **`ternary_counting_sort()`** — O(n) for pure {-1, 0, +1} arrays
- **`ternary_quicksort()`** — Dutch National Flag partition (works on any Ord type)
- **`ternary_radix_sort()`** — Radix sort by balanced ternary digits
- **`is_sorted()`** — Verify ternary array ordering

---

## Quick Start

```rust
use ternary_sort::{ternary_counting_sort, ternary_quicksort, Trit};

// Pure ternary data — O(n) counting sort
let data: Vec<Trit> = vec![1, -1, 0, 1, -1, 0, 0, 1, -1];
let sorted = ternary_counting_sort(&data);
assert_eq!(sorted, vec![-1, -1, -1, 0, 0, 0, 1, 1, 1]);

// General sorted data — DNF partition
let mut arr = vec![3, 1, 2, 1, 3, 2, 1];
ternary_quicksort(&mut arr);
assert!(is_sorted(&arr));
```

---

## Ecosystem

- **ternary-heap** — Ternary priority queue (related ordered structure)
- **ternary-btree** — Ternary B-tree (ordered map)
- **ternary-search** — Search algorithms for ternary data
- **ternary-flux** — Stream processing with ternary signals

## License
MIT
