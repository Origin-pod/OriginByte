/**
 * @file 01-Bubble Sort.rs
 * @brief Bubble Sort Algorithm Implementation in Rust
 *
 * This file covers the bubble sort algorithm with various optimizations,
 * including standard bubble sort, optimized bubble sort, cocktail shaker sort,
 * and performance analysis with detailed explanations.
 */

use std::time::Instant;

// ================================================================================
// 1. BASIC BUBBLE SORT
// ================================================================================

/// Basic bubble sort implementation
/// Time Complexity: O(n²) worst and average case
/// Space Complexity: O(1) - in-place sorting
/// Stable: Yes (maintains relative order of equal elements)
pub fn bubble_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// ================================================================================
// 2. OPTIMIZED BUBBLE SORT
// ================================================================================

/// Optimized bubble sort with early termination
/// Stops early if no swaps were made in a pass
pub fn optimized_bubble_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // If no swaps occurred, the array is already sorted
        if !swapped {
            break;
        }
    }
}

// ================================================================================
// 3. COCKTAIL SHAKER SORT (BI-DIRECTIONAL BUBBLE SORT)
// ================================================================================

/// Cocktail shaker sort - bi-directional bubble sort
/// Sorts in both directions in each pass
/// Time Complexity: O(n²) worst and average case
/// Space Complexity: O(1)
pub fn cocktail_shaker_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mut start = 0;
    let mut end = arr.len() - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        // Forward pass
        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        // Reset swapped for backward pass
        swapped = false;
        end -= 1;

        // Backward pass
        for i in (start + 1..=end).rev() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }

        start += 1;
    }
}

// ================================================================================
// 4. RECURSIVE BUBBLE SORT
// ================================================================================

/// Recursive bubble sort implementation
/// Not practical for large arrays due to stack overflow risk
/// but included for educational purposes
pub fn recursive_bubble_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    // Perform one pass of bubble sort
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
        }
    }

    // Recursively sort the first n-1 elements
    recursive_bubble_sort(&mut arr[..arr.len() - 1]);
}

// ================================================================================
// 5. BUBBLE SORT WITH COMPARISON COUNT
// ================================================================================

/// Bubble sort that counts comparisons and swaps
/// Useful for analyzing algorithm performance
pub struct BubbleSortStats {
    pub comparisons: usize,
    pub swaps: usize,
    pub passes: usize,
}

