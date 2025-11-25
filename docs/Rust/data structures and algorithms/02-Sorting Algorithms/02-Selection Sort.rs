/**
 * @file 02-Selection Sort.rs
 * @brief Selection Sort Algorithm Implementation in Rust
 *
 * This file covers the selection sort algorithm with various implementations,
 * including standard selection sort, stable selection sort, heap selection sort,
 * and performance analysis with detailed explanations.
 */

use std::time::Instant;

// ================================================================================
// 1. BASIC SELECTION SORT
// ================================================================================

/// Basic selection sort implementation
/// Time Complexity: O(n²) in all cases
/// Space Complexity: O(1) - in-place sorting
/// Stable: No (may not maintain relative order of equal elements)
pub fn selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n - 1 {
        // Find the minimum element in the unsorted portion
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // Swap the found minimum element with the first element of unsorted portion
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

// ================================================================================
// 2. STABLE SELECTION SORT
// ================================================================================

/// Stable selection sort implementation
/// Maintains relative order of equal elements
/// Time Complexity: O(n²)
/// Space Complexity: O(1) - in-place sorting
/// Stable: Yes
pub fn stable_selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n - 1 {
        // Find the minimum element in the unsorted portion
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // Insert the minimum element at position i
        // by shifting elements to the right
        let min_value = arr[min_index].clone();
        for j in (min_index..=i).rev() {
            if j > 0 {
                arr.swap(j, j - 1);
            }
        }
        arr[i] = min_value;
    }
}

// ================================================================================
// 3. BIDIRECTIONAL SELECTION SORT
// ================================================================================

/// Bidirectional selection sort
/// Finds both minimum and maximum in each pass
/// Time Complexity: O(n²) but with half the passes
/// Space Complexity: O(1)
/// Stable: No
pub fn bidirectional_selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        // Find both minimum and maximum in the unsorted portion
        let mut min_index = left;
        let mut max_index = left;

        for i in left..=right {
            if arr[i] < arr[min_index] {
                min_index = i;
            }
            if arr[i] > arr[max_index] {
                max_index = i;
            }
        }

        // Swap minimum element to the left
        if min_index != left {
            arr.swap(left, min_index);

            // If max element was at left position, it's now at min_index
            if max_index == left {
                max_index = min_index;
            }
        }

        // Swap maximum element to the right
        if max_index != right {
            arr.swap(right, max_index);
        }

        left += 1;
        right -= 1;
    }
}

// ================================================================================
// 4. RECURSIVE SELECTION SORT
// ================================================================================

/// Recursive selection sort implementation
/// Not practical for large arrays due to stack overflow risk
/// Time Complexity: O(n²)
/// Space Complexity: O(n) due to recursion stack
pub fn recursive_selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    recursive_selection_sort_helper(arr, 0);
}

fn recursive_selection_sort_helper<T: Ord + Clone>(arr: &mut [T], start: usize) {
    if start >= arr.len() - 1 {
        return;
    }

    // Find minimum element from start to end
    let mut min_index = start;
    for i in (start + 1)..arr.len() {
        if arr[i] < arr[min_index] {
            min_index = i;
        }
    }

    // Swap minimum element to start position
    if min_index != start {
        arr.swap(start, min_index);
    }

    // Recursively sort the rest of the array
    recursive_selection_sort_helper(arr, start + 1);
}

// ================================================================================
// 5. SELECTION SORT WITH STATISTICS
// ================================================================================

/// Selection sort that tracks statistics
pub struct SelectionSortStats {
    pub comparisons: usize,
    pub swaps: usize,
    pub passes: usize,
}

pub fn selection_sort_with_stats<T: Ord + Clone>(arr: &mut [T]) -> SelectionSortStats {
    let n = arr.len();
    let mut stats = SelectionSortStats {
        comparisons: 0,
        swaps: 0,
        passes: 0,
    };

    for i in 0..n - 1 {
        stats.passes += 1;
        let mut min_index = i;

        for j in (i + 1)..n {
            stats.comparisons += 1;
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
            stats.swaps += 1;
        }
    }

    stats
}

