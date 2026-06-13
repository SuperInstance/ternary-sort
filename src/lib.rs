//! Sorting algorithms for ternary ({-1, 0, +1}) and general data.
//!
//! * `ternary_counting_sort` — O(n) for arrays of trits in {-1, 0, +1}
//! * `ternary_quicksort`     — 3-way Dutch National Flag partition; ideal for
//!                             data with many duplicates, O(n log n) average
//! * `ternary_radix_sort`    — MSD radix sort using balanced-ternary digit
//!                             decomposition (base 3)

/// Canonical ternary type — re-exported from [ternary-types](https://github.com/SuperInstance/ternary-types).
pub use ternary_types::Ternary;

/// Deprecated: use [`Ternary`] instead.
#[deprecated(since = "0.2.0", note = "use ternary_types::Ternary instead")]
pub type Trit = i8; // must be -1, 0, or +1

/// O(n) counting sort for slices containing only values in {-1, 0, +1}.
/// Returns a new sorted Vec.
pub fn ternary_counting_sort(input: &[Trit]) -> Vec<Trit> {
    let mut count = [0usize; 3]; // index: 0→-1, 1→0, 2→+1
    for &t in input {
        count[(t + 1) as usize] += 1;
    }
    let mut out = Vec::with_capacity(input.len());
    for &v in &[-1i8, 0, 1] {
        for _ in 0..count[(v + 1) as usize] {
            out.push(v);
        }
    }
    out
}

/// 3-way Dutch National Flag quicksort on an `Ord` slice (in-place).
/// Partitions into lt / eq / gt around a pivot.
pub fn ternary_quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let (lt, gt) = dnf_partition(arr);
    ternary_quicksort(&mut arr[..lt]);
    ternary_quicksort(&mut arr[gt..]);
}

/// Returns (lt, gt) such that arr[..lt] < pivot, arr[lt..gt] == pivot, arr[gt..] > pivot.
fn dnf_partition<T: Ord>(arr: &mut [T]) -> (usize, usize) {
    // Use median-of-three pivot
    let n = arr.len();
    let mid = n / 2;
    let last = n - 1;
    if arr[0] > arr[mid] { arr.swap(0, mid); }
    if arr[0] > arr[last] { arr.swap(0, last); }
    if arr[mid] > arr[last] { arr.swap(mid, last); }
    arr.swap(mid, last - 1);
    // pivot is now at last-1 unless n < 3
    let pivot_idx = if n >= 3 { last - 1 } else { 0 };
    // Move pivot to end temporarily
    arr.swap(pivot_idx, last);

    let mut lt = 0;
    let mut eq = 0;
    let mut gt = last; // points to pivot at arr[last]

    while eq < gt {
        match arr[eq].cmp(&arr[last]) {
            std::cmp::Ordering::Less => {
                arr.swap(lt, eq);
                lt += 1;
                eq += 1;
            }
            std::cmp::Ordering::Equal => {
                eq += 1;
            }
            std::cmp::Ordering::Greater => {
                gt -= 1;
                arr.swap(eq, gt);
            }
        }
    }
    // Place pivot
    arr.swap(eq, last);
    (lt, eq + 1)
}

/// Radix sort for i32 values using base-3 (balanced ternary) decomposition.
/// Sorts in-place using LSD radix sort with 3 buckets per pass.
pub fn ternary_radix_sort(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }
    // Shift all values so they are non-negative for digit extraction
    let min_val = *arr.iter().min().unwrap();
    let max_val = *arr.iter().max().unwrap();
    let range = (max_val - min_val) as u64;

    if range == 0 {
        return; // all equal
    }

    // Number of base-3 digits needed
    let digits = if range > 0 {
        let mut d = 1u32;
        let mut p = 3u64;
        while p <= range {
            d += 1;
            p *= 3;
        }
        d
    } else {
        1
    };

    // Convert to shifted non-negative values
    let mut working: Vec<u64> = arr.iter().map(|&x| (x as i64 - min_val as i64) as u64).collect();

    for digit in 0..digits {
        let base = 3u64.pow(digit);
        let mut buckets: [Vec<u64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for &v in working.iter() {
            let trit = ((v / base) % 3) as usize;
            buckets[trit].push(v);
        }
        working.clear();
        for bucket in &buckets {
            working.extend_from_slice(bucket);
        }
    }

    // Convert back
    for (dst, src) in arr.iter_mut().zip(working.iter()) {
        *dst = (*src as i64 + min_val as i64) as i32;
    }
}

