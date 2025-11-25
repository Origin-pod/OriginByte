#pragma once
#include <vector>

/**
 * @brief Interpolation Search Implementation
 *
 * Interpolation Search is an improved variant of binary search for sorted arrays
 * with uniformly distributed values. It estimates the position of the target
 * value based on the values at the current bounds.
 *
 * Time Complexity:
 * - Best Case: O(1) - when target is at estimated position
 * - Average Case: O(log log n) - for uniformly distributed data
 * - Worst Case: O(n) - for non-uniformly distributed data
 *
 * Space Complexity: O(1) - constant extra space
 *
 * Precondition: Array must be sorted and preferably uniformly distributed
 *
 * Formula: pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low])
 *
 * Advantages:
 * - Faster than binary search for uniformly distributed data
 * - More efficient when target values are at extremes
 * - Good performance on numeric data with consistent gaps
 *
 * Disadvantages:
 * - Requires uniformly distributed data for optimal performance
 * - Poor performance on clustered or skewed data
 * - Only works on sorted arrays
 * - More complex than binary search
 *
 * Applications:
 * - Phone book directories
 * - Dictionary searches
 * - Database index searches with uniform distribution
 * - Numerical data analysis
 */

/**
 * @brief Basic Interpolation Search implementation
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_basic(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        if (low == high) {
            if (arr[low] == target) {
                return low;
            }
            return -1;
        }

        // Calculate position using interpolation formula
        int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

        // Handle potential out-of-bounds due to integer division
        if (pos < low) pos = low;
        if (pos > high) pos = high;

        if (arr[pos] == target) {
            return pos;  // Target found
        } else if (arr[pos] < target) {
            low = pos + 1;  // Search right half
        } else {
            high = pos - 1;  // Search left half
        }
    }

    return -1;  // Target not found
}

/**
 * @brief Interpolation Search with comparison count
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param comparisons Output parameter for comparison count
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_with_count(const std::vector<int>& arr, int target, int& comparisons) {
    comparisons = 0;
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        comparisons++;

        if (low == high) {
            if (arr[low] == target) {
                return low;
            }
            return -1;
        }

        // Calculate position using interpolation formula
        int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

        if (pos < low) pos = low;
        if (pos > high) pos = high;

        comparisons++;
        if (arr[pos] == target) {
            return pos;
        } else if (arr[pos] < target) {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }

    return -1;
}

/**
 * @brief Recursive Interpolation Search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @param low Starting index
 * @param high Ending index
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_recursive(const std::vector<int>& arr, int target, int low, int high) {
    if (low > high || target < arr[low] || target > arr[high]) {
        return -1;
    }

    if (low == high) {
        if (arr[low] == target) {
            return low;
        }
        return -1;
    }

    // Calculate position using interpolation formula
    int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

    if (pos < low) pos = low;
    if (pos > high) pos = high;

    if (arr[pos] == target) {
        return pos;
    } else if (arr[pos] < target) {
        return interpolation_search_recursive(arr, target, pos + 1, high);
    } else {
        return interpolation_search_recursive(arr, target, low, pos - 1);
    }
}

/**
 * @brief Wrapper function for recursive interpolation search
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_recursive(const std::vector<int>& arr, int target) {
    if (arr.empty()) return -1;
    return interpolation_search_recursive(arr, target, 0, static_cast<int>(arr.size()) - 1);
}

/**
 * @brief Interpolation Search with safety checks
 * Handles edge cases and prevents division by zero
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_safe(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        // Prevent division by zero
        if (arr[high] == arr[low]) {
            if (arr[low] == target) {
                return low;
            }
            break;
        }

        // Calculate position using interpolation formula
        long long numerator = static_cast<long long>(target - arr[low]) * (high - low);
        int pos = low + static_cast<int>(numerator / (arr[high] - arr[low]));

        // Ensure position is within bounds
        pos = std::max(low, std::min(high, pos));

        if (arr[pos] == target) {
            return pos;
        } else if (arr[pos] < target) {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }

    return -1;
}

/**
 * @brief Interpolation Search for floating-point values
 * @param arr Sorted array of floating-point numbers
 * @param target Value to search for
 * @param epsilon Tolerance for floating-point comparison
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_float(const std::vector<double>& arr, double target, double epsilon = 1e-10) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target >= arr[low] - epsilon && target <= arr[high] + epsilon) {
        if (low == high) {
            if (std::abs(arr[low] - target) <= epsilon) {
                return low;
            }
            break;
        }

        // Calculate position using interpolation formula
        double pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);
        int index = static_cast<int>(pos);

        // Ensure index is within bounds
        index = std::max(low, std::min(high, index));

        if (std::abs(arr[index] - target) <= epsilon) {
            return index;
        } else if (arr[index] < target) {
            low = index + 1;
        } else {
            high = index - 1;
        }
    }

    return -1;
}

/**
 * @brief Interpolation Search with exponential backoff
 * Falls back to binary search when interpolation estimates are poor
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_adaptive(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;
    int consecutive_failures = 0;
    const int MAX_FAILURES = 3;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        if (low == high) {
            if (arr[low] == target) {
                return low;
            }
            break;
        }

        int pos;
        if (consecutive_failures >= MAX_FAILURES) {
            // Fall back to binary search
            pos = low + (high - low) / 2;
        } else {
            // Use interpolation
            if (arr[high] == arr[low]) {
                pos = low;
            } else {
                long long numerator = static_cast<long long>(target - arr[low]) * (high - low);
                pos = low + static_cast<int>(numerator / (arr[high] - arr[low]));
                pos = std::max(low, std::min(high, pos));
            }
        }

        if (arr[pos] == target) {
            return pos;
        } else if (arr[pos] < target) {
            // Check if interpolation was effective
            int expected_mid = low + (high - low) / 2;
            if (pos < expected_mid && consecutive_failures < MAX_FAILURES) {
                consecutive_failures++;
            } else {
                consecutive_failures = 0;
            }
            low = pos + 1;
        } else {
            // Check if interpolation was effective
            int expected_mid = low + (high - low) / 2;
            if (pos > expected_mid && consecutive_failures < MAX_FAILURES) {
                consecutive_failures++;
            } else {
                consecutive_failures = 0;
            }
            high = pos - 1;
        }
    }

    return -1;
}

/**
 * @brief Interpolation Search for descending sorted array
 * @param arr Array sorted in descending order
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int interpolation_search_descending(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target <= arr[low] && target >= arr[high]) {
        if (low == high) {
            if (arr[low] == target) {
                return low;
            }
            break;
        }

        // Modified interpolation formula for descending order
        int pos = low + ((arr[low] - target) * (high - low)) / (arr[low] - arr[high]);

        if (pos < low) pos = low;
        if (pos > high) pos = high;

        if (arr[pos] == target) {
            return pos;
        } else if (arr[pos] > target) {
            low = pos + 1;  // Search right half
        } else {
            high = pos - 1;  // Search left half
        }
    }

    return -1;
}

/**
 * @brief Generic Interpolation Search for any comparable type
 * @tparam T Type that supports comparison and arithmetic operations
 * @param arr Sorted array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
template <typename T>
int interpolation_search_generic(const std::vector<T>& arr, const T& target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        if (low == high) {
            if (arr[low] == target) {
                return low;
            }
            return -1;
        }

        // Calculate position using interpolation formula
        T pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);
        int index = static_cast<int>(pos);

        if (index < low) index = low;
        if (index > high) index = high;

        if (arr[index] == target) {
            return index;
        } else if (arr[index] < target) {
            low = index + 1;
        } else {
            high = index - 1;
        }
    }

    return -1;
}

/**
 * @brief Find insertion point using interpolation search
 * @param arr Sorted array to search in
 * @param target Value to find insertion point for
 * @return Index where target should be inserted
 */
