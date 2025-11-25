#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Insertion Sort Implementation
 *
 * Selection sort and insertion sort are both simple sorting algorithms, but they operate
 * in different ways.
 *
 * Selection sort repeatedly selects the smallest element from the unsorted portion of the
 * array and moves it to the front of the sorted portion. It does this in a linear time
 * complexity, making it efficient for small datasets, but with a poor time complexity of
 * O(n²) in the worst case.
 *
 * Insertion sort builds the final sorted array one item at a time. It iterates through
 * input elements and inserts each element into its correct position in the sorted portion.
 * It does this in a linear time complexity, making it efficient for small datasets, but
 * with the same poor time complexity of O(n²) in the worst case.
 *
 * The main difference is that selection sort selects and moves the smallest element, while
 * insertion sort inserts each element into its correct position in the sorted portion.
 * This makes insertion sort more efficient for partially sorted data, as it only needs to
 * shift elements forward to make space for the new element.
 *
 * In summary, selection sort repeatedly selects and moves the smallest element, while
 * insertion sort builds the final sorted array one item at a time.
 * Insertion Sort builds the final sorted array one item at a time. It iterates through
 * input elements and inserts each element into its correct position in the sorted portion.
 *
 * Time Complexity:
 * - Best Case: O(n) - when array is already sorted
 * - Average Case: O(n²)
 * - Worst Case: O(n²) - when array is reverse sorted
 *
 * Space Complexity: O(1) - in-place sorting
 *
 * Stable: Yes (maintains relative order of equal elements)
 *
 * Advantages:
 * - Simple to implement
 * - Efficient for small datasets
 * - Adaptive (performs well on partially sorted data)
 * - Stable sort
 * - Online algorithm (can sort as it receives data)
 * - In-place sorting
 *
 * Disadvantages:
 * - Inefficient for large datasets
 * - Quadratic time complexity in worst/average cases
 *
 * Applications:
 * - Small datasets
 * - Nearly sorted data
 * - When data is being received in streams
 * - As part of more complex algorithms (like quicksort's small subarrays)
 * - Educational purposes
 */

/**
 * @brief Basic Insertion Sort implementation
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int insertion_sort_basic(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;

    for (int i = 1; i < n; ++i) {
        int key = arr[i];
        int j = i - 1;

        // Move elements greater than key one position ahead
        while (j >= 0 && arr[j] > key) {
            comparisons++;
            arr[j + 1] = arr[j];
            j--;
        }
        if (j >= 0) comparisons++;  // Count the final comparison that fails

        // Insert key at correct position
        arr[j + 1] = key;
    }

    return comparisons;
}

/**
 * @brief Insertion Sort with binary search for insertion point
 * Reduces the number of comparisons by using binary search
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int insertion_sort_binary(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;

    for (int i = 1; i < n; ++i) {
        int key = arr[i];
        int left = 0;
        int right = i - 1;

        // Binary search to find insertion point
        while (left <= right) {
            int mid = left + (right - left) / 2;
            comparisons++;
            if (arr[mid] > key) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // Shift elements to make space for key
        for (int j = i - 1; j >= left; --j) {
            arr[j + 1] = arr[j];
        }

        // Insert key at correct position
        arr[left] = key;
    }

    return comparisons;
}

/**
 * @brief Recursive Insertion Sort
 * Demonstrates recursive approach to insertion sort
 * @param arr Vector to be sorted
 * @param n Size of array portion to sort
 */
void insertion_sort_recursive(std::vector<int>& arr, int n) {
    // Base case: if array has 0 or 1 element, it's already sorted
    if (n <= 1) {
        return;
    }

    // Sort first n-1 elements
    insertion_sort_recursive(arr, n - 1);

    // Insert last element at correct position in sorted array
    int last = arr[n - 1];
    int j = n - 2;

    // Move elements greater than last one position ahead
    while (j >= 0 && arr[j] > last) {
        arr[j + 1] = arr[j];
        j--;
    }

    arr[j + 1] = last;
}

/**
 * @brief Generic Insertion Sort for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Vector to be sorted
 * @param ascending Sort order (true for ascending, false for descending)
 */
template <typename T>
void insertion_sort_generic(std::vector<T>& arr, bool ascending = true) {
    int n = arr.size();

    for (int i = 1; i < n; ++i) {
        T key = arr[i];
        int j = i - 1;

        // Move elements based on sort order
        if (ascending) {
            while (j >= 0 && arr[j] > key) {
                arr[j + 1] = arr[j];
                j--;
            }
        } else {
            while (j >= 0 && arr[j] < key) {
                arr[j + 1] = arr[j];
                j--;
            }
        }

        arr[j + 1] = key;
    }
}

/**
 * @brief Shell Sort (Improved Insertion Sort)
 * Generalization of insertion sort that allows exchange of items far apart
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int shell_sort(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;

    // Start with a large gap, then reduce the gap
    for (int gap = n / 2; gap > 0; gap /= 2) {
        // Perform a gapped insertion sort for this gap size
        for (int i = gap; i < n; ++i) {
            int temp = arr[i];
            int j;

            // Shift earlier gap-sorted elements up until the correct location for arr[i] is found
            for (j = i; j >= gap; j -= gap) {
                comparisons++;
                if (arr[j - gap] > temp) {
                    arr[j] = arr[j - gap];
                } else {
                    break;
                }
            }

            // Put temp (the original arr[i]) in its correct location
            arr[j] = temp;
        }
    }

    return comparisons;
}

/**
 * @brief Insertion Sort with sentinel
 * Uses a sentinel value to eliminate boundary checks
 * @param arr Vector to be sorted (must have at least one element)
 * @return Number of comparisons made
 */
int insertion_sort_sentinel(std::vector<int>& arr) {
    int n = arr.size();
    if (n <= 1) return 0;

    int comparisons = 0;

    // Find minimum element and place it at beginning
    int min_idx = 0;
    for (int i = 1; i < n; ++i) {
        if (arr[i] < arr[min_idx]) {
            min_idx = i;
        }
    }
    std::swap(arr[0], arr[min_idx]);

    // Now arr[0] is the minimum, so we can use it as sentinel
    for (int i = 1; i < n; ++i) {
        int key = arr[i];
        int j = i - 1;

        // No need to check j >= 0 because arr[0] is minimum
        while (arr[j] > key) {
            comparisons++;
            arr[j + 1] = arr[j];
            j--;
        }
        comparisons++;  // Count the final comparison that fails

        arr[j + 1] = key;
    }

    return comparisons;
}

/**
 * @brief Insertion Sort for linked list
 * Demonstrates insertion sort on a different data structure
 */
struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode* insertion_sort_linked_list(ListNode* head) {
    if (!head || !head->next) {
        return head;
    }

    ListNode dummy(0);  // Dummy node to serve as starting point
    ListNode* current = head;

    while (current) {
        ListNode* next = current->next;
        ListNode* prev = &dummy;

        // Find position to insert current node
        while (prev->next && prev->next->val < current->val) {
            prev = prev->next;
        }

        // Insert current node
        current->next = prev->next;
        prev->next = current;

        current = next;
    }

    return dummy.next;
}

/**
 * @brief Binary Insertion Sort with shift optimization
 * Uses binary search and block shifting for better cache performance
 * @param arr Vector to be sorted
 */
void insertion_sort_optimized(std::vector<int>& arr) {
    int n = arr.size();

    for (int i = 1; i < n; ++i) {
        int key = arr[i];

        // Binary search for insertion point
        int left = 0, right = i;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (arr[mid] <= key) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // Use std::copy_backward for efficient shifting
        if (left < i) {
            std::copy_backward(arr.begin() + left, arr.begin() + i, arr.begin() + i + 1);
            arr[left] = key;
        }
    }
}

/**
 * @brief Count inversions in array using modified insertion sort
 * An inversion is a pair (i, j) such that i < j and arr[i] > arr[j]
 * @param arr Vector to analyze
 * @return Number of inversions
 */
int count_inversions(std::vector<int> arr) {
    int n = arr.size();
    int inversions = 0;

    for (int i = 1; i < n; ++i) {
        int key = arr[i];
        int j = i - 1;

        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
            inversions++;  // Each shift represents an inversion
        }

        arr[j + 1] = key;
    }

    return inversions;
}

