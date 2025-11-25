#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Binary Search Implementation
 *
 * Binary Search is a divide-and-conquer algorithm that searches for a target value
 * in a sorted array by repeatedly dividing the search interval in half.
 *
 * Time Complexity:
 * - Best Case: O(1) - when target is at middle
 * - Average Case: O(log n)
 * - Worst Case: O(log n)
 *
 * Space Complexity:
 * - Iterative: O(1)
 * - Recursive: O(log n) for recursion stack
 *
 * Precondition: Array must be sorted
 *
 * Advantages:
 * - Very efficient for large datasets
 * - Guaranteed logarithmic time complexity
 * - Simple implementation
 *
 * Disadvantages:
 * - Requires sorted data
 * - Not suitable for linked lists
 * - Poor performance on frequently changing data
 *
 * Applications:
 * - Searching in large databases
 * - Dictionary lookups
 * - Game development (AI decision trees)
 * - File system searches
 */

/**
 * @brief Iterative Binary Search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int binary_search_iterative(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;  // Prevent overflow

        if (arr[mid] == target) {
            return mid;  // Target found
        } else if (arr[mid] < target) {
            left = mid + 1;  // Search right half
        } else {
            right = mid - 1;  // Search left half
        }
    }

    return -1;  // Target not found
}

/**
 * @brief Recursive Binary Search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param left Starting index
 * @param right Ending index
 * @return Index of target if found, -1 otherwise
 */
int binary_search_recursive(const std::vector<int>& arr, int target, int left, int right) {
    if (left > right) {
        return -1;  // Base case: target not found
    }

    int mid = left + (right - left) / 2;

    if (arr[mid] == target) {
        return mid;  // Target found
    } else if (arr[mid] < target) {
        return binary_search_recursive(arr, target, mid + 1, right);
    } else {
        return binary_search_recursive(arr, target, left, mid - 1);
    }
}

/**
 * @brief Wrapper function for recursive binary search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int binary_search_recursive(const std::vector<int>& arr, int target) {
    return binary_search_recursive(arr, target, 0, static_cast<int>(arr.size()) - 1);
}

/**
 * @brief Binary Search with comparison count
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param comparisons Output parameter for comparison count
 * @return Index of target if found, -1 otherwise
 */
int binary_search_with_count(const std::vector<int>& arr, int target, int& comparisons) {
    comparisons = 0;
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;
        comparisons++;

        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

/**
 * @brief Find first occurrence of target (for arrays with duplicates)
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of first occurrence, -1 if not found
 */
int binary_search_first_occurrence(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;
    int result = -1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            result = mid;  // Store current position
            right = mid - 1;  // Continue searching left half
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return result;
}

/**
 * @brief Find last occurrence of target (for arrays with duplicates)
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of last occurrence, -1 if not found
 */
int binary_search_last_occurrence(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;
    int result = -1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            result = mid;  // Store current position
            left = mid + 1;  // Continue searching right half
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return result;
}

/**
 * @brief Find ceiling of target (smallest element >= target)
 * @param arr Sorted array to search in
 * @param target Value to find ceiling for
 * @return Index of ceiling element, -1 if no ceiling exists
 */
int binary_search_ceiling(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;
    int result = -1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] >= target) {
            result = mid;  // Potential ceiling found
            right = mid - 1;  // Try to find smaller ceiling
        } else {
            left = mid + 1;
        }
    }

    return result;
}

/**
 * @brief Find floor of target (largest element <= target)
 * @param arr Sorted array to search in
 * @param target Value to find floor for
 * @return Index of floor element, -1 if no floor exists
 */
int binary_search_floor(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;
    int result = -1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] <= target) {
            result = mid;  // Potential floor found
            left = mid + 1;  // Try to find larger floor
        } else {
            right = mid - 1;
        }
    }

    return result;
}

/**
 * @brief Find first element greater than target
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of first element greater than target, -1 if none exists
 */
int binary_search_upper_bound(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;
    int result = -1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] > target) {
            result = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return result;
}

/**
 * @brief Binary Search on descending sorted array
 * @param arr Array sorted in descending order
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int binary_search_descending(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] > target) {
            left = mid + 1;  // Search right half (smaller values)
        } else {
            right = mid - 1;  // Search left half (larger values)
        }
    }

    return -1;
}

/**
 * @brief Generic Binary Search for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
template <typename T>
int binary_search_generic(const std::vector<T>& arr, const T& target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

/**
 * @brief Binary Search with custom comparator
 * @tparam T Type of elements
 * @tparam Compare Comparator function type
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param comp Comparison function
 * @return Index of target if found, -1 otherwise
 */
template <typename T, typename Compare>
int binary_search_custom(const std::vector<T>& arr, const T& target, Compare comp) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        } else if (comp(arr[mid], target)) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

/**
 * @brief Exponential Search (useful when array size is unknown)
 * Combines exponential range finding with binary search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int exponential_search(const std::vector<int>& arr, int target) {
    if (arr.empty()) return -1;

    // If target is at first position
    if (arr[0] == target) return 0;

    // Find range for binary search by repeated doubling
    int i = 1;
    int n = static_cast<int>(arr.size());
    while (i < n && arr[i] <= target) {
        i *= 2;
    }

    // Perform binary search on found range
    int left = i / 2;
    int right = std::min(i, n - 1);

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

/**
 * @brief Binary Search in rotated sorted array
 * @param arr Rotated sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int binary_search_rotated(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        }

        // Check if left half is sorted
        if (arr[left] <= arr[mid]) {
            // Target is in left half
            if (arr[left] <= target && target < arr[mid]) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // Right half is sorted
            if (arr[mid] < target && target <= arr[right]) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    return -1;
}

/**
 * @brief Find insertion point for target (equivalent to std::lower_bound)
 * @param arr Sorted array to search in
 * @param target Value to find insertion point for
 * @return Index where target should be inserted
 */
