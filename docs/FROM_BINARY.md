# From Binary to Ternary: Sorting

## The Trap

Binary sorting tells you to partition into two piles: less-than or greater-than. This is the **Dutch National Flag problem** in disguise — when your data has duplicates, a binary partition degenerates into O(n²) because the equal-to-pivot case gets scattered across both sides. You reach for fancy pivot selection, median-of-medians, or introsort heuristics to work around a fundamental encoding problem.

The deeper trap: you've been sorting elements that *could* be in {−1, 0, +1} but your algorithm doesn't know it. Counting sort over a binary alphabet wastes half your bucket array. Radix sort with base 2 takes 32 passes. You're paying for generality the problem doesn't need.

## Map to Three States

| Domain | −1 | 0 | +1 |
|--------|----|---|-----|
| Trit values | −1 | 0 | +1 |
| Quicksort partition | `< pivot` | `= pivot` | `> pivot` |
| Radix digit | digit = 0 | digit = 1 | digit = 2 |

## From Binary to Ternary

**Before: binary counting sort (wasted capacity)**

```rust
fn counting_sort_binary(input: &[i8]) -> Vec<i8> {
    let mut counts = [0i32; 2]; // only 2 buckets
    // ... but our data is {-1, 0, +1}
    // We'd need to shift values and still lose the 0 bucket
}
```

You map three values into two buckets. Something always gets squashed.

**After: ternary counting sort**

```rust
fn ternary_counting_sort(trits: &[i8]) -> Vec<i8> {
    let mut counts = [0i32; 3]; // one bucket per trit
    for &t in trits {
        counts[(t + 1) as usize] += 1; // map -1→0, 0→1, 1→2
    }
    // counts[-1], counts[0], counts[1] — three natural buckets
    // Reconstruct in O(n)
}
```

Every trit maps directly to its bucket. No shifting, no collisions, no wasted encoding.

**Before: binary quicksort on duplicates → O(n²)**

```rust
// Standard 2-way partition
// All elements equal to pivot get scattered
// Recursion hits O(n²) on duplicate-rich data
```

**After: 3-way quicksort → O(n)**

```rust
// Dutch National Flag 3-way partition
// arr[0..lt] < pivot
// arr[lt..eq] = pivot  ← collapsed, no recursion here
// arr[gt..] > pivot
// All-equal input: one pass, done. O(n).
```

**The 0 is not nothing:** In ternary radix sort, the neutral bucket (digit = 0) is as important as the positive and negative buckets. It's not an empty gap waiting to be filled — it's a genuine partition. The ternary conservation law applies: each pass moves every element to exactly one of three buckets, and the sum of bucket sizes equals n.

## Why It Matters

Ternary sorting matches the natural structure of trit-valued data. Counting sort becomes O(n) with zero waste. Quicksort on duplicate-heavy data goes from O(n²) to O(n). Radix sort uses tiny, cache-friendly 3-element buckets. The algorithms are simpler, faster, and provably optimal for ternary alphabets — no heuristics, no workarounds, just the right encoding for the problem.