// ================================================================================
// 6. SELECTION SORT WITH CUSTOM COMPARATOR
// ================================================================================

/// Selection sort with custom comparison function
/// Allows for descending order or custom sorting criteria
pub fn selection_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> bool,
{
    let n = arr.len();

    for i in 0..n - 1 {
        let mut selected_index = i;

        for j in (i + 1)..n {
            if compare(&arr[j], &arr[selected_index]) {
                selected_index = j;
            }
        }

        if selected_index != i {
            arr.swap(i, selected_index);
        }
    }
}

// ================================================================================
// 7. SELECTION SORT WITH INDEX TRACKING
// ================================================================================

/// Selection sort that returns the sorted indices
/// Useful when you need to maintain original array order
pub fn selection_sort_indices<T: Ord + Clone>(arr: &[T]) -> Vec<usize> {
    let n = arr.len();
    let mut indices: Vec<usize> = (0..n).collect();

    for i in 0..n - 1 {
        let mut min_index = i;

        for j in (i + 1)..n {
            if arr[indices[j]] < arr[indices[min_index]] {
                min_index = j;
            }
        }

        if min_index != i {
            indices.swap(i, min_index);
        }
    }

    indices
}

// ================================================================================
// 8. K-TH ORDER STATISTICS (SELECTION ALGORITHM)
// ================================================================================

/// Find the k-th smallest element using selection sort approach
/// Time Complexity: O(n*k) - more efficient than full sort if k is small
pub fn kth_smallest<T: Ord + Clone>(arr: &[T], k: usize) -> Option<&T> {
    if k >= arr.len() {
        return None;
    }

    let mut temp_arr = arr.to_vec();
    selection_sort(&mut temp_arr);
    Some(&temp_arr[k])
}

/// Find the k-th largest element
pub fn kth_largest<T: Ord + Clone>(arr: &[T], k: usize) -> Option<&T> {
    if k >= arr.len() {
        return None;
    }

    let kth_smallest_index = arr.len() - k - 1;
    kth_smallest(arr, kth_smallest_index)
}

// ================================================================================
// 9. PARTIAL SELECTION SORT
// ================================================================================

