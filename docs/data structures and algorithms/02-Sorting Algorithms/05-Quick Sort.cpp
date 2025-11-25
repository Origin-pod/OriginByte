#pragma once
#include <vector>
#include <algorithm>
#include <stack>

/**
 * @brief Quick Sort Implementation
 *
 * Quick Sort is a divide-and-conquer algorithm that picks an element as pivot and
 * partitions the array around the pivot, placing smaller elements to the left and
 * larger elements to the right.
 *
 * Time Complexity:
 * - Best Case: O(n log n) - when pivot divides array into equal halves
 * - Average Case: O(n log n)
 * - Worst Case: O(n²) - when pivot is always smallest or largest element
 *
 * Space Complexity: O(log n) - average case for recursion stack
 * Space Complexity: O(n) - worst case for recursion stack
 *
 * Stable: No (doesn't maintain relative order of equal elements)
 *
 * Advantages:
 * - Fast in practice on average
 * - In-place sorting (minimal extra space)
 * - Good cache locality
 * - Can be parallelized
 *
 * Disadvantages:
 * - Worst case O(n²) performance
 * - Not stable
 * - Performance depends on pivot selection
 *
 * Applications:
 * - General-purpose sorting
 * - When average-case performance is more important than worst-case
 * - In-place sorting requirements
 * - Internal sorting in many standard libraries
 */

/**
 * @brief Partition function using Lomuto partition scheme
 * @param arr Vector to be partitioned
 * @param low Starting index
 * @param high Ending index
 * @return Index of pivot after partition
 */
int partition_lomuto(std::vector<int>& arr, int low, int high) {
    int pivot = arr[high];  // Choose last element as pivot
    int i = low - 1;        // Index of smaller element

    for (int j = low; j < high; ++j) {
        if (arr[j] <= pivot) {
            i++;
            std::swap(arr[i], arr[j]);
        }
    }

    std::swap(arr[i + 1], arr[high]);
    return i + 1;
}

/**
 * @brief Partition function using Hoare partition scheme
 * More efficient than Lomuto in some cases
 * @param arr Vector to be partitioned
 * @param low Starting index
 * @param high Ending index
 * @return Partition index
 */
int partition_hoare(std::vector<int>& arr, int low, int high) {
    int pivot = arr[low];  // Choose first element as pivot
    int i = low - 1;
    int j = high + 1;

    while (true) {
        // Find leftmost element greater than or equal to pivot
        do {
            i++;
        } while (arr[i] < pivot);

        // Find rightmost element less than or equal to pivot
        do {
            j--;
        } while (arr[j] > pivot);

        // If pointers cross, partition is done
        if (i >= j) {
            return j;
        }

        std::swap(arr[i], arr[j]);
    }
}

/**
 * @brief Quick Sort using Lomuto partition scheme
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_lomuto(std::vector<int>& arr, int low, int high) {
    if (low < high) {
        int pivot_idx = partition_lomuto(arr, low, high);
        quick_sort_lomuto(arr, low, pivot_idx - 1);
        quick_sort_lomuto(arr, pivot_idx + 1, high);
    }
}

/**
 * @brief Quick Sort using Hoare partition scheme
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_hoare(std::vector<int>& arr, int low, int high) {
    if (low < high) {
        int pivot_idx = partition_hoare(arr, low, high);
        quick_sort_hoare(arr, low, pivot_idx);
        quick_sort_hoare(arr, pivot_idx + 1, high);
    }
}

/**
 * @brief Quick Sort with median-of-three pivot selection
 * Reduces chance of worst-case behavior
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_median_of_three(std::vector<int>& arr, int low, int high) {
    if (low < high) {
        // Median-of-three pivot selection
        int mid = low + (high - low) / 2;

        // Sort low, mid, high to find median
        if (arr[mid] < arr[low]) std::swap(arr[low], arr[mid]);
        if (arr[high] < arr[low]) std::swap(arr[low], arr[high]);
        if (arr[high] < arr[mid]) std::swap(arr[mid], arr[high]);

        // Place median at high-1 position
        std::swap(arr[mid], arr[high - 1]);

        // Partition using arr[high-1] as pivot
        int pivot = arr[high - 1];
        int i = low;
        int j = high - 1;

        while (true) {
            while (arr[++i] < pivot);
            while (arr[--j] > pivot);

            if (i >= j) break;
            std::swap(arr[i], arr[j]);
        }

        std::swap(arr[i], arr[high - 1]);

        // Recursively sort partitions
        quick_sort_median_of_three(arr, low, i - 1);
        quick_sort_median_of_three(arr, i + 1, high);
    }
}

/**
 * @brief Iterative Quick Sort using explicit stack
 * Avoids recursion stack overflow
 * @param arr Vector to be sorted
 */
