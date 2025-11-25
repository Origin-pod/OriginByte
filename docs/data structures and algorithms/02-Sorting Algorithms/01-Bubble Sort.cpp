#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Bubble Sort Implementation
 *
 * Bubble Sort is a simple sorting algorithm that repeatedly steps through the list,
 * compares adjacent elements and swaps them if they are in the wrong order.
 * The pass through the list is repeated until the list is sorted.
 *
 * Time Complexity:
 * - Best Case: O(n) - when array is already sorted (with optimization)
 * - Average Case: O(n²)
 * - Worst Case: O(n²)
 *
 * Space Complexity: O(1) - in-place sorting
 *
 * Stable: Yes (maintains relative order of equal elements)
 *
 * Advantages:
 * - Simple to understand and implement
 * - Stable sort
 * - In-place sorting (minimal extra space)
 * - Can detect if array is already sorted
 *
 * Disadvantages:
 * - Inefficient for large datasets
 * - Much slower than O(n log n) algorithms like quicksort, mergesort
 *
 * Applications:
 * - Educational purposes (demonstrates sorting concepts)
 * - Small datasets where simplicity is preferred
 * - When you need to detect if array is already sorted
 */

/**
 * @brief Basic Bubble Sort implementation
 * @param arr Vector to be sorted
 * @return Number of comparisons made (for analysis)
 */
int bubble_sort_basic(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;

    for (int i = 0; i < n - 1; ++i) {
        for (int j = 0; j < n - i - 1; ++j) {
            comparisons++;
            if (arr[j] > arr[j + 1]) {
                std::swap(arr[j], arr[j + 1]);
            }
        }
    }

    return comparisons;
}

/**
 * @brief Optimized Bubble Sort with early termination
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int bubble_sort_optimized(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;
    bool swapped;

    for (int i = 0; i < n - 1; ++i) {
        swapped = false;
        for (int j = 0; j < n - i - 1; ++j) {
            comparisons++;
            if (arr[j] > arr[j + 1]) {
                std::swap(arr[j], arr[j + 1]);
                swapped = true;
            }
        }

        // If no swapping occurred, array is already sorted
        if (!swapped) {
            break;
        }
    }

    return comparisons;
}

/**
 * @brief Bubble Sort with boundary optimization
 * Tracks the last swap position to reduce the range of subsequent passes
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int bubble_sort_boundary_optimized(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;
    int new_n;

    while (n > 1) {
        new_n = 0;
        for (int i = 1; i < n; ++i) {
            comparisons++;
            if (arr[i - 1] > arr[i]) {
                std::swap(arr[i - 1], arr[i]);
                new_n = i;  // Last swap position
            }
        }
        n = new_n;
    }

    return comparisons;
}

/**
 * @brief Recursive Bubble Sort
 * Demonstrates the recursive approach to bubble sort
 * @param arr Vector to be sorted
 * @param n Size of unsorted portion
 */
void bubble_sort_recursive(std::vector<int>& arr, int n) {
    // Base case: if array has 0 or 1 element, it's already sorted
    if (n == 0 || n == 1) {
        return;
    }

    // One pass of bubble sort to move largest element to end
    for (int i = 0; i < n - 1; ++i) {
        if (arr[i] > arr[i + 1]) {
            std::swap(arr[i], arr[i + 1]);
        }
    }

    // Recursively sort remaining array
    bubble_sort_recursive(arr, n - 1);
}

/**
 * @brief Generic Bubble Sort for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Vector to be sorted
 * @param ascending Sort order (true for ascending, false for descending)
 */
template <typename T>
void bubble_sort_generic(std::vector<T>& arr, bool ascending = true) {
    int n = arr.size();
    bool swapped;

    for (int i = 0; i < n - 1; ++i) {
        swapped = false;
        for (int j = 0; j < n - i - 1; ++j) {
            if ((ascending && arr[j] > arr[j + 1]) ||
                (!ascending && arr[j] < arr[j + 1])) {
                std::swap(arr[j], arr[j + 1]);
                swapped = true;
            }
        }
        if (!swapped) break;
    }
}

/**
 * @brief Cocktail Shaker Sort (Bidirectional Bubble Sort)
 * A variation of bubble sort that sorts in both directions on each pass
 * @param arr Vector to be sorted
 */
