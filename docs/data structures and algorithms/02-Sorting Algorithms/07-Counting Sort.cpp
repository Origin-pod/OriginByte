#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Counting Sort Implementation
 *
 * Counting Sort is a non-comparison-based sorting algorithm that works by counting
 * the number of occurrences of each distinct element in the input array. It's
 * particularly efficient when the range of input values is not significantly
 * greater than the number of elements.
 *
 * Time Complexity:
 * - Best Case: O(n + k) where k is the range of input
 * - Average Case: O(n + k)
 * - Worst Case: O(n + k)
 *
 * Space Complexity: O(k) - additional space for counting array
 *
 * Stable: Yes (can be implemented as stable)
 *
 * Advantages:
 * - Linear time complexity when k = O(n)
 * - Stable sort
 * - Simple to implement
 * - No comparisons needed
 *
 * Disadvantages:
 * - Only works for integers or objects that can be mapped to integers
 * - Requires extra space proportional to the range of input
 * - Not suitable for large range values
 *
 * Applications:
 * - Sorting integers with limited range
 * - As subroutine in radix sort
 * - When stability is required
 * - Character string sorting
 */

/**
 * @brief Basic Counting Sort implementation
 * @param arr Vector to be sorted
 * @param max_val Maximum value in the array
 */
void counting_sort_basic(std::vector<int>& arr, int max_val) {
    int n = arr.size();
    if (n == 0) return;

    // Create count array to store count of each element
    std::vector<int> count(max_val + 1, 0);

    // Store count of each character
    for (int i = 0; i < n; ++i) {
        count[arr[i]]++;
    }

    // Build the output array
    int index = 0;
    for (int i = 0; i <= max_val; ++i) {
        while (count[i] > 0) {
            arr[index++] = i;
            count[i]--;
        }
    }
}

/**
 * @brief Stable Counting Sort implementation
 * Maintains relative order of equal elements
 * @param arr Vector to be sorted
 * @param max_val Maximum value in the array
 */
void counting_sort_stable(std::vector<int>& arr, int max_val) {
    int n = arr.size();
    if (n == 0) return;

    // Create count array
    std::vector<int> count(max_val + 1, 0);

    // Store count of each element
    for (int i = 0; i < n; ++i) {
        count[arr[i]]++;
    }

    // Change count[i] so that count[i] contains actual position
    // of this element in output array
    for (int i = 1; i <= max_val; ++i) {
        count[i] += count[i - 1];
    }

    // Build output array (work from right to left for stability)
    std::vector<int> output(n);
    for (int i = n - 1; i >= 0; --i) {
        output[count[arr[i]] - 1] = arr[i];
        count[arr[i]]--;
    }

    // Copy output array to arr
    for (int i = 0; i < n; ++i) {
        arr[i] = output[i];
    }
}

/**
 * @brief Counting Sort with automatic range detection
 * Automatically finds min and max values
 * @param arr Vector to be sorted
 */
void counting_sort_auto_range(std::vector<int>& arr) {
    int n = arr.size();
    if (n == 0) return;

    // Find min and max values
    int min_val = arr[0];
    int max_val = arr[0];
    for (int i = 1; i < n; ++i) {
        min_val = std::min(min_val, arr[i]);
        max_val = std::max(max_val, arr[i]);
    }

    int range = max_val - min_val + 1;

    // Create count array
    std::vector<int> count(range, 0);

    // Store count of each element (adjusted for min_val)
    for (int i = 0; i < n; ++i) {
        count[arr[i] - min_val]++;
    }

    // Build output array
    int index = 0;
    for (int i = 0; i < range; ++i) {
        while (count[i] > 0) {
            arr[index++] = i + min_val;
            count[i]--;
        }
    }
}

/**
 * @brief Counting Sort for negative numbers
 * Handles both positive and negative integers
 * @param arr Vector to be sorted
 */
void counting_sort_with_negatives(std::vector<int>& arr) {
    int n = arr.size();
    if (n == 0) return;

    // Separate positive and negative numbers
    std::vector<int> positives, negatives;

    for (int num : arr) {
        if (num >= 0) {
            positives.push_back(num);
        } else {
            negatives.push_back(-num);  // Store absolute value
        }
    }

    // Sort positive numbers
    if (!positives.empty()) {
        int max_pos = *std::max_element(positives.begin(), positives.end());
        counting_sort_basic(positives, max_pos);
    }

    // Sort negative numbers (sort by absolute value, then reverse)
    if (!negatives.empty()) {
        int max_neg = *std::max_element(negatives.begin(), negatives.end());
        counting_sort_basic(negatives, max_neg);
        // Reverse and convert back to negative
        std::reverse(negatives.begin(), negatives.end());
        for (int& num : negatives) {
            num = -num;
        }
    }

    // Combine results (negatives first, then positives)
    int index = 0;
    for (int num : negatives) {
        arr[index++] = num;
    }
    for (int num : positives) {
        arr[index++] = num;
    }
}

