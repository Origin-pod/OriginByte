#pragma once
#include <vector>
#include <cmath>

/**
 * @brief Jump Search Implementation
 *
 * Jump Search is a searching algorithm for sorted arrays. The basic idea is to
 * check fewer elements by jumping ahead by fixed steps and then performing a
 * linear search for the block where the element could be present.
 *
 * Time Complexity:
 * - Best Case: O(1) - when target is at the beginning
 * - Average Case: O(√n)
 * - Worst Case: O(√n)
 *
 * Space Complexity: O(1) - constant extra space
 *
 * Precondition: Array must be sorted
 *
 * Optimal block size: √n
 *
 * Advantages:
 * - Better than linear search for large arrays
 * - Simpler than binary search
 * - Fewer comparisons than linear search
 *
 * Disadvantages:
 * - Requires sorted data
 * - Slower than binary search (O(log n))
 * - Not suitable for linked lists
 *
 * Applications:
 * - When data is sorted but binary search is too complex
 * - When random access is expensive (reduces number of jumps)
 * - Educational purposes
 */

/**
 * @brief Basic Jump Search implementation
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int jump_search_basic(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    // Optimal block size
    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find the block where target could be present
    while (arr[std::min(step, n) - 1] < target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return -1;  // Target not found
        }
    }

    // Linear search in the identified block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] == target) {
            return i;
        }
    }

    return -1;  // Target not found
}

/**
 * @brief Jump Search with custom block size
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param block_size Size of each jump block
 * @return Index of target if found, -1 otherwise
 */
int jump_search_custom_block(const std::vector<int>& arr, int target, int block_size) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    int prev = 0;
    int step = block_size;

    // Find the block where target could be present
    while (step < n && arr[step] < target) {
        prev = step;
        step += block_size;
        if (prev >= n) {
            return -1;
        }
    }

    // Adjust step to not exceed array bounds
    step = std::min(step, n - 1);

    // Linear search in the identified block
    for (int i = prev; i <= step; ++i) {
        if (arr[i] == target) {
            return i;
        }
    }

    return -1;
}

/**
 * @brief Jump Search with comparison count
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param comparisons Output parameter for comparison count
 * @return Index of target if found, -1 otherwise
 */
int jump_search_with_count(const std::vector<int>& arr, int target, int& comparisons) {
    comparisons = 0;
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target could be present
    while (arr[std::min(step, n) - 1] < target) {
        comparisons++;
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return -1;
        }
    }
    comparisons++;  // Final comparison that fails

    // Linear search in the block
    for (int i = prev; i < std::min(step, n); ++i) {
        comparisons++;
        if (arr[i] == target) {
            return i;
        }
    }

    return -1;
}

/**
 * @brief Bidirectional Jump Search
 * Jumps forward and backward from different starting points
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int jump_search_bidirectional(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    int step = static_cast<int>(sqrt(n));

    // Start from both ends
    int left_step = 0;
    int right_step = n - 1;

    // Check if target is at extremes
    if (arr[0] == target) return 0;
    if (arr[n - 1] == target) return n - 1;

    // Search from both ends
    while (left_step < right_step) {
        // Jump from left
        left_step = std::min(left_step + step, right_step);
        if (arr[left_step] >= target) {
            // Linear search in this block from left
            for (int i = left_step - step; i <= left_step && i < n; ++i) {
                if (arr[i] == target) {
                    return i;
                }
            }
        }

        // Jump from right
        right_step = std::max(right_step - step, left_step);
        if (arr[right_step] <= target) {
            // Linear search in this block from right
            for (int i = right_step; i <= right_step + step && i < n; ++i) {
                if (arr[i] == target) {
                    return i;
                }
            }
        }
    }

    return -1;
}

/**
 * @brief Adaptive Jump Search
 * Adjusts block size based on data distribution
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int jump_search_adaptive(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    // Estimate data density to adjust block size
    if (n > 1) {
        int range = arr[n - 1] - arr[0];
        int density = range / n;

        // Adjust block size based on density
        int step;
        if (density < 10) {
            step = static_cast<int>(sqrt(n) * 2);  // Larger jumps for dense data
        } else if (density > 100) {
            step = static_cast<int>(sqrt(n) / 2);  // Smaller jumps for sparse data
        } else {
            step = static_cast<int>(sqrt(n));  // Standard optimal size
        }

        step = std::max(1, step);  // Ensure step is at least 1

        return jump_search_custom_block(arr, target, step);
    }

    return jump_search_basic(arr, target);
}

/**
 * @brief Jump Search for descending sorted array
 * @param arr Array sorted in descending order
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int jump_search_descending(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target could be present
    while (arr[std::min(step, n) - 1] > target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return -1;
        }
    }

    // Linear search in the identified block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] == target) {
            return i;
        }
    }

    return -1;
}

/**
 * @brief Jump Search with early termination
 * Stops early if target is found or if it's clear target doesn't exist
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int jump_search_optimized(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    // Check if target is outside the range
    if (target < arr[0] || target > arr[n - 1]) {
        return -1;
    }

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target could be present
    while (prev < n && arr[std::min(step, n) - 1] < target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
    }

    // Linear search in the identified block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] == target) {
            return i;
        }
        // Early termination if we've passed the target
        if (arr[i] > target) {
            break;
        }
    }

    return -1;
}

/**
 * @brief Find all occurrences using Jump Search
 * @param arr Sorted array with possible duplicates
 * @param target Value to search for
 * @return Vector containing all indices where target is found
 */
std::vector<int> jump_search_all_occurrences(const std::vector<int>& arr, int target) {
    std::vector<int> result;
    int n = static_cast<int>(arr.size());
    if (n == 0) return result;

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target could be present
    while (arr[std::min(step, n) - 1] < target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return result;
        }
    }

    // Linear search in the identified block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] == target) {
            result.push_back(i);
        }
    }

    return result;
}