void cocktail_shaker_sort(std::vector<int>& arr) {
    int n = arr.size();
    bool swapped = true;
    int start = 0;
    int end = n - 1;

    while (swapped) {
        // Forward pass
        swapped = false;
        for (int i = start; i < end; ++i) {
            if (arr[i] > arr[i + 1]) {
                std::swap(arr[i], arr[i + 1]);
                swapped = true;
            }
        }

        // If no swapping occurred, array is sorted
        if (!swapped) break;

        // Reset swapped flag for backward pass
        swapped = false;
        // Decrease end point
        end--;

        // Backward pass
        for (int i = end - 1; i >= start; --i) {
            if (arr[i] > arr[i + 1]) {
                std::swap(arr[i], arr[i + 1]);
                swapped = true;
            }
        }

        // Increase start point
        start++;
    }
}

/**
 * @brief Utility function to check if array is sorted
 * @param arr Vector to check
 * @param ascending Check order (true for ascending, false for descending)
 * @return true if array is sorted in specified order
 */
bool is_sorted(const std::vector<int>& arr, bool ascending = true) {
    for (size_t i = 0; i < arr.size() - 1; ++i) {
        if ((ascending && arr[i] > arr[i + 1]) ||
            (!ascending && arr[i] < arr[i + 1])) {
            return false;
        }
    }
    return true;
}

/**
 * @brief Print array for debugging
 * @param arr Vector to print
 * @param label Optional label for the output
 */
void print_array(const std::vector<int>& arr, const std::string& label = "") {
    if (!label.empty()) {
        std::cout << label << ": ";
    }
    std::cout << "[";
    for (size_t i = 0; i < arr.size(); ++i) {
        std::cout << arr[i];
        if (i < arr.size() - 1) {
            std::cout << ", ";
        }
    }
    std::cout << "]" << std::endl;
}

/**
 * @brief Test function to demonstrate bubble sort variants
 */
void test_bubble_sort() {
    std::vector<int> test_arrays[] = {
        {64, 34, 25, 12, 22, 11, 90},
        {5, 1, 4, 2, 8},
        {1, 2, 3, 4, 5},  // Already sorted
        {5, 4, 3, 2, 1},  // Reverse sorted
        {1},              // Single element
        {}                // Empty array
    };

    std::string test_names[] = {
        "Random array",
        "Small array",
        "Already sorted",
        "Reverse sorted",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 6; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;

        // Test basic bubble sort
        auto arr1 = test_arrays[i];
        print_array(arr1, "Original");
        int comparisons1 = bubble_sort_basic(arr1);
        print_array(arr1, "Basic bubble sort");
        std::cout << "Comparisons: " << comparisons1 << std::endl;

        // Test optimized bubble sort
        auto arr2 = test_arrays[i];
        int comparisons2 = bubble_sort_optimized(arr2);
        std::cout << "Optimized comparisons: " << comparisons2 << std::endl;

        // Test cocktail shaker sort
        auto arr3 = test_arrays[i];
        cocktail_shaker_sort(arr3);
        print_array(arr3, "Cocktail shaker sort");
    }
}

/**
 * @brief Performance comparison function
 * Compares different bubble sort variants on the same data
 */
void compare_bubble_sort_variants() {
    // Generate test data
    std::vector<int> test_data(1000);
    for (int i = 0; i < 1000; ++i) {
        test_data[i] = rand() % 1000;
    }

    std::cout << "\n=== Performance Comparison (1000 elements) ===" << std::endl;

    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    int comps1 = bubble_sort_basic(arr1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    auto arr2 = test_data;
    start = std::chrono::high_resolution_clock::now();
    int comps2 = bubble_sort_optimized(arr2);
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    auto arr3 = test_data;
    start = std::chrono::high_resolution_clock::now();
    bubble_sort_boundary_optimized(arr3);
    end = std::chrono::high_resolution_clock::now();
    auto time3 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Basic bubble sort: " << time1.count() << " μs, " << comps1 << " comparisons" << std::endl;
    std::cout << "Optimized bubble sort: " << time2.count() << " μs, " << comps2 << " comparisons" << std::endl;
    std::cout << "Boundary optimized: " << time3.count() << " μs" << std::endl;
}