/// Returns true if `arr` is non-decreasingly sorted.
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- ternary_counting_sort ---

    #[test]
    fn counting_sort_empty() {
        assert_eq!(ternary_counting_sort(&[]), vec![]);
    }

    #[test]
    fn counting_sort_single() {
        assert_eq!(ternary_counting_sort(&[1]), vec![1]);
        assert_eq!(ternary_counting_sort(&[-1]), vec![-1]);
        assert_eq!(ternary_counting_sort(&[0]), vec![0]);
    }

    #[test]
    fn counting_sort_basic() {
        let input = vec![1i8, -1, 0, 1, -1, 0, 1];
        let sorted = ternary_counting_sort(&input);
        assert_eq!(sorted, vec![-1, -1, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn counting_sort_all_same() {
        let input = vec![0i8; 5];
        assert_eq!(ternary_counting_sort(&input), vec![0; 5]);
    }

    #[test]
    fn counting_sort_all_neg() {
        let input = vec![-1i8; 4];
        assert_eq!(ternary_counting_sort(&input), vec![-1; 4]);
    }

    // --- ternary_quicksort ---

    #[test]
    fn quicksort_empty() {
        let mut v: Vec<i32> = vec![];
        ternary_quicksort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn quicksort_single() {
        let mut v = vec![42];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn quicksort_sorted_input() {
        let mut v = vec![1, 2, 3, 4, 5];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn quicksort_reverse_input() {
        let mut v = vec![5, 4, 3, 2, 1];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn quicksort_with_duplicates() {
        let mut v = vec![3, 1, 3, 2, 3, 1];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 3, 3]);
    }

    #[test]
    fn quicksort_all_duplicates() {
        let mut v = vec![7; 10];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![7; 10]);
    }

    #[test]
    fn quicksort_negative_values() {
        let mut v = vec![-3, 1, -1, 2, 0, -2];
        ternary_quicksort(&mut v);
        assert_eq!(v, vec![-3, -2, -1, 0, 1, 2]);
    }

    #[test]
    fn quicksort_large_random() {
        let mut v: Vec<i32> = (0..100).rev().collect();
        ternary_quicksort(&mut v);
        assert!(is_sorted(&v));
        assert_eq!(v.len(), 100);
    }

    // --- ternary_radix_sort ---

    #[test]
    fn radix_sort_empty() {
        let mut v: Vec<i32> = vec![];
        ternary_radix_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn radix_sort_basic() {
        let mut v = vec![9, 3, 7, 1, 5, 8, 2, 4, 6];
        ternary_radix_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn radix_sort_negatives() {
        let mut v = vec![-3, 1, -1, 2, 0, -2];
        ternary_radix_sort(&mut v);
        assert_eq!(v, vec![-3, -2, -1, 0, 1, 2]);
    }

    #[test]
    fn radix_sort_large() {
        let mut v: Vec<i32> = (0..200).rev().collect();
        ternary_radix_sort(&mut v);
        assert!(is_sorted(&v));
    }

    // --- is_sorted ---

    #[test]
    fn is_sorted_checks() {
        assert!(is_sorted(&[1, 2, 3, 4]));
        assert!(is_sorted(&[1, 1, 2, 3]));
        assert!(!is_sorted(&[3, 2, 1]));
        assert!(is_sorted::<i32>(&[]));
        assert!(is_sorted(&[42]));
    }
}