void quick_sort_iterative(std::vector<int>& arr) {
    int low = 0;
    int high = arr.size() - 1;

    // Create auxiliary stack
    std::stack<std::pair<int, int>> stack;

    // Push initial values
    stack.push({low, high});

    while (!stack.empty()) {
        // Pop low and high
        low = stack.top().first;
        high = stack.top().second;
        stack.pop();

        // Partition array and get pivot index
        int pivot_idx = partition_lomuto(arr, low, high);

        // Push left side to stack if there are elements
        if (pivot_idx - 1 > low) {
            stack.push({low, pivot_idx - 1});
        }

        // Push right side to stack if there are elements
        if (pivot_idx + 1 < high) {
            stack.push({pivot_idx + 1, high});
        }
    }
}

/**
 * @brief Quick Sort with random pivot selection
 * Further reduces chance of worst-case behavior
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_random_pivot(std::vector<int>& arr, int low, int high) {
    if (low < high) {
        // Generate random pivot index
        int random_idx = low + rand() % (high - low + 1);
        std::swap(arr[random_idx], arr[high]);

        // Partition with random pivot
        int pivot_idx = partition_lomuto(arr, low, high);

        // Recursively sort partitions
        quick_sort_random_pivot(arr, low, pivot_idx - 1);
        quick_sort_random_pivot(arr, pivot_idx + 1, high);
    }
}

/**
 * @brief Three-way Quick Sort (Dutch National Flag)
 * Efficient for arrays with many duplicate elements
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_three_way(std::vector<int>& arr, int low, int high) {
    if (low >= high) return;

    // Three-way partition
    int pivot = arr[low];
    int lt = low;      // arr[low..lt-1] < pivot
    int gt = high;     // arr[gt+1..high] > pivot
    int i = low + 1;   // arr[lt..i-1] == pivot

    while (i <= gt) {
        if (arr[i] < pivot) {
            std::swap(arr[lt], arr[i]);
            lt++;
            i++;
        } else if (arr[i] > pivot) {
            std::swap(arr[i], arr[gt]);
            gt--;
        } else {
            i++;
        }
    }

    // Recursively sort less and greater partitions
    quick_sort_three_way(arr, low, lt - 1);
    quick_sort_three_way(arr, gt + 1, high);
}

/**
 * @brief Tail-call optimized Quick Sort
 * Reduces stack depth by always recursing on smaller partition
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 */
void quick_sort_tail_optimized(std::vector<int>& arr, int low, int high) {
    while (low < high) {
        int pivot_idx = partition_lomuto(arr, low, high);

        // Recurse on smaller partition first
        if (pivot_idx - low < high - pivot_idx) {
            quick_sort_tail_optimized(arr, low, pivot_idx - 1);
            low = pivot_idx + 1;
        } else {
            quick_sort_tail_optimized(arr, pivot_idx + 1, high);
            high = pivot_idx - 1;
        }
    }
}

/**
 * @brief Hybrid Quick Sort (switches to insertion sort for small subarrays)
 * Combines the benefits of both algorithms
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 * @param cutoff Size below which to use insertion sort
 */
void quick_sort_hybrid(std::vector<int>& arr, int low, int high, int cutoff = 10) {
    if (high - low + 1 <= cutoff) {
        // Use insertion sort for small subarrays
        for (int i = low + 1; i <= high; ++i) {
            int key = arr[i];
            int j = i - 1;
            while (j >= low && arr[j] > key) {
                arr[j + 1] = arr[j];
                j--;
            }
            arr[j + 1] = key;
        }
        return;
    }

    int pivot_idx = partition_lomuto(arr, low, high);
    quick_sort_hybrid(arr, low, pivot_idx - 1, cutoff);
    quick_sort_hybrid(arr, pivot_idx + 1, high, cutoff);
}

/**
 * @brief Quick Sort with custom comparator
 * @tparam T Type that supports comparison
 * @tparam Compare Comparator function type
 * @param arr Vector to be sorted
 * @param low Starting index
 * @param high Ending index
 * @param comp Comparison function
 */
template <typename T, typename Compare>
void quick_sort_custom(std::vector<T>& arr, int low, int high, Compare comp) {
    if (low < high) {
        T pivot = arr[high];
        int i = low - 1;

        for (int j = low; j < high; ++j) {
            if (comp(arr[j], pivot)) {
                i++;
                std::swap(arr[i], arr[j]);
            }
        }
        std::swap(arr[i + 1], arr[high]);

        int pivot_idx = i + 1;
        quick_sort_custom(arr, low, pivot_idx - 1, comp);
        quick_sort_custom(arr, pivot_idx + 1, high, comp);
    }
}

/**
 * @brief Quick Select Algorithm (find kth smallest element)
 * Uses quick sort partitioning to find kth order statistic
 * @param arr Vector to search in
 * @param low Starting index
 * @param high Ending index
 * @param k Position (1-based) to find
 * @return kth smallest element
 */
int quick_select(std::vector<int>& arr, int low, int high, int k) {
    if (low == high) {
        return arr[low];
    }

    int pivot_idx = partition_lomuto(arr, low, high);
    int count = pivot_idx - low + 1;  // Number of elements <= pivot

    if (count == k) {
        return arr[pivot_idx];
    } else if (k < count) {
        return quick_select(arr, low, pivot_idx - 1, k);
    } else {
        return quick_select(arr, pivot_idx + 1, high, k - count);
    }
}