int interpolation_search_insertion_point(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = static_cast<int>(arr.size()) - 1;

    if (target < arr[0]) return 0;
    if (target > arr[high]) return high + 1;

    while (low <= high) {
        if (low == high) {
            return (arr[low] < target) ? low + 1 : low;
        }

        int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);
        pos = std::max(low, std::min(high, pos));

        if (arr[pos] == target) {
            // Find first occurrence
            while (pos > 0 && arr[pos - 1] == target) {
                pos--;
            }
            return pos;
        } else if (arr[pos] < target) {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }

    return low;
}

/**
 * @brief Test function to demonstrate interpolation search variants
 */
void test_interpolation_search() {
    // Test with uniformly distributed data (optimal case)
    std::vector<int> uniform_data;
    for (int i = 0; i < 100; ++i) {
        uniform_data.push_back(i * 2);  // Even numbers: 0, 2, 4, ..., 198
    }

    std::cout << "=== Testing Uniformly Distributed Data ===" << std::endl;
    print_array(std::vector<int>(uniform_data.begin(), uniform_data.begin() + 20),
                "First 20 elements of uniform data");

    int targets[] = {0, 25, 50, 75, 99, 100, 150, 198, 200};

    for (int target : targets) {
        std::cout << "\nSearching for " << target << ":" << std::endl;

        // Basic interpolation search
        int result1 = interpolation_search_basic(uniform_data, target);
        std::cout << "Basic: " << result1 << std::endl;

        // With comparison count
        int comparisons;
        int result2 = interpolation_search_with_count(uniform_data, target, comparisons);
        std::cout << "With count: " << result2 << ", comparisons: " << comparisons << std::endl;

        // Safe interpolation search
        int result3 = interpolation_search_safe(uniform_data, target);
        std::cout << "Safe: " << result3 << std::endl;

        // Adaptive interpolation search
        int result4 = interpolation_search_adaptive(uniform_data, target);
        std::cout << "Adaptive: " << result4 << std::endl;

        // Compare with binary search
        int result5 = binary_search_iterative(uniform_data, target);
        std::cout << "Binary search: " << result5 << std::endl;
    }
}

