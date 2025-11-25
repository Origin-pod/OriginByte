#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Selection Sort Implementation
 *
 * Selection Sort divides the input list into two parts: a sorted sublist which is built up
 * from left to right, and a sublist of the remaining unsorted items. It repeatedly finds
 * the minimum element from the unsorted sublist and moves it to the sorted sublist.
 *
 * Time Complexity:
 * - Best Case: O(n²)
 * - Average Case: O(n²)
 * - Worst Case: O(n²)
 *
 * Space Complexity: O(1) - in-place sorting
 *
 * Stable: No (can be made stable with additional space)
 *
 * Advantages:
 * - Simple to understand and implement
 * - Minimal memory usage (in-place)
 * - Performs well on small lists
 * - Reduces number of swaps compared to bubble sort
 *
 * Disadvantages:
 * - Inefficient for large datasets
 * - Performance doesn't improve on partially sorted data
 * - Not stable by default
 *
 * Applications:
 * - Small datasets where simplicity is preferred
 * - When memory writes are expensive (minimizes swaps)
 * - Educational purposes
 * - Embedded systems with limited memory
 */

/**
 * @brief Basic Selection Sort implementation
 * Finds minimum element in unsorted portion and swaps with first unsorted element
 * @param arr Vector to be sorted
 * @return Number of swaps performed
 */
int selection_sort_basic(std::vector<int>& arr) {
    int n = arr.size();
    int swaps = 0;

    for (int i = 0; i < n - 1; ++i) {
        // Find index of minimum element in unsorted portion
        int min_idx = i;
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
        }

        // Swap minimum element with first unsorted element
        if (min_idx != i) {
            std::swap(arr[i], arr[min_idx]);
            swaps++;
        }
    }

    return swaps;
}

/**
 * @brief Selection Sort with maximum element approach
 * Finds maximum element and places it at the end
 * @param arr Vector to be sorted
 * @return Number of swaps performed
 */
int selection_sort_max_approach(std::vector<int>& arr) {
    int n = arr.size();
    int swaps = 0;

    for (int i = n - 1; i > 0; --i) {
        // Find index of maximum element in unsorted portion
        int max_idx = i;
        for (int j = 0; j < i; ++j) {
            if (arr[j] > arr[max_idx]) {
                max_idx = j;
            }
        }

        // Swap maximum element with last unsorted element
        if (max_idx != i) {
            std::swap(arr[i], arr[max_idx]);
            swaps++;
        }
    }

    return swaps;
}

/**
 * @brief Bidirectional Selection Sort
 * Finds both minimum and maximum in each pass
 * @param arr Vector to be sorted
 * @return Number of swaps performed
 */
int selection_sort_bidirectional(std::vector<int>& arr) {
    int n = arr.size();
    int swaps = 0;
    int left = 0;
    int right = n - 1;

    while (left < right) {
        // Find both minimum and maximum in current range
        int min_idx = left;
        int max_idx = left;

        for (int i = left; i <= right; ++i) {
            if (arr[i] < arr[min_idx]) {
                min_idx = i;
            }
            if (arr[i] > arr[max_idx]) {
                max_idx = i;
            }
        }

        // Swap minimum with left
        if (min_idx != left) {
            std::swap(arr[left], arr[min_idx]);
            swaps++;

            // If max_idx was at left position, it's now at min_idx
            if (max_idx == left) {
                max_idx = min_idx;
            }
        }

        // Swap maximum with right
        if (max_idx != right) {
            std::swap(arr[right], arr[max_idx]);
            swaps++;
        }

        left++;
        right--;
    }

    return swaps;
}

/**
 * @brief Stable Selection Sort
 * Maintains relative order of equal elements
 * @param arr Vector to be sorted
 * @return Number of shifts performed
 */
int selection_sort_stable(std::vector<int>& arr) {
    int n = arr.size();
    int shifts = 0;

    for (int i = 0; i < n - 1; ++i) {
        // Find minimum element in unsorted portion
        int min_idx = i;
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
        }

        // Instead of swapping, insert minimum element at correct position
        int min_value = arr[min_idx];
        while (min_idx > i) {
            arr[min_idx] = arr[min_idx - 1];
            min_idx--;
            shifts++;
        }
        arr[i] = min_value;
    }

    return shifts;
}

/**
 * @brief Generic Selection Sort for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Vector to be sorted
 * @param ascending Sort order (true for ascending, false for descending)
 */
template <typename T>
void selection_sort_generic(std::vector<T>& arr, bool ascending = true) {
    int n = arr.size();

    for (int i = 0; i < n - 1; ++i) {
        int extreme_idx = i;  // min_idx for ascending, max_idx for descending

        for (int j = i + 1; j < n; ++j) {
            if ((ascending && arr[j] < arr[extreme_idx]) ||
                (!ascending && arr[j] > arr[extreme_idx])) {
                extreme_idx = j;
            }
        }

        if (extreme_idx != i) {
            std::swap(arr[i], arr[extreme_idx]);
        }
    }
}

/**
 * @brief Selection Sort with early termination optimization
 * Stops early if array is already sorted
 * @param arr Vector to be sorted
 * @return Number of swaps performed
 */