/**
 * @brief Wrapper function for quick select
 * @param arr Vector to search in
 * @param k Position (1-based) to find
 * @return kth smallest element
 */
int find_kth_smallest_quick(std::vector<int> arr, int k) {
    if (k < 1 || k > arr.size()) {
        throw std::out_of_range("k is out of bounds");
    }
    return quick_select(arr, 0, arr.size() - 1, k);
}

/**
 * @brief Test function to demonstrate quick sort variants
 */
void test_quick_sort() {
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

        // Test Lomuto quick sort
        auto arr1 = test_arrays[i];
        if (!arr1.empty()) {
            print_array(arr1, "Original");
            quick_sort_lomuto(arr1, 0, arr1.size() - 1);
            print_array(arr1, "Lomuto quick sort");
        }

        // Test Hoare quick sort
        auto arr2 = test_arrays[i];
        if (!arr2.empty()) {
            quick_sort_hoare(arr2, 0, arr2.size() - 1);
            std::cout << "Hoare quick sort completed" << std::endl;
        }

        // Test three-way quick sort (good for duplicates)
        auto arr3 = test_arrays[i];
        if (!arr3.empty()) {
            quick_sort_three_way(arr3, 0, arr3.size() - 1);
            std::cout << "Three-way quick sort completed" << std::endl;
        }
    }
}

/**
 * @brief Performance comparison of quick sort variants
 */
void compare_quick_sort_variants() {
    // Generate test data
    std::vector<int> test_data(10000);
    for (int i = 0; i < 10000; ++i) {
        test_data[i] = rand() % 10000;
    }

    std::cout << "\n=== Quick Sort Variants Performance (10000 elements) ===" << std::endl;

    // Lomuto partition
    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    quick_sort_lomuto(arr1, 0, arr1.size() - 1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Hoare partition
    auto arr2 = test_data;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_hoare(arr2, 0, arr2.size() - 1);
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Median of three
    auto arr3 = test_data;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_median_of_three(arr3, 0, arr3.size() - 1);
    end = std::chrono::high_resolution_clock::now();
    auto time3 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Iterative
    auto arr4 = test_data;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_iterative(arr4);
    end = std::chrono::high_resolution_clock::now();
    auto time4 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Hybrid
    auto arr5 = test_data;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_hybrid(arr5, 0, arr5.size() - 1);
    end = std::chrono::high_resolution_clock::now();
    auto time5 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Lomuto partition: " << time1.count() << " μs" << std::endl;
    std::cout << "Hoare partition: " << time2.count() << " μs" << std::endl;
    std::cout << "Median of three: " << time3.count() << " μs" << std::endl;
    std::cout << "Iterative: " << time4.count() << " μs" << std::endl;
    std::cout << "Hybrid: " << time5.count() << " μs" << std::endl;
}

/**
 * @brief Demonstrate worst-case scenario
 */
void demonstrate_worst_case() {
    std::cout << "\n=== Worst Case Demonstration ===" << std::endl;

    // Already sorted array (worst case for basic quick sort)
    std::vector<int> worst_case(1000);
    for (int i = 0; i < 1000; ++i) {
        worst_case[i] = i;
    }

    // Test basic quick sort (will be slow)
    auto arr1 = worst_case;
    auto start = std::chrono::high_resolution_clock::now();
    quick_sort_lomuto(arr1, 0, arr1.size() - 1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Test random pivot quick sort (will be fast)
    auto arr2 = worst_case;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_random_pivot(arr2, 0, arr2.size() - 1);
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Test median of three quick sort (will be fast)
    auto arr3 = worst_case;
    start = std::chrono::high_resolution_clock::now();
    quick_sort_median_of_three(arr3, 0, arr3.size() - 1);
    end = std::chrono::high_resolution_clock::now();
    auto time3 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Basic quick sort (worst case): " << time1.count() << " μs" << std::endl;
    std::cout << "Random pivot quick sort: " << time2.count() << " μs" << std::endl;
    std::cout << "Median of three quick sort: " << time3.count() << " μs" << std::endl;
}

/**
 * @brief Test quick select algorithm
 */
void test_quick_select() {
    std::vector<int> arr = {7, 10, 4, 3, 20, 15};
    print_array(arr, "Original array");

    for (int k = 1; k <= arr.size(); ++k) {
        int kth_smallest = find_kth_smallest_quick(arr, k);
        std::cout << k << "th smallest element: " << kth_smallest << std::endl;
    }

    // Test with larger array
    std::vector<int> large_arr(100);
    for (int i = 0; i < 100; ++i) {
        large_arr[i] = rand() % 1000;
    }

    int median = find_kth_smallest_quick(large_arr, 50);
    std::cout << "\nMedian of 100 random numbers: " << median << std::endl;
}