pub fn bubble_sort_with_stats<T: Ord + Clone>(arr: &mut [T]) -> BubbleSortStats {
    let n = arr.len();
    let mut stats = BubbleSortStats {
        comparisons: 0,
        swaps: 0,
        passes: 0,
    };

    for i in 0..n {
        stats.passes += 1;
        let mut swapped = false;

        for j in 0..n - i - 1 {
            stats.comparisons += 1;
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                stats.swaps += 1;
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    stats
}

// ================================================================================
// 6. BUBBLE SORT WITH CUSTOM COMPARATOR
// ================================================================================

/// Bubble sort with custom comparison function
/// Allows for descending order or custom sorting criteria
pub fn bubble_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> bool,
{
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if compare(&arr[j], &arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
}

// ================================================================================
// 7. PREDICTIVE BUBBLE SORT
// ================================================================================

/// Bubble sort that tries to predict if array is nearly sorted
/// Uses adaptive behavior based on initial disorder measurement
pub fn predictive_bubble_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    // First, measure the disorder in the array
    let mut disorder = 0;
    for i in 0..n - 1 {
        if arr[i] > arr[i + 1] {
            disorder += 1;
        }
    }

    // If array is mostly sorted (less than 10% disorder), use optimized approach
    if disorder < n / 10 {
        optimized_bubble_sort(arr);
    } else {
        // Use standard bubble sort
        bubble_sort(arr);
    }
}

// ================================================================================
// 8. BUBBLE SORT WITH VISUALIZATION STEPS
// ================================================================================

/// Bubble sort that returns each step for visualization
pub struct SortStep<T> {
    pub array: Vec<T>,
    pub comparisons: usize,
    pub swaps: usize,
    pub current_indices: Vec<usize>,
}

pub fn bubble_sort_visualized<T: Ord + Clone + std::fmt::Debug>(arr: &[T]) -> Vec<SortStep<T>> {
    let mut current_array = arr.to_vec();
    let mut steps = Vec::new();
    let n = current_array.len();

    // Record initial state
    steps.push(SortStep {
        array: current_array.clone(),
        comparisons: 0,
        swaps: 0,
        current_indices: Vec::new(),
    });

    for i in 0..n {
        let mut swapped = false;
        let mut step_comparisons = 0;
        let mut step_swaps = 0;

        for j in 0..n - i - 1 {
            step_comparisons += 1;

            // Record comparison step
            steps.push(SortStep {
                array: current_array.clone(),
                comparisons: step_comparisons,
                swaps: step_swaps,
                current_indices: vec![j, j + 1],
            });

            if current_array[j] > current_array[j + 1] {
                current_array.swap(j, j + 1);
                step_swaps += 1;
                swapped = true;

                // Record swap step
                steps.push(SortStep {
                    array: current_array.clone(),
                    comparisons: step_comparisons,
                    swaps: step_swaps,
                    current_indices: vec![j, j + 1],
                });
            }
        }

        if !swapped {
            break;
        }
    }

    steps
}

// ================================================================================
// 9. PERFORMANCE ANALYSIS AND BENCHMARKING
// ================================================================================

pub struct SortPerformance {
    pub array_size: usize,
    pub time_taken: std::time::Duration,
    pub comparisons: usize,
    pub swaps: usize,
}

pub fn benchmark_bubble_sort<T: Ord + Clone + Copy>(arr: &[T]) -> SortPerformance {
    let mut test_arr = arr.to_vec();
    let n = test_arr.len();

    let start = Instant::now();
    let stats = bubble_sort_with_stats(&mut test_arr);
    let duration = start.elapsed();

    SortPerformance {
        array_size: n,
        time_taken: duration,
        comparisons: stats.comparisons,
        swaps: stats.swaps,
    }
}

pub fn compare_bubble_sort_variants<T: Ord + Clone + Copy + std::fmt::Debug>(
    original: &[T],
) -> (SortPerformance, SortPerformance, SortPerformance) {
    // Test basic bubble sort
    let mut arr1 = original.to_vec();
    let start1 = Instant::now();
    bubble_sort(&mut arr1);
    let perf1 = SortPerformance {
        array_size: original.len(),
        time_taken: start1.elapsed(),
        comparisons: 0,
        swaps: 0,
    };

    // Test optimized bubble sort
    let mut arr2 = original.to_vec();
    let start2 = Instant::now();
    let stats2 = bubble_sort_with_stats(&mut arr2);
    let perf2 = SortPerformance {
        array_size: original.len(),
        time_taken: start2.elapsed(),
        comparisons: stats2.comparisons,
        swaps: stats2.swaps,
    };

    // Test cocktail shaker sort
    let mut arr3 = original.to_vec();
    let start3 = Instant::now();
    cocktail_shaker_sort(&mut arr3);
    let perf3 = SortPerformance {
        array_size: original.len(),
        time_taken: start3.elapsed(),
        comparisons: 0,
        swaps: 0,
    };

    (perf1, perf2, perf3)
}

// ================================================================================
// 10. UTILITY FUNCTIONS
// ================================================================================

/// Checks if an array is sorted
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Generates test arrays of different patterns
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

pub fn demonstrate_bubble_sort() {
    println!("=== BUBBLE SORT DEMONSTRATION ===");

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr);

    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
    println!("Is sorted: {}", is_sorted(&arr));
}

pub fn demonstrate_optimized_bubble_sort() {
    println!("\n=== OPTIMIZED BUBBLE SORT DEMONSTRATION ===");

    let mut arr = [5, 1, 4, 2, 8];
    println!("Original array: {:?}", arr);

    let stats = bubble_sort_with_stats(&mut arr);
    println!("Sorted array: {:?}", arr);
    println!("Comparisons: {}", stats.comparisons);
    println!("Swaps: {}", stats.swaps);
    println!("Passes: {}", stats.passes);
}

pub fn demonstrate_cocktail_shaker_sort() {
    println!("\n=== COCKTAIL SHAKER SORT DEMONSTRATION ===");

    let mut arr = [5, 1, 4, 2, 8, 0, 2];
    println!("Original array: {:?}", arr);

    cocktail_shaker_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

pub fn demonstrate_custom_comparator() {
    println!("\n=== CUSTOM COMPARATOR DEMONSTRATION ===");

    let mut arr = [5, 2, 8, 1, 9, 3];
    println!("Original array: {:?}", arr);

    // Sort in descending order
    bubble_sort_by(&mut arr, |a, b| a < b);
    println!("Descending order: {:?}", arr);

    // Sort ascending again
    bubble_sort_by(&mut arr, |a, b| a > b);
    println!("Ascending order: {:?}", arr);
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
        let (basic_perf, optimized_perf, cocktail_perf) = compare_bubble_sort_variants(arr);

        println!("Basic bubble sort: {:?}", basic_perf.time_taken);
        println!("Optimized bubble sort: {:?} ({} comparisons, {} swaps)",
                 optimized_perf.time_taken, optimized_perf.comparisons, optimized_perf.swaps);
        println!("Cocktail shaker sort: {:?}", cocktail_perf.time_taken);
    }
}

pub fn demonstrate_visualization() {
    println!("\n=== VISUALIZATION DEMONSTRATION ===");

    let arr = [5, 1, 4, 2, 8];
    println!("Original array: {:?}", arr);

    let steps = bubble_sort_visualized(&arr);
    println!("Total steps: {}", steps.len());

    // Show first few steps
    for (i, step) in steps.iter().take(5).enumerate() {
        println!("Step {}: {:?} (comparing: {:?})", i + 1, step.array, step.current_indices);
    }
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║               BUBBLE SORT ALGORITHM                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_bubble_sort();
    demonstrate_optimized_bubble_sort();
    demonstrate_cocktail_shaker_sort();
    demonstrate_custom_comparator();
    demonstrate_performance_comparison();
    demonstrate_visualization();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("Bubble Sort Characteristics:");
    println!("• Time Complexity: O(n²) worst and average case");
    println!("• Space Complexity: O(1) - in-place sorting");
    println!("• Stable: Yes - maintains relative order of equal elements");
    println!("• Adaptive: Yes - optimized version stops early for sorted data");
    println!();
    println!("Advantages:");
    println!("✓ Simple to understand and implement");
    println!("✓ In-place sorting (no extra memory required)");
    println!("✓ Stable sorting algorithm");
    println!("✓ Adaptive to already sorted data");
    println!();
    println!("Disadvantages:");
    println!("✗ Inefficient for large datasets");
    println!("✗ O(n²) time complexity makes it impractical for most real-world use");
    println!("✗ Performs poorly compared to more advanced algorithms");
    println!();
    println!("Best Use Cases:");
    println!("• Educational purposes to understand sorting concepts");
    println!("• Small datasets where simplicity is valued over efficiency");
    println!("• Nearly sorted data (with optimization)");
    println!("• Situations where stability is crucial and data size is small");
}

/*
================================================================================
BUBBLE SORT ALGORITHM SUMMARY:
================================================================================

1. ALGORITHM DESCRIPTION:
   - Repeatedly steps through the list
   - Compares adjacent elements and swaps them if in wrong order
   - Passes through the list until no swaps are needed

2. TIME COMPLEXITY:
   - Best Case: O(n) - already sorted array (with optimization)
   - Average Case: O(n²)
   - Worst Case: O(n²) - reverse sorted array

3. SPACE COMPLEXITY:
   - O(1) - in-place sorting
   - Only requires a constant amount of additional memory

4. STABILITY:
   - Stable sorting algorithm
   - Maintains relative order of equal elements

5. VARIANTS IMPLEMENTED:
   - Basic bubble sort
   - Optimized bubble sort (early termination)
   - Cocktail shaker sort (bi-directional)
   - Recursive bubble sort
   - Bubble sort with statistics
   - Custom comparator support
   - Predictive bubble sort
   - Visualization steps

6. OPTIMIZATIONS:
   - Early termination when no swaps occur
   - Bi-directional sorting (cocktail shaker)
   - Adaptive behavior based on initial disorder
   - Reduced inner loop size after each pass

7. PERFORMANCE CHARACTERISTICS:
   - Excellent educational value
   - Poor performance on large datasets
   - Works well for small or nearly sorted arrays
   - Cache-friendly due to sequential access

8. PRACTICAL APPLICATIONS:
   - Educational demonstrations
   - Small dataset sorting
   - Situations requiring stability with simple implementation
   - Testing framework for sorting algorithms

COMPILATION:
rustc 01-Bubble_Sort.rs -o bubble_sort_demo

USAGE:
./bubble_sort_demo

Note: For random test arrays, add `rand = "0.8"` to Cargo.toml
================================================================================
*/