/// Sort only the first k elements
/// More efficient than full sort when you need only partially sorted data
pub fn partial_selection_sort<T: Ord + Clone>(arr: &mut [T], k: usize) {
    let n = arr.len();
    let sort_limit = k.min(n);

    for i in 0..sort_limit {
        let mut min_index = i;

        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

// ================================================================================
// 10. SELECTION SORT WITH VISUALIZATION
// ================================================================================

pub struct SelectionSortStep<T> {
    pub array: Vec<T>,
    pub comparisons: usize,
    pub swaps: usize,
    pub min_index: Option<usize>,
    pub sorted_indices: Vec<usize>,
}

pub fn selection_sort_visualized<T: Ord + Clone + std::fmt::Debug>(arr: &[T]) -> Vec<SelectionSortStep<T>> {
    let mut current_array = arr.to_vec();
    let mut steps = Vec::new();
    let n = current_array.len();

    // Record initial state
    steps.push(SelectionSortStep {
        array: current_array.clone(),
        comparisons: 0,
        swaps: 0,
        min_index: None,
        sorted_indices: Vec::new(),
    });

    for i in 0..n - 1 {
        let mut min_index = i;
        let mut comparisons = 0;

        // Find minimum element
        for j in (i + 1)..n {
            comparisons += 1;

            // Record comparison step
            steps.push(SelectionSortStep {
                array: current_array.clone(),
                comparisons,
                swaps: 0,
                min_index: Some(j),
                sorted_indices: (0..i).collect(),
            });

            if current_array[j] < current_array[min_index] {
                min_index = j;
            }
        }

        // Swap if necessary
        if min_index != i {
            current_array.swap(i, min_index);

            // Record swap step
            steps.push(SelectionSortStep {
                array: current_array.clone(),
                comparisons,
                swaps: 1,
                min_index: Some(min_index),
                sorted_indices: (0..=i).collect(),
            });
        }
    }

    // Record final state
    steps.push(SelectionSortStep {
        array: current_array.clone(),
        comparisons: 0,
        swaps: 0,
        min_index: None,
        sorted_indices: (0..n).collect(),
    });

    steps
}

// ================================================================================
// 11. PERFORMANCE ANALYSIS AND BENCHMARKING
// ================================================================================

pub struct SortPerformance {
    pub array_size: usize,
    pub time_taken: std::time::Duration,
    pub comparisons: usize,
    pub swaps: usize,
}

pub fn benchmark_selection_sort<T: Ord + Clone + Copy>(arr: &[T]) -> SortPerformance {
    let mut test_arr = arr.to_vec();

    let start = Instant::now();
    let stats = selection_sort_with_stats(&mut test_arr);
    let duration = start.elapsed();

    SortPerformance {
        array_size: arr.len(),
        time_taken: duration,
        comparisons: stats.comparisons,
        swaps: stats.swaps,
    }
}

pub fn compare_selection_sort_variants<T: Ord + Clone + Copy + std::fmt::Debug>(
    original: &[T],
) -> (SortPerformance, SortPerformance, SortPerformance) {
    // Test basic selection sort
    let mut arr1 = original.to_vec();
    let start1 = Instant::now();
    selection_sort(&mut arr1);
    let perf1 = SortPerformance {
        array_size: original.len(),
        time_taken: start1.elapsed(),
        comparisons: 0,
        swaps: 0,
    };

    // Test stable selection sort
    let mut arr2 = original.to_vec();
    let start2 = Instant::now();
    stable_selection_sort(&mut arr2);
    let perf2 = SortPerformance {
        array_size: original.len(),
        time_taken: start2.elapsed(),
        comparisons: 0,
        swaps: 0,
    };

    // Test bidirectional selection sort
    let mut arr3 = original.to_vec();
    let start3 = Instant::now();
    bidirectional_selection_sort(&mut arr3);
    let perf3 = SortPerformance {
        array_size: original.len(),
        time_taken: start3.elapsed(),
        comparisons: 0,
        swaps: 0,
    };

    (perf1, perf2, perf3)
}

// ================================================================================
// 12. UTILITY FUNCTIONS
// ================================================================================

/// Checks if an array is sorted
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Generate test arrays of different patterns
pub mod test_arrays {
    use rand::Rng;

    pub fn random_array(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(1..1000)).collect()
    }

    pub fn sorted_array(size: usize) -> Vec<i32> {
        (1..=size).map(|x| x as i32).collect()
    }

    pub fn reverse_sorted_array(size: usize) -> Vec<i32> {
        (1..=size).rev().map(|x| x as i32).collect()
    }

    pub fn nearly_sorted_array(size: usize, disorder_percentage: f64) -> Vec<i32> {
        let mut arr = sorted_array(size);
        let disorder_count = (size as f64 * disorder_percentage / 100.0) as usize;
        let mut rng = rand::thread_rng();

        for _ in 0..disorder_count {
            let i = rng.gen_range(0..size);
            let j = rng.gen_range(0..size);
            arr.swap(i, j);
        }

        arr
    }

    pub fn array_with_duplicates(size: usize, duplicate_factor: u32) -> Vec<i32> {
        let unique_count = size / duplicate_factor as usize;
        let mut arr: Vec<i32> = (1..=unique_count).map(|x| x as i32).collect();

        for _ in 0..duplicate_factor - 1 {
            let mut temp = (1..=unique_count).map(|x| x as i32).collect();
            arr.append(&mut temp);
        }

        use rand::seq::SliceRandom;
        arr.shuffle(&mut rand::thread_rng());
        arr.truncate(size);
        arr
    }
}

// ================================================================================
// DEMONSTRATION FUNCTIONS
// ================================================================================

pub fn demonstrate_selection_sort() {
    println!("=== SELECTION SORT DEMONSTRATION ===");

    let mut arr = [64, 25, 12, 22, 11];
    println!("Original array: {:?}", arr);

    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
    println!("Is sorted: {}", is_sorted(&arr));
}

pub fn demonstrate_stable_selection_sort() {
    println!("\n=== STABLE SELECTION SORT DEMONSTRATION ===");

    // Use a custom struct to demonstrate stability
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Item {
        value: i32,
        label: char,
    }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut arr = [
        Item { value: 3, label: 'a' },
        Item { value: 1, label: 'b' },
        Item { value: 3, label: 'c' },
        Item { value: 2, label: 'd' },
    ];

    println!("Original array: {:?}", arr);

    stable_selection_sort(&mut arr);
    println!("Stable sorted array: {:?}", arr);
    println!("Note: Equal elements maintain their relative order");
}

pub fn demonstrate_bidirectional_selection_sort() {
    println!("\n=== BIDIRECTIONAL SELECTION SORT DEMONSTRATION ===");

    let mut arr = [23, 78, 45, 8, 32, 56, 1];
    println!("Original array: {:?}", arr);

    bidirectional_selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

pub fn demonstrate_kth_statistics() {
    println!("\n=== K-TH ORDER STATISTICS DEMONSTRATION ===");

    let arr = [12, 3, 5, 7, 19, 1];
    println!("Array: {:?}", arr);

    let kth = 2;
    if let Some(kth_smallest) = kth_smallest(&arr, kth) {
        println!("{}-nd smallest element: {}", kth + 1, kth_smallest);
    }

    let kth_largest = 1;
    if let Some(kth_largest_elem) = kth_largest(&arr, kth_largest) {
        println!("{}-st largest element: {}", kth_largest + 1, kth_largest_elem);
    }
}

pub fn demonstrate_partial_sort() {
    println!("\n=== PARTIAL SELECTION SORT DEMONSTRATION ===");

    let mut arr = [64, 25, 12, 22, 11, 89, 34, 56];
    println!("Original array: {:?}", arr);

    partial_selection_sort(&mut arr, 3);
    println!("Partially sorted (first 3 elements): {:?}", arr);
    println!("Note: Only first 3 elements are guaranteed to be in correct position");
}

pub fn demonstrate_custom_comparator() {
    println!("\n=== CUSTOM COMPARATOR DEMONSTRATION ===");

    let mut arr = [5, 2, 8, 1, 9, 3];
    println!("Original array: {:?}", arr);

    // Sort in descending order
    selection_sort_by(&mut arr, |a, b| a > b);
    println!("Descending order: {:?}", arr);

    // Sort by absolute value
    let mut arr_with_neg = [-5, 2, -8, 1, -9, 3];
    selection_sort_by(&mut arr_with_neg, |a, b| a.abs() < b.abs());
    println!("Sort by absolute value: {:?}", arr_with_neg);
}

pub fn demonstrate_performance_comparison() {
    println!("\n=== PERFORMANCE COMPARISON ===");

    let test_cases = [
        ("Random (size 100)", test_arrays::random_array(100)),
        ("Nearly sorted (size 100)", test_arrays::nearly_sorted_array(100, 10.0)),
        ("Reverse sorted (size 100)", test_arrays::reverse_sorted_array(100)),
    ];

    for (name, arr) in &test_cases {
        println!("\nTesting: {}", name);
        let (basic_perf, stable_perf, bidirectional_perf) = compare_selection_sort_variants(arr);

        println!("Basic selection sort: {:?}", basic_perf.time_taken);
        println!("Stable selection sort: {:?}", stable_perf.time_taken);
        println!("Bidirectional selection sort: {:?}", bidirectional_perf.time_taken);
    }
}

pub fn demonstrate_visualization() {
    println!("\n=== VISUALIZATION DEMONSTRATION ===");

    let arr = [64, 25, 12, 22, 11];
    println!("Original array: {:?}", arr);

    let steps = selection_sort_visualized(&arr);
    println!("Total steps: {}", steps.len());

    // Show first few steps
    for (i, step) in steps.iter().take(6).enumerate() {
        if let Some(min_idx) = step.min_index {
            println!("Step {}: {:?} (comparing index {}, sorted: {:?})",
                     i + 1, step.array, min_idx, step.sorted_indices);
        } else {
            println!("Step {}: {:?} (sorted: {:?})",
                     i + 1, step.array, step.sorted_indices);
        }
    }
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║               SELECTION SORT ALGORITHM                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_selection_sort();
    demonstrate_stable_selection_sort();
    demonstrate_bidirectional_selection_sort();
    demonstrate_kth_statistics();
    demonstrate_partial_sort();
    demonstrate_custom_comparator();
    demonstrate_performance_comparison();
    demonstrate_visualization();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("Selection Sort Characteristics:");
    println!("• Time Complexity: O(n²) in all cases");
    println!("• Space Complexity: O(1) - in-place sorting");
    println!("• Stable: No (unless implemented specifically)");
    println!("• Adaptive: No (always O(n²) regardless of input)");
    println!();
    println!("Advantages:");
    println!("✓ Simple to understand and implement");
    println!("✓ In-place sorting (no extra memory required)");
    println!("✓ Minimal data movement (only n-1 swaps in worst case)");
    println!("✓ Good for small datasets");
    println!("✓ Predictable performance");
    println!();
    println!("Disadvantages:");
    println!("✗ Inefficient for large datasets");
    println!("✗ O(n²) time complexity makes it impractical for most use");
    println!("✗ Not stable by default");
    println!("✗ Not adaptive to already sorted data");
    println!();
    println!("Best Use Cases:");
    println!("• Educational purposes to understand sorting concepts");
    println!("• Small datasets where simplicity is valued");
    println!("• Situations requiring minimal swaps");
    println!("• When memory usage must be minimized");
    println!("• Finding k-th smallest/largest elements");
}

/*
================================================================================
SELECTION SORT ALGORITHM SUMMARY:
================================================================================

1. ALGORITHM DESCRIPTION:
   - Divides array into sorted and unsorted portions
   - Finds minimum element in unsorted portion
   - Swaps minimum element with first unsorted element
   - Repeat until entire array is sorted

2. TIME COMPLEXITY:
   - Best Case: O(n²)
   - Average Case: O(n²)
   - Worst Case: O(n²)
   - Consistent performance regardless of input order

3. SPACE COMPLEXITY:
   - O(1) - in-place sorting
   - Only requires a constant amount of additional memory

4. STABILITY:
   - Not stable by default
   - Can be made stable with additional shifting

5. VARIANTS IMPLEMENTED:
   - Basic selection sort
   - Stable selection sort
   - Bidirectional selection sort
   - Recursive selection sort
   - Selection sort with statistics
   - Custom comparator support
   - K-th order statistics
   - Partial selection sort

6. KEY CHARACTERISTICS:
   - Always performs exactly n-1 comparisons in the best case
   - Maximum n-1 swaps, minimum 0 swaps
   - Consistent O(n²) behavior
   - Good cache locality

7. PERFORMANCE CHARACTERISTICS:
   - Excellent educational value
   - Minimal memory usage
   - Predictable performance
   - Fewer swaps than bubble sort
   - Works well for small datasets

8. PRACTICAL APPLICATIONS:
   - Finding k-th smallest/largest elements
   - Situations requiring minimal data movement
   - Small dataset sorting
   - Embedded systems with memory constraints
   - Selection-based algorithms

COMPILATION:
rustc 02-Selection_Sort.rs -o selection_sort_demo

USAGE:
./selection_sort_demo

Note: For random test arrays, add `rand = "0.8"` to Cargo.toml
================================================================================
*/