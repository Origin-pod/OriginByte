#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Heap Sort Implementation
 *
 * Heap Sort is a comparison-based sorting algorithm that uses a binary heap data structure.
 * It first builds a max heap from the input data, then repeatedly extracts the maximum
 * element and places it at the end of the array.
 *
 * Time Complexity:
 * - Best Case: O(n log n)
 * - Average Case: O(n log n)
 * - Worst Case: O(n log n)
 *
 * Space Complexity: O(1) - in-place sorting
 *
 * Stable: No (doesn't maintain relative order of equal elements)
 *
 * Advantages:
 * - Guaranteed O(n log n) performance
 * - In-place sorting (minimal extra space)
 * - No worst-case scenarios like quick sort
 * - Good cache performance
 *
 * Disadvantages:
 * - Not stable
 * - Slower in practice than well-implemented quick sort
 * - Not adaptive (doesn't benefit from partially sorted input)
 *
 * Applications:
 * - When guaranteed O(n log n) performance is required
 * - In-place sorting needs
 * - Priority queue operations
 * - Embedded systems with limited memory
 */

/**
 * @brief Heapify function to maintain max heap property
 * @param arr Vector to be heapified
 * @param n Size of heap
 * @param i Root index of subtree
 */
void heapify(std::vector<int>& arr, int n, int i) {
    int largest = i;        // Initialize largest as root
    int left = 2 * i + 1;   // Left child index
    int right = 2 * i + 2;  // Right child index

    // If left child exists and is greater than root
    if (left < n && arr[left] > arr[largest]) {
        largest = left;
    }

    // If right child exists and is greater than current largest
    if (right < n && arr[right] > arr[largest]) {
        largest = right;
    }

    // If largest is not root, swap and recursively heapify
    if (largest != i) {
        std::swap(arr[i], arr[largest]);
        heapify(arr, n, largest);
    }
}

/**
 * @brief Basic Heap Sort implementation
 * @param arr Vector to be sorted
 */
void heap_sort_basic(std::vector<int>& arr) {
    int n = arr.size();

    // Build max heap (rearrange array)
    for (int i = n / 2 - 1; i >= 0; --i) {
        heapify(arr, n, i);
    }

    // Extract elements from heap one by one
    for (int i = n - 1; i > 0; --i) {
        // Move current root to end
        std::swap(arr[0], arr[i]);

        // Call heapify on the reduced heap
        heapify(arr, i, 0);
    }
}

/**
 * @brief Heapify function for min heap
 * @param arr Vector to be heapified
 * @param n Size of heap
 * @param i Root index of subtree
 */
void heapify_min(std::vector<int>& arr, int n, int i) {
    int smallest = i;
    int left = 2 * i + 1;
    int right = 2 * i + 2;

    if (left < n && arr[left] < arr[smallest]) {
        smallest = left;
    }

    if (right < n && arr[right] < arr[smallest]) {
        smallest = right;
    }

    if (smallest != i) {
        std::swap(arr[i], arr[smallest]);
        heapify_min(arr, n, smallest);
    }
}

/**
 * @brief Heap Sort using min heap (sorts in descending order)
 * @param arr Vector to be sorted
 */
void heap_sort_min_heap(std::vector<int>& arr) {
    int n = arr.size();

    // Build min heap
    for (int i = n / 2 - 1; i >= 0; --i) {
        heapify_min(arr, n, i);
    }

    // Extract elements and place at beginning
    for (int i = n - 1; i > 0; --i) {
        std::swap(arr[0], arr[i]);
        heapify_min(arr, i, 0);
    }
}

/**
 * @brief Iterative heapify function
 * Avoids recursion for better performance
 * @param arr Vector to be heapified
 * @param n Size of heap
 * @param i Root index of subtree
 */
void heapify_iterative(std::vector<int>& arr, int n, int i) {
    while (true) {
        int largest = i;
        int left = 2 * i + 1;
        int right = 2 * i + 2;

        if (left < n && arr[left] > arr[largest]) {
            largest = left;
        }

        if (right < n && arr[right] > arr[largest]) {
            largest = right;
        }

        if (largest != i) {
            std::swap(arr[i], arr[largest]);
            i = largest;
        } else {
            break;
        }
    }
}

/**
 * @brief Heap Sort with iterative heapify
 * @param arr Vector to be sorted
 */
void heap_sort_iterative(std::vector<int>& arr) {
    int n = arr.size();

    // Build max heap iteratively
    for (int i = n / 2 - 1; i >= 0; --i) {
        heapify_iterative(arr, n, i);
    }

    // Extract elements
    for (int i = n - 1; i > 0; --i) {
        std::swap(arr[0], arr[i]);
        heapify_iterative(arr, i, 0);
    }
}

/**
 * @brief Build heap using bottom-up approach
 * @param arr Vector to be heapified
 */
void build_heap_bottom_up(std::vector<int>& arr) {
    int n = arr.size();

    // Start from last non-leaf node and heapify each node
    for (int i = n / 2 - 1; i >= 0; --i) {
        heapify(arr, n, i);
    }
}

/**
 * @brief Heap Sort with optimized heap building
 * @param arr Vector to be sorted
 */
void heap_sort_optimized(std::vector<int>& arr) {
    int n = arr.size();

    // Build max heap using bottom-up approach
    build_heap_bottom_up(arr);

    // Extract elements
    for (int i = n - 1; i > 0; --i) {
        std::swap(arr[0], arr[i]);
        heapify(arr, i, 0);
    }
}

/**
 * @brief Generic Heap Sort for any comparable type
 * @tparam T Type that supports comparison operators
 * @param arr Vector to be sorted
 * @param ascending Sort order (true for ascending, false for descending)
 */
template <typename T>
void heap_sort_generic(std::vector<T>& arr, bool ascending = true) {
    int n = arr.size();

    // Build heap (max heap for ascending, min heap for descending)
    for (int i = n / 2 - 1; i >= 0; --i) {
        if (ascending) {
            // Build max heap
            int largest = i;
            while (true) {
                int left = 2 * largest + 1;
                int right = 2 * largest + 2;
                int max_idx = largest;

                if (left < n && arr[left] > arr[max_idx]) {
                    max_idx = left;
                }
                if (right < n && arr[right] > arr[max_idx]) {
                    max_idx = right;
                }

                if (max_idx != largest) {
                    std::swap(arr[largest], arr[max_idx]);
                    largest = max_idx;
                } else {
                    break;
                }
            }
        }
    }

    // Extract elements
    for (int i = n - 1; i > 0; --i) {
        std::swap(arr[0], arr[i]);

        // Heapify reduced heap
        int root = 0;
        while (true) {
            int left = 2 * root + 1;
            int right = 2 * root + 2;
            int extreme_idx = root;

            if (ascending) {
                // Max heap for ascending order
                if (left < i && arr[left] > arr[extreme_idx]) {
                    extreme_idx = left;
                }
                if (right < i && arr[right] > arr[extreme_idx]) {
                    extreme_idx = right;
                }
            } else {
                // Min heap for descending order
                if (left < i && arr[left] < arr[extreme_idx]) {
                    extreme_idx = left;
                }
                if (right < i && arr[right] < arr[extreme_idx]) {
                    extreme_idx = right;
                }
            }

            if (extreme_idx != root) {
                std::swap(arr[root], arr[extreme_idx]);
                root = extreme_idx;
            } else {
                break;
            }
        }
    }
}

/**
 * @brief Find kth largest element using heap sort approach
 * @param arr Vector to search in
 * @param k Position (1-based) from largest
 * @return kth largest element
 */
int find_kth_largest_heap(std::vector<int> arr, int k) {
    if (k < 1 || k > arr.size()) {
        throw std::out_of_range("k is out of bounds");
    }

    int n = arr.size();

    // Build max heap
    for (int i = n / 2 - 1; i >= 0; --i) {
        heapify(arr, n, i);
    }

    // Extract k-1 largest elements
    for (int i = n - 1; i > n - k; --i) {
        std::swap(arr[0], arr[i]);
        heapify(arr, i, 0);
    }

    // Return kth largest (now at root)
    return arr[0];
}

/**
 * @brief Heap sort for range [start, end] in array
 * @param arr Vector to be partially sorted
 * @param start Starting index
 * @param end Ending index
 */
void heap_sort_range(std::vector<int>& arr, int start, int end) {
    if (start >= end || start < 0 || end >= arr.size()) {
        return;
    }

    int range_size = end - start + 1;

    // Create temporary array for the range
    std::vector<int> temp(arr.begin() + start, arr.begin() + end + 1);

    // Sort the temporary array
    heap_sort_basic(temp);

    // Copy back to original array
    for (int i = 0; i < range_size; ++i) {
        arr[start + i] = temp[i];
    }
}

/**
 * @brief Test if array represents a valid max heap
 * @param arr Array to check
 * @return true if valid max heap
 */
bool is_max_heap(const std::vector<int>& arr) {
    int n = arr.size();

    for (int i = 0; i < n; ++i) {
        int left = 2 * i + 1;
        int right = 2 * i + 2;

        if (left < n && arr[i] < arr[left]) {
            return false;
        }
        if (right < n && arr[i] < arr[right]) {
            return false;
        }
    }

    return true;
}

/**
 * @brief Test function to demonstrate heap sort variants
 */
void test_heap_sort() {
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

        // Test basic heap sort
        auto arr1 = test_arrays[i];
        if (!arr1.empty()) {
            print_array(arr1, "Original");
            heap_sort_basic(arr1);
            print_array(arr1, "Heap sort");
        }

        // Test min heap sort (descending)
        auto arr2 = test_arrays[i];
        if (!arr2.empty()) {
            heap_sort_min_heap(arr2);
            print_array(arr2, "Min heap sort (descending)");
        }

        // Test iterative heap sort
        auto arr3 = test_arrays[i];
        if (!arr3.empty()) {
            heap_sort_iterative(arr3);
            std::cout << "Iterative heap sort completed" << std::endl;
        }
    }
}

/**
 * @brief Performance comparison with other sorting algorithms
 */
void compare_sorting_performance() {
    // Generate test data
    std::vector<int> test_data(10000);
    for (int i = 0; i < 10000; ++i) {
        test_data[i] = rand() % 10000;
    }

    std::cout << "\n=== Performance Comparison (10000 elements) ===" << std::endl;

    // Heap Sort
    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    heap_sort_basic(arr1);
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

    std::cout << "Heap Sort: " << time1.count() << " μs" << std::endl;
    std::cout << "Quick Sort: " << time2.count() << " μs" << std::endl;
    std::cout << "Merge Sort: " << time3.count() << " μs" << std::endl;
}

/**
 * @brief Demonstrate heap operations
 */
void demonstrate_heap_operations() {
    std::cout << "\n=== Heap Operations Demonstration ===" << std::endl;

    std::vector<int> arr = {3, 1, 6, 5, 2, 4};
    print_array(arr, "Original array");

    // Build max heap
    build_heap_bottom_up(arr);
    print_array(arr, "After building max heap");

    // Check if it's a valid max heap
    std::cout << "Is valid max heap: " << (is_max_heap(arr) ? "Yes" : "No") << std::endl;

    // Extract maximum element
    int max_element = arr[0];
    std::swap(arr[0], arr[arr.size() - 1]);
    arr.pop_back();
    heapify(arr, arr.size(), 0);

    std::cout << "Extracted maximum: " << max_element << std::endl;
    print_array(arr, "Heap after extraction");

    // Find kth largest elements
    std::vector<int> original = {3, 1, 6, 5, 2, 4, 8, 7};
    print_array(original, "Original array for kth largest test");

    for (int k = 1; k <= std::min(3, (int)original.size()); ++k) {
        int kth_largest = find_kth_largest_heap(original, k);
        std::cout << k << "th largest element: " << kth_largest << std::endl;
    }
}