/**
 * @brief Counting Sort for characters
 * Sorts an array of characters
 * @param arr Vector of characters to be sorted
 */
void counting_sort_chars(std::vector<char>& arr) {
    int n = arr.size();
    if (n == 0) return;

    // Assuming ASCII characters (0-255)
    const int RANGE = 256;
    std::vector<int> count(RANGE, 0);

    // Count occurrences
    for (int i = 0; i < n; ++i) {
        count[static_cast<unsigned char>(arr[i])]++;
    }

    // Build output array
    int index = 0;
    for (int i = 0; i < RANGE; ++i) {
        while (count[i] > 0) {
            arr[index++] = static_cast<char>(i);
            count[i]--;
        }
    }
}

/**
 * @brief Counting Sort with frequency analysis
 * Returns frequency of each element along with sorted array
 * @param arr Vector to be sorted
 * @param max_val Maximum value in the array
 * @return Vector containing frequency of each value
 */
std::vector<int> counting_sort_with_frequency(std::vector<int>& arr, int max_val) {
    int n = arr.size();
    if (n == 0) return {};

    // Create count array
    std::vector<int> count(max_val + 1, 0);

    // Count occurrences
    for (int i = 0; i < n; ++i) {
        count[arr[i]]++;
    }

    // Store frequency for return
    std::vector<int> frequency = count;

    // Build output array
    int index = 0;
    for (int i = 0; i <= max_val; ++i) {
        while (count[i] > 0) {
            arr[index++] = i;
            count[i]--;
        }
    }

    return frequency;
}

/**
 * @brief Counting Sort for sorting in descending order
 * @param arr Vector to be sorted
 * @param max_val Maximum value in the array
 */
void counting_sort_descending(std::vector<int>& arr, int max_val) {
    int n = arr.size();
    if (n == 0) return;

    // Create count array
    std::vector<int> count(max_val + 1, 0);

    // Count occurrences
    for (int i = 0; i < n; ++i) {
        count[arr[i]]++;
    }

    // Build output array in descending order
    int index = 0;
    for (int i = max_val; i >= 0; --i) {
        while (count[i] > 0) {
            arr[index++] = i;
            count[i]--;
        }
    }
}

/**
 * @brief Partial Counting Sort (sort only first k elements)
 * Finds k smallest elements using counting sort approach
 * @param arr Vector to be partially sorted
 * @param k Number of smallest elements to find
 */
void counting_sort_partial(std::vector<int>& arr, int k) {
    int n = arr.size();
    if (k <= 0 || k > n) return;

    // Find min and max values
    int min_val = *std::min_element(arr.begin(), arr.end());
    int max_val = *std::max_element(arr.begin(), arr.end());

    int range = max_val - min_val + 1;
    std::vector<int> count(range, 0);

    // Count occurrences
    for (int i = 0; i < n; ++i) {
        count[arr[i] - min_val]++;
    }

    // Place k smallest elements at beginning
    int index = 0;
    for (int i = 0; i < range && index < k; ++i) {
        while (count[i] > 0 && index < k) {
            arr[index++] = i + min_val;
            count[i]--;
        }
    }
}

/**
 * @brief Counting Sort for floating-point numbers
 * Works by scaling floating-point numbers to integers
 * @param arr Vector of floating-point numbers
 * @param precision Number of decimal places to consider
 */
void counting_sort_float(std::vector<float>& arr, int precision = 2) {
    int n = arr.size();
    if (n == 0) return;

    // Find min and max values
    float min_val = arr[0];
    float max_val = arr[0];
    for (int i = 1; i < n; ++i) {
        min_val = std::min(min_val, arr[i]);
        max_val = std::max(max_val, arr[i]);
    }

    // Scale to integers
    int scale = static_cast<int>(pow(10, precision));
    std::vector<int> scaled_arr(n);
    for (int i = 0; i < n; ++i) {
        scaled_arr[i] = static_cast<int>((arr[i] - min_val) * scale);
    }

    // Find max scaled value
    int max_scaled = *std::max_element(scaled_arr.begin(), scaled_arr.end());

    // Counting sort on scaled values
    counting_sort_stable(scaled_arr, max_scaled);

    // Convert back to floats
    for (int i = 0; i < n; ++i) {
        arr[i] = (scaled_arr[i] / static_cast<float>(scale)) + min_val;
    }
}

/**
 * @brief Multi-key Counting Sort for pairs
 * Sorts pairs based on first element, then second element
 * @param arr Vector of pairs to be sorted
 */
