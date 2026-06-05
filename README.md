# ternary-sort

Sorting algorithms inspired by ternary ({−1, 0, +1}) decomposition: counting sort for trits, Dutch National Flag quicksort, and base-3 radix sort.

---

## What Problem Does This Solve?

Sorting is the bedrock of algorithmics. This crate explores three algorithms where **ternary thinking** yields asymptotic or practical advantages:

1. **Ternary counting sort** — O(n) for data drawn from a 3-element alphabet.
2. **Dutch National Flag quicksort** — 3-way partitioning eliminates quadratic behaviour on duplicate-heavy input.
3. **Ternary radix sort** — Uses base-3 digit decomposition, trading a larger digit count for simpler bucket logic.

### Why Base 3?

In radix sort, running time is **O(d · (n + b))** where *d* is digit count and *b* is base (bucket count). For base *b*:
- Digit count: $d = \log_b U$ where *U* is the value range.
- Work per pass: O(n + b).

For 32-bit integers ($U = 2^{32}$):
- **Base 2:** $d = 32$ passes, $b = 2$ buckets.
- **Base 10:** $d \approx 10$ passes, $b = 10$ buckets.
- **Base 256:** $d = 4$ passes, $b = 256$ buckets.
- **Base 3:** $d \approx 21$ passes, $b = 3$ buckets.

Base 3 sits in an interesting middle ground: the bucket array is tiny (3 vectors), cache-friendly, and trivial to bounds-check. The extra passes are offset by extremely cheap per-element work. More importantly, **balanced ternary** (−1, 0, +1) is the natural alphabet for trit-based data structures, making this crate a companion to any ternary-tree or ternary-heap project.

---

## Mathematical Complexity Analysis

### Ternary Counting Sort

Input: *n* trits in {−1, 0, +1}.

$$T(n) = n \text{ (count)} + n \text{ (output)} = \Theta(n)$$

Space: Θ(n) for the output vector, Θ(1) additional (3 counters).

### Dutch National Flag Quicksort

Standard quicksort partitions into **two** regions (< pivot, ≥ pivot). With many duplicates, the recursion degenerates to O(n²). The **3-way partition** splits into (< pivot, = pivot, > pivot), collapsing the middle region into a single recursive call.

| Case | Standard Quicksort | **3-Way Quicksort** |
|------|-------------------|---------------------|
| All distinct | O(n log n) | O(n log n) |
| All equal | O(n²) | **O(n)** |
| *k* distinct keys | O(n²) | **O(n log k)** |

### Ternary Radix Sort (LSD)

After shifting values to be non-negative, each pass distributes *n* items into 3 buckets and concatenates:

$$T(n) = d \cdot \Theta(n + 3) = \Theta(n \cdot \log_3 U)$$

For 32-bit signed integers ($U \leq 2^{32}$):

$$T(n) = \Theta\left(n \cdot \frac{32}{\log_2 3}\right) \approx \Theta(20.2 \cdot n)$$

Space: Θ(n) working buffer + Θ(3) buckets.

---

## Architecture

### Ternary Counting Sort

```text
Input:  [ 1, -1, 0, 1, -1, 0, 1 ]

Count buckets (index = trit + 1):
  [-1] → 2
  [ 0] → 2
  [ 1] → 3

Output: [ -1, -1, 0, 0, 1, 1, 1 ]
```

### Dutch National Flag Partition

```text
Array during partitioning:

[ < pivot ][ = pivot ][ unexplored ][ > pivot ]
  ^         ^          ^            ^
  lt        eq         gt           last (pivot)

Invariant:
  arr[0 .. lt]   < pivot
  arr[lt .. eq]  = pivot
  arr[eq .. gt]  unexplored
  arr[gt ..]     > pivot
```

### Ternary Radix Sort (LSD)

```text
Digit extraction (base 3):
  value = 7  →  7 = 2·3⁰ + 1·3¹  →  digits [2, 1]

Pass 0 (3⁰): bucket by digit 0
Pass 1 (3¹): bucket by digit 1
...
```

Each pass is stable, so LSD radix sort preserves relative order of equal keys across passes.

---

## Getting Started

```rust
use ternary_sort::{
    ternary_counting_sort,
    ternary_quicksort,
    ternary_radix_sort,
    is_sorted,
};

fn main() {
    // 1. O(n) counting sort for {-1, 0, +1}
    let trits = vec![1i8, -1, 0, 1, -1, 0, 1];
    let sorted = ternary_counting_sort(&trits);
    assert_eq!(sorted, vec![-1, -1, 0, 0, 1, 1, 1]);

    // 2. 3-way quicksort (in-place)
    let mut data = vec![3, 1, 3, 2, 3, 1];
    ternary_quicksort(&mut data);
    assert!(is_sorted(&data));
    println!("{:?}", data); // [1, 1, 2, 3, 3, 3]

    // 3. Ternary radix sort (in-place)
    let mut nums = vec![9, 3, 7, 1, 5, 8, 2, 4, 6];
    ternary_radix_sort(&mut nums);
    assert!(is_sorted(&nums));
    println!("{:?}", nums); // [1, 2, 3, 4, 5, 6, 7, 8, 9]
}
```

---

## Running the Tests

Run the full suite with:

```bash
cargo test
```

There are **18 tests**, organised by algorithm:

### Ternary Counting Sort (5 tests)

| Test | What It Verifies |
|------|-----------------|
| `counting_sort_empty` | Empty input yields an empty vector. |
| `counting_sort_single` | Each singleton (−1, 0, +1) is returned unchanged. |
| `counting_sort_basic` | A mixed trit sequence is sorted correctly. |
| `counting_sort_all_same` | A run of identical trits is handled in O(n). |
| `counting_sort_all_neg` | A run of all −1 trits is handled in O(n). |

### Ternary Quicksort (8 tests)

| Test | What It Verifies |
|------|-----------------|
| `quicksort_empty` | Empty slice is a no-op. |
| `quicksort_single` | Singleton slice is untouched. |
| `quicksort_sorted_input` | Already-sorted data stays sorted (no degradation). |
| `quicksort_reverse_input` | Reverse-order data is sorted correctly. |
| `quicksort_with_duplicates` | 3-way partitioning handles moderate duplicates. |
| `quicksort_all_duplicates` | All-equal data is sorted in O(n), not O(n²). |
| `quicksort_negative_values` | Signed integers are partitioned correctly. |
| `quicksort_large_random` | 100 reverse-ordered elements are fully sorted. |

### Ternary Radix Sort (4 tests)

| Test | What It Verifies |
|------|-----------------|
| `radix_sort_empty` | Empty vector is a no-op. |
| `radix_sort_basic` | A 9-element positive sequence is sorted. |
| `radix_sort_negatives` | Negative values are shifted, sorted, and restored. |
| `radix_sort_large` | 200 reverse-ordered elements are fully sorted. |

### Utility (1 test)

| Test | What It Verifies |
|------|-----------------|
| `is_sorted_checks` | The `is_sorted` helper correctly identifies sorted, unsorted, empty, and singleton slices. |

---

## Related Crates

Explore the broader ternary ecosystem on crates.io:

- [`ternary-tree`](https://crates.io/crates/ternary-tree) — General-purpose ternary tree structures.
- [`ternary-compression`](https://crates.io/crates/ternary-compression) — Data compression using ternary alphabets.
- [`ternary-memory`](https://crates.io/crates/ternary-memory) — Ternary-addressable memory abstractions.
- [`ternary-tensor`](https://crates.io/crates/ternary-tensor) — Ternary-valued tensors for machine learning.

---

## License

MIT