/**
 * @brief Test with non-uniformly distributed data
 */
void test_non_uniform_data() {
    std::cout << "\n=== Testing Non-Uniformly Distributed Data ===" << std::endl;

    // Create skewed data: many small numbers, few large numbers
    std::vector<int> skewed_data;
    for (int i = 0; i < 50; ++i) {
        skewed_data.push_back(i);  // 0-49
    }
    for (int i = 0; i < 10; ++i) {
        skewed_data.push_back(100 + i * 100);  // 100, 200, ..., 1000
    }

    print_array(skewed_data, "Skewed data");

    int targets[] = {25, 50, 100, 500, 1000};

    for (int target : targets) {
        std::cout << "\nSearching for " << target << ":" << std::endl;

        // Interpolation search (may perform poorly)
        auto start = std::chrono::high_resolution_clock::now();
        int result1 = interpolation_search_basic(skewed_data, target);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        // Binary search (consistent performance)
        start = std::chrono::high_resolution_clock::now();
        int result2 = binary_search_iterative(skewed_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        std::cout << "Interpolation: " << result1 << " (" << time1.count() << " ns)" << std::endl;
        std::cout << "Binary search: " << result2 << " (" << time2.count() << " ns)" << std::endl;
    }
}

/**
 * @brief Performance comparison analysis
 */
void analyze_performance() {
    std::cout << "\n=== Performance Analysis ===" << std::endl;

    // Generate different types of data distributions
    const int SIZE = 10000;

    // Uniform distribution
    std::vector<int> uniform_data;
    for (int i = 0; i < SIZE; ++i) {
        uniform_data.push_back(i);
    }

    // Exponential distribution
    std::vector<int> exponential_data;
    for (int i = 0; i < SIZE; ++i) {
        exponential_data.push_back(static_cast<int>(pow(1.01, i)));
    }
    sort(exponential_data.begin(), exponential_data.end());

    std::cout << "Array size: " << SIZE << std::endl;
    std::cout << "Testing with targets at different positions:" << std::endl;

    int positions[] = {SIZE * 0.01, SIZE * 0.25, SIZE * 0.5, SIZE * 0.75, SIZE * 0.99};

    std::cout << "\nUniform Distribution:" << std::endl;
    for (int pos : positions) {
        int target = uniform_data[pos];

        auto start = std::chrono::high_resolution_clock::now();
        int result1 = interpolation_search_basic(uniform_data, target);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        start = std::chrono::high_resolution_clock::now();
        int result2 = binary_search_iterative(uniform_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        std::cout << "Position " << pos << ": Interpolation=" << time1.count()
                  << "ns, Binary=" << time2.count() << "ns" << std::endl;
    }

    std::cout << "\nExponential Distribution:" << std::endl;
    for (int pos : positions) {
        int target = exponential_data[pos];

        auto start = std::chrono::high_resolution_clock::now();
        int result1 = interpolation_search_basic(exponential_data, target);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        start = std::chrono::high_resolution_clock::now();
        int result2 = binary_search_iterative(exponential_data, target);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

        std::cout << "Position " << pos << ": Interpolation=" << time1.count()
                  << "ns, Binary=" << time2.count() << "ns" << std::endl;
    }
}

/**
 * @brief Test floating-point interpolation search
 */
void test_floating_point_search() {
    std::cout << "\n=== Testing Floating-Point Interpolation Search ===" << std::endl;

    std::vector<double> float_data;
    for (int i = 0; i < 100; ++i) {
        float_data.push_back(i * 0.5);  // 0.0, 0.5, 1.0, ..., 49.5
    }

    double targets[] = {0.0, 12.5, 25.0, 37.5, 49.5, 23.7};

    for (double target : targets) {
        int result = interpolation_search_float(float_data, target);
        std::cout << "Target " << target << ": found at index " << result;
        if (result != -1) {
            std::cout << " (value: " << float_data[result] << ")";
        }
        std::cout << std::endl;
    }
}