/**
 * @brief Test function to demonstrate insertion sort variants
 */
void test_insertion_sort() {
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

        // Test basic insertion sort
        auto arr1 = test_arrays[i];
        print_array(arr1, "Original");
        int comps1 = insertion_sort_basic(arr1);
        print_array(arr1, "Basic insertion sort");
        std::cout << "Comparisons: " << comps1 << std::endl;

        // Test binary insertion sort
        auto arr2 = test_arrays[i];
        int comps2 = insertion_sort_binary(arr2);
        std::cout << "Binary insertion comparisons: " << comps2 << std::endl;

        // Test shell sort
        auto arr3 = test_arrays[i];
        int comps3 = shell_sort(arr3);
        std::cout << "Shell sort comparisons: " << comps3 << std::endl;

        // Count inversions
        if (!test_arrays[i].empty()) {
            int inversions = count_inversions(test_arrays[i]);
            std::cout << "Inversions: " << inversions << std::endl;
        }
    }
}

/**
 * @brief Performance comparison of insertion sort variants
 */
void compare_insertion_sort_variants() {
    // Generate different types of test data
    std::vector<std::vector<int>> test_cases;

    // Random data
    std::vector<int> random_data(1000);
    for (int i = 0; i < 1000; ++i) random_data[i] = rand() % 1000;
    test_cases.push_back(random_data);

    // Nearly sorted data
    std::vector<int> nearly_sorted(1000);
    for (int i = 0; i < 1000; ++i) nearly_sorted[i] = i;
    // Add some random swaps
    for (int i = 0; i < 50; ++i) {
        int idx1 = rand() % 1000;
        int idx2 = rand() % 1000;
        std::swap(nearly_sorted[idx1], nearly_sorted[idx2]);
    }
    test_cases.push_back(nearly_sorted);

    std::string case_names[] = {"Random data", "Nearly sorted data"};

    for (size_t case_idx = 0; case_idx < test_cases.size(); ++case_idx) {
        std::cout << "\n=== " << case_names[case_idx] << " (1000 elements) ===" << std::endl;

        // Basic insertion sort
        auto arr1 = test_cases[case_idx];
        auto start = std::chrono::high_resolution_clock::now();
        int comps1 = insertion_sort_basic(arr1);
        auto end = std::chrono::high_resolution_clock::now();
        auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        // Binary insertion sort
        auto arr2 = test_cases[case_idx];
        start = std::chrono::high_resolution_clock::now();
        int comps2 = insertion_sort_binary(arr2);
        end = std::chrono::high_resolution_clock::now();
        auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        // Shell sort
        auto arr3 = test_cases[case_idx];
        start = std::chrono::high_resolution_clock::now();
        int comps3 = shell_sort(arr3);
        end = std::chrono::high_resolution_clock::now();
        auto time3 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

        std::cout << "Basic Insertion: " << time1.count() << " μs, " << comps1 << " comparisons" << std::endl;
        std::cout << "Binary Insertion: " << time2.count() << " μs, " << comps2 << " comparisons" << std::endl;
        std::cout << "Shell Sort: " << time3.count() << " μs, " << comps3 << " comparisons" << std::endl;
    }
}

/**
 * @brief Demonstrate online nature of insertion sort
 * Simulates receiving data and maintaining sorted order
 */
void demonstrate_online_sorting() {
    std::cout << "\n=== Online Sorting Demonstration ===" << std::endl;
    std::vector<int> stream_data = {5, 2, 8, 1, 9, 3, 7, 4, 6};
    std::vector<int> sorted_stream;

    std::cout << "Processing stream: ";
    for (int num : stream_data) {
        std::cout << num << " ";
        sorted_stream.push_back(num);

        // Insert new element in sorted position
        int i = sorted_stream.size() - 1;
        int key = sorted_stream[i];
        while (i > 0 && sorted_stream[i - 1] > key) {
            sorted_stream[i] = sorted_stream[i - 1];
            i--;
        }
        sorted_stream[i] = key;

        std::cout << "-> Sorted: [";
        for (size_t j = 0; j < sorted_stream.size(); ++j) {
            std::cout << sorted_stream[j];
            if (j < sorted_stream.size() - 1) std::cout << ", ";
        }
        std::cout << "]" << std::endl;
    }
}