int binary_search_insertion_point(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size());

    while (left < right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    return left;
}

/**
 * @brief Count occurrences of target using binary search
 * @param arr Sorted array to search in
 * @param target Value to count
 * @return Number of occurrences of target
 */
int count_occurrences_binary(const std::vector<int>& arr, int target) {
    int first = binary_search_first_occurrence(arr, target);
    if (first == -1) return 0;

    int last = binary_search_last_occurrence(arr, target);
    return last - first + 1;
}

/**
 * @brief Find range containing all occurrences of target
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Pair containing (first_index, last_index), (-1, -1) if not found
 */
std::pair<int, int> binary_search_range(const std::vector<int>& arr, int target) {
    int first = binary_search_first_occurrence(arr, target);
    if (first == -1) return {-1, -1};

    int last = binary_search_last_occurrence(arr, target);
    return {first, last};
}

/**
 * @brief Test function to demonstrate binary search variants
 */
void test_binary_search() {
    std::vector<int> test_arrays[] = {
        {1, 3, 5, 7, 9, 11, 13, 15, 17, 19},
        {2, 4, 6, 8, 10, 12, 14, 16, 18, 20},
        {1, 1, 1, 1, 1, 1, 1, 1, 1, 1},  // All same
        {1, 2, 3, 4, 5, 6, 7, 8, 9, 10},   // Sequential
        {1},                                // Single element
        {}                                  // Empty array
    };

    std::string test_names[] = {
        "Odd positions",
        "Even positions",
        "All same elements",
        "Sequential",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 6; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;
        print_array(test_arrays[i], "Array");

        if (!test_arrays[i].empty()) {
            // Test various targets
            int targets[] = {1, 5, 10, 20};
            for (int target : targets) {
                std::cout << "\nSearching for " << target << ":" << std::endl;

                // Iterative binary search
                int result1 = binary_search_iterative(test_arrays[i], target);
                std::cout << "Iterative: " << result1 << std::endl;

                // Recursive binary search
                int result2 = binary_search_recursive(test_arrays[i], target);
                std::cout << "Recursive: " << result2 << std::endl;

                // With comparison count
                int comparisons;
                int result3 = binary_search_with_count(test_arrays[i], target, comparisons);
                std::cout << "With count: " << result3 << ", comparisons: " << comparisons << std::endl;

                // First and last occurrence
                int first = binary_search_first_occurrence(test_arrays[i], target);
                int last = binary_search_last_occurrence(test_arrays[i], target);
                std::cout << "First: " << first << ", Last: " << last << std::endl;

                // Ceiling and floor
                int ceiling = binary_search_ceiling(test_arrays[i], target);
                int floor = binary_search_floor(test_arrays[i], target);
                std::cout << "Ceiling: " << ceiling << ", Floor: " << floor << std::endl;
            }
        }
    }
}

/**
 * @brief Test binary search on rotated array
 */
void test_rotated_binary_search() {
    std::cout << "\n=== Testing Rotated Array Binary Search ===" << std::endl;

    std::vector<std::vector<int>> rotated_arrays = {
        {4, 5, 6, 7, 0, 1, 2},
        {30, 40, 50, 10, 20},
        {5, 6, 7, 1, 2, 3, 4}
    };

    for (const auto& arr : rotated_arrays) {
        print_array(arr, "Rotated array");

        // Test various targets
        for (int target : {0, 5, 10, 30, 50}) {
            int result = binary_search_rotated(arr, target);
            std::cout << "Search " << target << ": " << result;
            if (result != -1) {
                std::cout << " (value: " << arr[result] << ")";
            }
            std::cout << std::endl;
        }
        std::cout << std::endl;
    }
}

/**
 * @brief Performance comparison with linear search
 */
void compare_search_performance() {
    // Generate sorted test data
    std::vector<int> sorted_data(100000);
    for (int i = 0; i < 100000; ++i) {
        sorted_data[i] = i;
    }

    int targets[] = {0, 50000, 99999, 100000};  // First, middle, last, not found

    std::cout << "\n=== Search Performance Comparison (100000 elements) ===" << std::endl;

    for (int target : targets) {
        // Binary Search
        auto start = std::chrono::high_resolution_clock::now();
        int result1 = binary_search_iterative(sorted_data, target);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        // Linear Search
        start = std::chrono::high_resolution_clock::now();
        int result2 = linear_search_basic(sorted_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        std::cout << "Target " << target << ":" << std::endl;
        std::cout << "  Binary Search: " << time1.count() << " ns" << std::endl;
        std::cout << "  Linear Search: " << time2.count() << " ns" << std::endl;
        std::cout << "  Speedup: " << (time2.count() / time1.count()) << "x" << std::endl;
        std::cout << std::endl;
    }
}

/**
 * @brief Test exponential search
 */
void test_exponential_search() {
    std::cout << "\n=== Testing Exponential Search ===" << std::endl;

    std::vector<int> arr;
    for (int i = 0; i < 1000; i += 2) {  // Even numbers from 0 to 998
        arr.push_back(i);
    }

    int targets[] = {0, 123, 500, 998, 999};

    for (int target : targets) {
        int result1 = exponential_search(arr, target);
        int result2 = binary_search_iterative(arr, target);

        std::cout << "Target " << target << ": Exponential=" << result1
                  << ", Binary=" << result2 << std::endl;
    }
}