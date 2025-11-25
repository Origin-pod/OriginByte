#pragma once
#include <vector>

/**
 * @brief Linear Search Implementation
 *
 * Linear Search is the simplest search algorithm that sequentially checks each element
 * in a list until a match is found or the whole list has been searched.
 *
 * Time Complexity:
 * - Best Case: O(1) - when element is at the beginning
 * - Average Case: O(n)
 * - Worst Case: O(n) - when element is at the end or not present
 *
 * Space Complexity: O(1) - constant extra space
 *
 * Advantages:
 * - Simple to implement
 * - Works on unsorted data
 * - Works on any data structure that supports sequential access
 * - No preprocessing required
 *
 * Disadvantages:
 * - Inefficient for large datasets
 * - Much slower than binary search for sorted data
 *
 * Applications:
 * - Small datasets
 * - Unsorted data
 * - When only one or few searches are needed
 * - Linked lists (where binary search is not applicable)
 */

/**
 * @brief Basic Linear Search
 * Searches for target value in array and returns its index
 * @param arr Array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int linear_search_basic(const std::vector<int>& arr, int target) {
    for (size_t i = 0; i < arr.size(); ++i) {
        if (arr[i] == target) {
            return static_cast<int>(i);
        }
    }
    return -1;  // Target not found
}

/**
 * @brief Linear Search with count
 * Returns both index and number of comparisons made
 * @param arr Array to search in
 * @param target Value to search for
 * @param comparisons Output parameter for comparison count
 * @return Index of target if found, -1 otherwise
 */
int linear_search_with_count(const std::vector<int>& arr, int target, int& comparisons) {
    comparisons = 0;
    for (size_t i = 0; i < arr.size(); ++i) {
        comparisons++;
        if (arr[i] == target) {
            return static_cast<int>(i);
        }
    }
    return -1;
}

/**
 * @brief Find all occurrences of target
 * Returns indices of all occurrences of target value
 * @param arr Array to search in
 * @param target Value to search for
 * @return Vector containing all indices where target is found
 */
std::vector<int> linear_search_all(const std::vector<int>& arr, int target) {
    std::vector<int> indices;
    for (size_t i = 0; i < arr.size(); ++i) {
        if (arr[i] == target) {
            indices.push_back(static_cast<int>(i));
        }
    }
    return indices;
}

/**
 * @brief Linear Search from end (reverse search)
 * Searches from the end of the array towards the beginning
 * @param arr Array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int linear_search_reverse(const std::vector<int>& arr, int target) {
    for (int i = static_cast<int>(arr.size()) - 1; i >= 0; --i) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Linear Search with range limitation
 * Searches within specified range [start, end]
 * @param arr Array to search in
 * @param target Value to search for
 * @param start Starting index (inclusive)
 * @param end Ending index (inclusive)
 * @return Index of target if found, -1 otherwise
 */