void counting_sort_pairs(std::vector<std::pair<int, int>>& arr) {
    int n = arr.size();
    if (n == 0) return;

    // First, sort by second element
    int max_second = 0;
    for (const auto& p : arr) {
        max_second = std::max(max_second, p.second);
    }

    // Stable sort by second element
    std::vector<int> count_second(max_second + 1, 0);
    for (const auto& p : arr) {
        count_second[p.second]++;
    }
    for (int i = 1; i <= max_second; ++i) {
        count_second[i] += count_second[i - 1];
    }

    std::vector<std::pair<int, int>> temp(n);
    for (int i = n - 1; i >= 0; --i) {
        temp[count_second[arr[i].second] - 1] = arr[i];
        count_second[arr[i].second]--;
    }

    // Then, sort by first element
    int max_first = 0;
    for (const auto& p : temp) {
        max_first = std::max(max_first, p.first);
    }

    std::vector<int> count_first(max_first + 1, 0);
    for (const auto& p : temp) {
        count_first[p.first]++;
    }
    for (int i = 1; i <= max_first; ++i) {
        count_first[i] += count_first[i - 1];
    }

    for (int i = n - 1; i >= 0; --i) {
        arr[count_first[temp[i].first] - 1] = temp[i];
        count_first[temp[i].first]--;
    }
}

/**
 * @brief Test function to demonstrate counting sort variants
 */
void test_counting_sort() {
    std::vector<int> test_arrays[] = {
        {4, 2, 2, 8, 3, 3, 1, 9, 5, 6},
        {1, 4, 1, 2, 7, 5, 2},
        {0, 1, 0, 1, 0, 1, 0, 1},  // Binary
        {5, 5, 5, 5, 5},            // All same
        {1},                        // Single element
        {}                          // Empty array
    };

    std::string test_names[] = {
        "Random array",
        "Small array",
        "Binary array",
        "All same elements",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 6; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;

        // Test basic counting sort
        auto arr1 = test_arrays[i];
        if (!arr1.empty()) {
            print_array(arr1, "Original");
            int max_val = *std::max_element(arr1.begin(), arr1.end());
            counting_sort_basic(arr1, max_val);
            print_array(arr1, "Basic counting sort");
        }

        // Test stable counting sort
        auto arr2 = test_arrays[i];
        if (!arr2.empty()) {
            int max_val = *std::max_element(arr2.begin(), arr2.end());
            counting_sort_stable(arr2, max_val);
            std::cout << "Stable counting sort completed" << std::endl;
        }

        // Test auto-range counting sort
        auto arr3 = test_arrays[i];
        if (!arr3.empty()) {
            counting_sort_auto_range(arr3);
            std::cout << "Auto-range counting sort completed" << std::endl;
        }
    }
}

/**
 * @brief Test counting sort with negative numbers
 */
void test_negative_numbers() {
    std::cout << "\n=== Testing Negative Numbers ===" << std::endl;

    std::vector<int> arr = {-5, 3, -2, 8, -1, 0, 4, -3};
    print_array(arr, "Original with negatives");

    counting_sort_with_negatives(arr);
    print_array(arr, "Sorted with negatives");
}

/**
 * @brief Test counting sort with characters
 */
void test_character_sorting() {
    std::cout << "\n=== Testing Character Sorting ===" << std::endl;

    std::vector<char> chars = {'d', 'a', 'c', 'b', 'e', 'a', 'd'};
    std::cout << "Original characters: ";
    for (char c : chars) std::cout << c << " ";
    std::cout << std::endl;

    counting_sort_chars(chars);
    std::cout << "Sorted characters: ";
    for (char c : chars) std::cout << c << " ";
    std::cout << std::endl;
}

/**
 * @brief Performance comparison with other sorting algorithms
 */
void compare_sorting_performance() {
    // Generate test data with limited range
    std::vector<int> test_data(10000);
    for (int i = 0; i < 10000; ++i) {
        test_data[i] = rand() % 1000;  // Limited range for counting sort
    }

    std::cout << "\n=== Performance Comparison (10000 elements, range 0-999) ===" << std::endl;

    // Counting Sort
    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    counting_sort_auto_range(arr1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Quick Sort
    auto arr2 = test_data;
    start = std::chrono::high_resolution_clock::now();
    std::sort(arr2.begin(), arr2.end());
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Merge Sort
    auto arr3 = test_data;
    start = std::chrono::high_resolution_clock::now();
    merge_sort(arr3);
    end = std::chrono::high_resolution_clock::now();
    auto time3 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Counting Sort: " << time1.count() << " μs" << std::endl;
    std::cout << "Quick Sort: " << time2.count() << " μs" << std::endl;
    std::cout << "Merge Sort: " << time3.count() << " μs" << std::endl;
}