/**
 * @brief Generic Jump Search for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
template <typename T>
int jump_search_generic(const std::vector<T>& arr, const T& target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return -1;

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target could be present
    while (arr[std::min(step, n) - 1] < target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return -1;
        }
    }

    // Linear search in the identified block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] == target) {
            return i;
        }
    }

    return -1;
}

/**
 * @brief Find approximate position using Jump Search
 * Returns the position where target would be inserted
 * @param arr Sorted array to search in
 * @param target Value to find approximate position for
 * @return Index where target should be inserted
 */
int jump_search_insertion_point(const std::vector<int>& arr, int target) {
    int n = static_cast<int>(arr.size());
    if (n == 0) return 0;

    // If target is smaller than all elements
    if (target < arr[0]) return 0;

    // If target is larger than all elements
    if (target > arr[n - 1]) return n;

    int step = static_cast<int>(sqrt(n));
    int prev = 0;

    // Find block where target should be inserted
    while (arr[std::min(step, n) - 1] < target) {
        prev = step;
        step += static_cast<int>(sqrt(n));
        if (prev >= n) {
            return n;
        }
    }

    // Find exact insertion point in the block
    for (int i = prev; i < std::min(step, n); ++i) {
        if (arr[i] >= target) {
            return i;
        }
    }

    return std::min(step, n);
}

/**
 * @brief Test function to demonstrate jump search variants
 */
void test_jump_search() {
    std::vector<int> test_arrays[] = {
        {1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25},
        {2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24},
        {10, 20, 30, 40, 50, 60, 70, 80, 90, 100},
        {1, 2, 3, 4, 5, 6, 7, 8, 9, 10},
        {1},  // Single element
        {}    // Empty array
    };

    std::string test_names[] = {
        "Odd positions",
        "Even positions",
        "Multiples of 10",
        "Sequential",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 6; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;
        print_array(test_arrays[i], "Array");

        if (!test_arrays[i].empty()) {
            // Test various targets
            int targets[] = {1, 5, 10, 20, 25};
            for (int target : targets) {
                std::cout << "\nSearching for " << target << ":" << std::endl;

                // Basic jump search
                int result1 = jump_search_basic(test_arrays[i], target);
                std::cout << "Basic Jump Search: " << result1 << std::endl;

                // With comparison count
                int comparisons;
                int result2 = jump_search_with_count(test_arrays[i], target, comparisons);
                std::cout << "With count: " << result2 << ", comparisons: " << comparisons << std::endl;

                // Optimized jump search
                int result3 = jump_search_optimized(test_arrays[i], target);
                std::cout << "Optimized: " << result3 << std::endl;

                // Adaptive jump search
                int result4 = jump_search_adaptive(test_arrays[i], target);
                std::cout << "Adaptive: " << result4 << std::endl;
            }
        }
    }
}

/**
 * @brief Test jump search with different block sizes
 */
void test_block_sizes() {
    std::cout << "\n=== Testing Different Block Sizes ===" << std::endl;

    std::vector<int> arr;
    for (int i = 0; i < 100; ++i) {
        arr.push_back(i * 2);  // Even numbers from 0 to 198
    }

    int target = 76;
    int optimal_size = static_cast<int>(sqrt(arr.size()));

    std::cout << "Array size: " << arr.size() << std::endl;
    std::cout << "Optimal block size: " << optimal_size << std::endl;
    std::cout << "Searching for target: " << target << std::endl;

    // Test different block sizes
    int block_sizes[] = {2, 5, optimal_size, 15, 20};
    for (int block_size : block_sizes) {
        int comparisons;
        int result = jump_search_custom_block(arr, target, block_size);
        std::cout << "Block size " << block_size << ": index " << result << std::endl;
    }
}

/**
 * @brief Performance comparison with other search algorithms
 */
void compare_search_performance() {
    // Generate sorted test data
    std::vector<int> sorted_data(10000);
    for (int i = 0; i < 10000; ++i) {
        sorted_data[i] = i;
    }

    int targets[] = {0, 2500, 5000, 7500, 9999, 10000};

    std::cout << "\n=== Search Performance Comparison (10000 elements) ===" << std::endl;

    for (int target : targets) {
        // Jump Search
        auto start = std::chrono::high_resolution_clock::now();
        int result1 = jump_search_basic(sorted_data, target);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        // Binary Search
        start = std::chrono::high_resolution_clock::now();
        int result2 = binary_search_iterative(sorted_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        // Linear Search
        start = std::chrono::high_resolution_clock::now();
        int result3 = linear_search_basic(sorted_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time3 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        std::cout << "Target " << target << ":" << std::endl;
        std::cout << "  Jump Search: " << time1.count() << " ns" << std::endl;
        std::cout << "  Binary Search: " << time2.count() << " ns" << std::endl;
        std::cout << "  Linear Search: " << time3.count() << " ns" << std::endl;
        std::cout << std::endl;
    }
}

/**
 * @brief Test descending order jump search
 */
void test_descending_search() {
    std::cout << "\n=== Testing Descending Order Jump Search ===" << std::endl;

    std::vector<int> descending_arr;
    for (int i = 100; i >= 0; --i) {
        descending_arr.push_back(i);
    }

    print_array(descending_arr, "Descending array (first 20 elements)");

    int targets[] = {0, 25, 50, 75, 100};

    for (int target : targets) {
        int result = jump_search_descending(descending_arr, target);
        std::cout << "Target " << target << ": found at index " << result;
        if (result != -1) {
            std::cout << " (value: " << descending_arr[result] << ")";
        }
        std::cout << std::endl;
    }
}