int linear_search_range(const std::vector<int>& arr, int target, int start, int end) {
    if (start < 0) start = 0;
    if (end >= static_cast<int>(arr.size())) end = arr.size() - 1;

    for (int i = start; i <= end; ++i) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

/**
 * @brief Generic Linear Search for any comparable type
 * @tparam T Type that supports equality comparison
 * @param arr Array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
template <typename T>
int linear_search_generic(const std::vector<T>& arr, const T& target) {
    for (size_t i = 0; i < arr.size(); ++i) {
        if (arr[i] == target) {
            return static_cast<int>(i);
        }
    }
    return -1;
}

/**
 * @brief Linear Search with custom comparator
 * @tparam T Type of elements
 * @tparam Compare Comparator function type
 * @param arr Array to search in
 * @param target Value to search for
 * @param comp Comparison function
 * @return Index of target if found, -1 otherwise
 */
template <typename T, typename Compare>
int linear_search_custom(const std::vector<T>& arr, const T& target, Compare comp) {
    for (size_t i = 0; i < arr.size(); ++i) {
        if (comp(arr[i], target)) {
            return static_cast<int>(i);
        }
    }
    return -1;
}

/**
 * @brief Linear Search in linked list
 * @param head Head of linked list
 * @param target Value to search for
 * @return Pointer to node containing target, nullptr if not found
 */
struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* linear_search_linked_list(ListNode* head, int target) {
    ListNode* current = head;
    while (current != nullptr) {
        if (current->val == target) {
            return current;
        }
        current = current->next;
    }
    return nullptr;
}

/**
 * @brief Linear Search with sentinel value
 * Adds sentinel value to avoid boundary checks
 * @param arr Array to search in (will be modified)
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int linear_search_sentinel(std::vector<int>& arr, int target) {
    if (arr.empty()) return -1;

    // Store last element and place sentinel
    int last = arr.back();
    arr.back() = target;

    int i = 0;
    while (arr[i] != target) {
        i++;
    }

    // Restore last element
    arr.back() = last;

    // Check if we found actual target or reached sentinel
    if (i < static_cast<int>(arr.size()) - 1 || arr.back() == target) {
        return i;
    }

    return -1;
}

/**
 * @brief Find minimum element using linear search
 * @param arr Array to search in
 * @return Index of minimum element
 */
int find_minimum_linear(const std::vector<int>& arr) {
    if (arr.empty()) return -1;

    int min_idx = 0;
    for (size_t i = 1; i < arr.size(); ++i) {
        if (arr[i] < arr[min_idx]) {
            min_idx = static_cast<int>(i);
        }
    }
    return min_idx;
}

/**
 * @brief Find maximum element using linear search
 * @param arr Array to search in
 * @return Index of maximum element
 */
int find_maximum_linear(const std::vector<int>& arr) {
    if (arr.empty()) return -1;

    int max_idx = 0;
    for (size_t i = 1; i < arr.size(); ++i) {
        if (arr[i] > arr[max_idx]) {
            max_idx = static_cast<int>(i);
        }
    }
    return max_idx;
}

/**
 * @brief Find second minimum element using linear search
 * @param arr Array to search in
 * @return Index of second minimum element, -1 if not found
 */
int find_second_minimum_linear(const std::vector<int>& arr) {
    if (arr.size() < 2) return -1;

    int min_idx = 0;
    int second_min_idx = -1;

    for (size_t i = 1; i < arr.size(); ++i) {
        if (arr[i] < arr[min_idx]) {
            second_min_idx = min_idx;
            min_idx = static_cast<int>(i);
        } else if (second_min_idx == -1 ||
                  (arr[i] != arr[min_idx] && arr[i] < arr[second_min_idx])) {
            second_min_idx = static_cast<int>(i);
        }
    }

    return second_min_idx;
}

/**
 * @brief Count occurrences of target
 * @param arr Array to search in
 * @param target Value to count
 * @return Number of occurrences of target
 */
int count_occurrences(const std::vector<int>& arr, int target) {
    int count = 0;
    for (int val : arr) {
        if (val == target) {
            count++;
        }
    }
    return count;
}

/**
 * @brief Linear Search with early termination based on condition
 * Stops searching when condition is met
 * @param arr Array to search in
 * @param target Value to search for
 * @param max_steps Maximum number of steps to take
 * @return Index of target if found within max_steps, -1 otherwise
 */
int linear_search_limited(const std::vector<int>& arr, int target, int max_steps) {
    int steps = std::min(max_steps, static_cast<int>(arr.size()));
    for (int i = 0; i < steps; ++i) {
        if (arr[i] == target) {
            return i;
        }
    }
    return -1;
}

/**
 * @ bidirectional Linear Search
 * Searches from both ends simultaneously
 * @param arr Array to search in
 * @param target Value to search for
 * @return Index of target if found, -1 otherwise
 */
int linear_search_bidirectional(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = static_cast<int>(arr.size()) - 1;

    while (left <= right) {
        if (arr[left] == target) {
            return left;
        }
        if (arr[right] == target) {
            return right;
        }
        left++;
        right--;
    }

    return -1;
}

/**
 * @brief Test function to demonstrate linear search variants
 */
void test_linear_search() {
    std::vector<int> test_arrays[] = {
        {64, 34, 25, 12, 22, 11, 90},
        {5, 1, 4, 2, 8},
        {1, 2, 3, 4, 5},  // Sorted
        {5, 4, 3, 2, 1},  // Reverse sorted
        {3, 1, 4, 1, 5, 9, 2, 6, 5},  // With duplicates
        {1},              // Single element
        {}                // Empty array
    };

    std::string test_names[] = {
        "Random array",
        "Small array",
        "Sorted array",
        "Reverse sorted",
        "With duplicates",
        "Single element",
        "Empty array"
    };

    for (int i = 0; i < 7; ++i) {
        std::cout << "\n=== " << test_names[i] << " ===" << std::endl;
        print_array(test_arrays[i], "Array");

        // Test basic linear search
        int target = 25;
        if (i < 5) {  // Only test for arrays that might contain the target
            int index = linear_search_basic(test_arrays[i], target);
            std::cout << "Searching for " << target << ": ";
            if (index != -1) {
                std::cout << "Found at index " << index << std::endl;
            } else {
                std::cout << "Not found" << std::endl;
            }

            // Test with count
            int comparisons;
            index = linear_search_with_count(test_arrays[i], target, comparisons);
            std::cout << "Comparisons made: " << comparisons << std::endl;

            // Test reverse search
            index = linear_search_reverse(test_arrays[i], target);
            std::cout << "Reverse search result: " << index << std::endl;

            // Test bidirectional search
            index = linear_search_bidirectional(test_arrays[i], target);
            std::cout << "Bidirectional search result: " << index << std::endl;
        }

        // Test finding min/max
        if (!test_arrays[i].empty()) {
            int min_idx = find_minimum_linear(test_arrays[i]);
            int max_idx = find_maximum_linear(test_arrays[i]);
            std::cout << "Minimum: " << test_arrays[i][min_idx] << " at index " << min_idx << std::endl;
            std::cout << "Maximum: " << test_arrays[i][max_idx] << " at index " << max_idx << std::endl;
        }
    }
}

/**
 * @brief Test linear search with duplicates
 */
void test_search_with_duplicates() {
    std::cout << "\n=== Testing with Duplicates ===" << std::endl;
    std::vector<int> arr = {3, 1, 4, 1, 5, 9, 2, 6, 5, 3};
    print_array(arr, "Array with duplicates");

    int target = 1;
    std::vector<int> all_indices = linear_search_all(arr, target);
    std::cout << "All occurrences of " << target << ": ";
    for (int idx : all_indices) {
        std::cout << idx << " ";
    }
    std::cout << std::endl;

    int count = count_occurrences(arr, target);
    std::cout << "Total occurrences of " << target << ": " << count << std::endl;

    target = 3;
    all_indices = linear_search_all(arr, target);
    std::cout << "All occurrences of " << target << ": ";
    for (int idx : all_indices) {
        std::cout << idx << " ";
    }
    std::cout << std::endl;
}

/**
 * @brief Performance comparison with binary search
 */
void compare_search_performance() {
    // Generate sorted test data
    std::vector<int> sorted_data(10000);
    for (int i = 0; i < 10000; ++i) {
        sorted_data[i] = i * 2;  // Even numbers from 0 to 19998
    }

    int target = 12345;  // Will not be found
    int target_found = 8766;  // Will be found (index 4383)

    std::cout << "\n=== Search Performance Comparison (10000 elements) ===" << std::endl;

    // Linear Search - target not found
    auto start = std::chrono::high_resolution_clock::now();
    int result1 = linear_search_basic(sorted_data, target);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

    // Linear Search - target found
    start = std::chrono::high_resolution_clock::now();
    int result2 = linear_search_basic(sorted_data, target_found);
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

    // Binary Search - target not found
    start = std::chrono::high_resolution_clock::now();
    auto it1 = std::lower_bound(sorted_data.begin(), sorted_data.end(), target);
    end = std::chrono::high_resolution_clock::now();
    auto time3 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

    // Binary Search - target found
    start = std::chrono::high_resolution_clock::now();
    auto it2 = std::lower_bound(sorted_data.begin(), sorted_data.end(), target_found);
    end = std::chrono::high_resolution_clock::now();
    auto time4 = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

    std::cout << "Linear Search (not found): " << time1.count() << " ns" << std::endl;
    std::cout << "Linear Search (found): " << time2.count() << " ns" << std::endl;
    std::cout << "Binary Search (not found): " << time3.count() << " ns" << std::endl;
    std::cout << "Binary Search (found): " << time4.count() << " ns" << std::endl;
    std::cout << "Binary search is " << (time2.count() / time4.count()) << "x faster for found case" << std::endl;
}

/**
 * @brief Test sentinel optimization
 */
void test_sentinel_search() {
    std::cout << "\n=== Testing Sentinel Search ===" << std::endl;

    std::vector<int> arr = {64, 34, 25, 12, 22, 11, 90};
    print_array(arr, "Original array");

    int target = 22;

    // Regular linear search
    int comparisons1;
    int result1 = linear_search_with_count(arr, target, comparisons1);

    // Sentinel linear search (needs modifiable array)
    std::vector<int> arr_copy = arr;
    int result2 = linear_search_sentinel(arr_copy, target);

    std::cout << "Regular search: " << result1 << ", comparisons: " << comparisons1 << std::endl;
    std::cout << "Sentinel search: " << result2 << std::endl;
    std::cout << "Both methods found the same result: " << (result1 == result2 ? "Yes" : "No") << std::endl;
}