int selection_sort_optimized(std::vector<int>& arr) {
    int n = arr.size();
    int swaps = 0;

    for (int i = 0; i < n - 1; ++i) {
        int min_idx = i;
        bool sorted = true;

        // Find minimum and check if array is already sorted
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
            if (arr[j - 1] > arr[j]) {
                sorted = false;
            }
        }

        // If array is sorted, we're done
        if (sorted && i == 0) {
            break;
        }

        // Swap if necessary
        if (min_idx != i) {
            std::swap(arr[i], arr[min_idx]);
            swaps++;
        }
    }

    return swaps;
}

/**
 * @brief Recursive Selection Sort
 * Demonstrates recursive approach to selection sort
 * @param arr Vector to be sorted
 * @param start Starting index of unsorted portion
 * @param n Size of the array
 */
void selection_sort_recursive(std::vector<int>& arr, int start, int n) {
    // Base case: if the array is sorted or has one element
    if (start >= n - 1) {
        return;
    }

    // Find minimum element in unsorted portion
    int min_idx = start;
    for (int i = start + 1; i < n; ++i) {
        if (arr[i] < arr[min_idx]) {
            min_idx = i;
        }
    }

    // Swap minimum element with first unsorted element
    if (min_idx != start) {
        std::swap(arr[start], arr[min_idx]);
    }

    // Recursively sort remaining array
    selection_sort_recursive(arr, start + 1, n);
}

/**
 * @brief Selection Sort for partial sorting
 * Sorts only first k elements (finds k smallest elements)
 * @param arr Vector to be partially sorted
 * @param k Number of elements to sort
 */
void selection_sort_partial(std::vector<int>& arr, int k) {
    int n = arr.size();
    if (k <= 0 || k > n) return;

    for (int i = 0; i < k; ++i) {
        int min_idx = i;
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
        }
        if (min_idx != i) {
            std::swap(arr[i], arr[min_idx]);
        }
    }
}

/**
 * @brief Find kth smallest element using selection sort approach
 * @param arr Input vector
 * @param k Position (1-based) of element to find
 * @return kth smallest element
 */
int find_kth_smallest(std::vector<int> arr, int k) {
    int n = arr.size();
    if (k <= 0 || k > n) {
        throw std::out_of_range("k is out of bounds");
    }

    for (int i = 0; i < k; ++i) {
        int min_idx = i;
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j;
            }
        }
        std::swap(arr[i], arr[min_idx]);
    }

    return arr[k - 1];
}

/**
 * @brief Test function to demonstrate selection sort variants
 */
void test_selection_sort() {
    std::vector<int> test_arrays[] = {
        {64, 34, 25, 12, 22, 11, 90},
        {5, 1, 4, 2, 8},
        {1, 2, 3, 4, 5},  // Already sorted
        {5, 4, 3, 2, 1},  // Reverse sorted
        {3, 1, 4, 1, 5, 9, 2, 6, 5},  // With duplicates
        {1},              // Single element
        {}                // Empty array
    };

    std::string test_names[] = {
        "Random array",
        "Small array",
        "Already sorted",
        "Reverse sorted",
        "With duplicates",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 7; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;

        // Test basic selection sort
        auto arr1 = test_arrays[i];
        print_array(arr1, "Original");
        int swaps1 = selection_sort_basic(arr1);
        print_array(arr1, "Basic selection sort");
        std::cout << "Swaps: " << swaps1 << std::endl;

        // Test stable selection sort
        auto arr2 = test_arrays[i];
        int shifts = selection_sort_stable(arr2);
        std::cout << "Stable version shifts: " << shifts << std::endl;

        // Test bidirectional selection sort
        auto arr3 = test_arrays[i];
        int swaps3 = selection_sort_bidirectional(arr3);
        std::cout << "Bidirectional swaps: " << swaps3 << std::endl;

        // Test partial sort (first 3 elements)
        if (test_arrays[i].size() > 3) {
            auto arr4 = test_arrays[i];
            selection_sort_partial(arr4, 3);
            print_array(arr4, "Partial sort (first 3)");
        }
    }
}

/**
 * @brief Performance comparison with other sorting algorithms
 */
void compare_sorting_performance() {
    // Generate test data
    std::vector<int> test_data(1000);
    for (int i = 0; i < 1000; ++i) {
        test_data[i] = rand() % 1000;
    }

    std::cout << "\n=== Performance Comparison (1000 elements) ===" << std::endl;

    // Selection Sort
    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    int swaps = selection_sort_basic(arr1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Bubble Sort (for comparison)
    auto arr2 = test_data;
    start = std::chrono::high_resolution_clock::now();
    bubble_sort_optimized(arr2);
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Selection Sort: " << time1.count() << " μs, " << swaps << " swaps" << std::endl;
    std::cout << "Bubble Sort: " << time2.count() << " μs" << std::endl;
}

/**
 * @brief Demonstrate finding kth smallest element
 */
void test_kth_smallest() {
    std::vector<int> arr = {7, 10, 4, 3, 20, 15};
    print_array(arr, "Original array");

    for (int k = 1; k <= arr.size(); ++k) {
        int kth_smallest = find_kth_smallest(arr, k);
        std::cout << k << "th smallest element: " << kth_smallest << std::endl;
    }
}