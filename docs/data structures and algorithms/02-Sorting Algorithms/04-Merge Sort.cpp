#pragma once
#include <vector>
#include <algorithm>

/**
 * @brief Merge Sort Implementation
 *
 * Merge Sort is a divide-and-conquer algorithm that divides the array into two halves,
 * recursively sorts each half, and then merges the sorted halves.
 *
 * Time Complexity:
 * - Best Case: O(n log n)
 * - Average Case: O(n log n)
 * - Worst Case: O(n log n)
 *
 * Space Complexity: O(n) - requires additional space for merging
 *
 * Stable: Yes (maintains relative order of equal elements)
 *
 * Advantages:
 * - Guaranteed O(n log n) performance
 * - Stable sort
 * - Parallelizable (can sort halves independently)
 * - Works well with external sorting (large files)
 * - Predictable performance regardless of input distribution
 *
 * Disadvantages:
 * - Requires additional space O(n)
 * - Not in-place
 * - Slower constant factors compared to quicksort for in-memory sorting
 *
 * Applications:
 * - Large datasets where stability is required
 * - External sorting (sorting data that doesn't fit in memory)
 * - Parallel sorting algorithms
 * - Inversion counting
 * - Stable sorting requirements
 */

/**
 * @brief Merge two sorted subarrays into one sorted array
 * @param arr Main array
 * @param left Starting index of first subarray
 * @param mid Ending index of first subarray
 * @param right Ending index of second subarray
 * @return Number of comparisons made during merge
 */
int merge(std::vector<int>& arr, int left, int mid, int right) {
    int comparisons = 0;

    // Create temporary arrays
    int n1 = mid - left + 1;
    int n2 = right - mid;

    std::vector<int> left_arr(n1);
    std::vector<int> right_arr(n2);

    // Copy data to temporary arrays
    for (int i = 0; i < n1; ++i) {
        left_arr[i] = arr[left + i];
    }
    for (int j = 0; j < n2; ++j) {
        right_arr[j] = arr[mid + 1 + j];
    }

    // Merge the temporary arrays back into arr[left..right]
    int i = 0;  // Initial index of first subarray
    int j = 0;  // Initial index of second subarray
    int k = left;  // Initial index of merged subarray

    while (i < n1 && j < n2) {
        comparisons++;
        if (left_arr[i] <= right_arr[j]) {
            arr[k] = left_arr[i];
            i++;
        } else {
            arr[k] = right_arr[j];
            j++;
        }
        k++;
    }

    // Copy remaining elements of left_arr if any
    while (i < n1) {
        arr[k] = left_arr[i];
        i++;
        k++;
    }

    // Copy remaining elements of right_arr if any
    while (j < n2) {
        arr[k] = right_arr[j];
        j++;
        k++;
    }

    return comparisons;
}

/**
 * @brief Basic Merge Sort implementation
 * @param arr Vector to be sorted
 * @param left Starting index
 * @param right Ending index
 * @return Number of comparisons made
 */
int merge_sort_recursive(std::vector<int>& arr, int left, int right) {
    int comparisons = 0;

    if (left < right) {
        // Find the middle point to divide the array into two halves
        int mid = left + (right - left) / 2;

        // Sort first and second halves
        comparisons += merge_sort_recursive(arr, left, mid);
        comparisons += merge_sort_recursive(arr, mid + 1, right);

        // Merge the sorted halves
        comparisons += merge(arr, left, mid, right);
    }

    return comparisons;
}

/**
 * @brief Wrapper function for recursive merge sort
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int merge_sort(std::vector<int>& arr) {
    if (arr.empty()) return 0;
    return merge_sort_recursive(arr, 0, arr.size() - 1);
}

/**
 * @brief Bottom-up (Iterative) Merge Sort
 * Avoids recursion by iteratively merging subarrays of increasing size
 * @param arr Vector to be sorted
 * @return Number of comparisons made
 */
int merge_sort_iterative(std::vector<int>& arr) {
    int n = arr.size();
    int comparisons = 0;

    // Start with subarrays of size 1 and double the size each iteration
    for (int curr_size = 1; curr_size < n; curr_size *= 2) {
        // Pick starting point of different subarrays of current size
        for (int left_start = 0; left_start < n - 1; left_start += 2 * curr_size) {
            // Find ending point of left subarray
            int mid = std::min(left_start + curr_size - 1, n - 1);
            int right_end = std::min(left_start + 2 * curr_size - 1, n - 1);

            // Merge subarrays arr[left_start..mid] and arr[mid+1..right_end]
            comparisons += merge(arr, left_start, mid, right_end);
        }
    }

    return comparisons;
}

/**
 * @brief In-place Merge Sort (reduces space usage)
 * Attempts to merge without using additional space
 * Note: This is more complex and may have worse performance
 * @param arr Vector to be sorted
 */
void merge_sort_inplace(std::vector<int>& arr, int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;

        merge_sort_inplace(arr, left, mid);
        merge_sort_inplace(arr, mid + 1, right);

        // In-place merge
        int i = left;
        int j = mid + 1;

        while (i <= mid && j <= right) {
            if (arr[i] <= arr[j]) {
                i++;
            } else {
                int value = arr[j];
                int index = j;

                // Shift all elements between i and j one position right
                while (index > i) {
                    arr[index] = arr[index - 1];
                    index--;
                }

                arr[i] = value;
                i++;
                mid++;
                j++;
            }
        }
    }
}

/**
 * @brief Merge Sort with custom comparator
 * @tparam T Type that supports comparison
 * @tparam Compare Comparator function type
 * @param arr Vector to be sorted
 * @param comp Comparison function
 */
template <typename T, typename Compare>
void merge_sort_custom(std::vector<T>& arr, Compare comp) {
    if (arr.size() <= 1) return;

    std::vector<T> temp(arr.size());
    merge_sort_helper(arr, temp, 0, arr.size() - 1, comp);
}

template <typename T, typename Compare>
void merge_sort_helper(std::vector<T>& arr, std::vector<T>& temp, int left, int right, Compare comp) {
    if (left >= right) return;

    int mid = left + (right - left) / 2;
    merge_sort_helper(arr, temp, left, mid, comp);
    merge_sort_helper(arr, temp, mid + 1, right, comp);
    merge_custom(arr, temp, left, mid, right, comp);
}

template <typename T, typename Compare>
void merge_custom(std::vector<T>& arr, std::vector<T>& temp, int left, int mid, int right, Compare comp) {
    int i = left, j = mid + 1, k = left;

    while (i <= mid && j <= right) {
        if (comp(arr[i], arr[j])) {
            temp[k++] = arr[i++];
        } else {
            temp[k++] = arr[j++];
        }
    }

    while (i <= mid) temp[k++] = arr[i++];
    while (j <= right) temp[k++] = arr[j++];

    for (i = left; i <= right; ++i) {
        arr[i] = temp[i];
    }
}

/**
 * @brief Count inversions using merge sort
 * An inversion is a pair (i, j) such that i < j and arr[i] > arr[j]
 * @param arr Vector to analyze
 * @return Number of inversions
 */
long long count_inversions_merge_sort(std::vector<int>& arr, int left, int right) {
    long long inversions = 0;

    if (left < right) {
        int mid = left + (right - left) / 2;
        inversions += count_inversions_merge_sort(arr, left, mid);
        inversions += count_inversions_merge_sort(arr, mid + 1, right);
        inversions += merge_and_count(arr, left, mid, right);
    }

    return inversions;
}

long long merge_and_count(std::vector<int>& arr, int left, int mid, int right) {
    int n1 = mid - left + 1;
    int n2 = right - mid;

    std::vector<int> left_arr(n1);
    std::vector<int> right_arr(n2);

    for (int i = 0; i < n1; ++i) left_arr[i] = arr[left + i];
    for (int j = 0; j < n2; ++j) right_arr[j] = arr[mid + 1 + j];

    int i = 0, j = 0, k = left;
    long long inversions = 0;

    while (i < n1 && j < n2) {
        if (left_arr[i] <= right_arr[j]) {
            arr[k++] = left_arr[i++];
        } else {
            arr[k++] = right_arr[j++];
            inversions += (n1 - i);  // All remaining elements in left_arr are greater
        }
    }

    while (i < n1) arr[k++] = left_arr[i++];
    while (j < n2) arr[k++] = right_arr[j++];

    return inversions;
}

/**
 * @brief External Merge Sort for large files
 * Demonstrates the concept of sorting data that doesn't fit in memory
 * @param data Large dataset to be sorted
 * @param memory_limit Maximum size of data that can fit in memory
 */
void external_merge_sort_simulation(std::vector<int>& data, size_t memory_limit) {
    std::cout << "External Merge Sort Simulation" << std::endl;
    std::cout << "Total data size: " << data.size() << std::endl;
    std::cout << "Memory limit: " << memory_limit << std::endl;

    // Phase 1: Create sorted chunks
    std::vector<std::vector<int>> chunks;
    for (size_t i = 0; i < data.size(); i += memory_limit) {
        size_t end = std::min(i + memory_limit, data.size());
        std::vector<int> chunk(data.begin() + i, data.begin() + end);
        merge_sort(chunk);
        chunks.push_back(chunk);
    }

    std::cout << "Created " << chunks.size() << " sorted chunks" << std::endl;

    // Phase 2: Merge chunks using k-way merge
    std::vector<int> result;
    std::vector<std::pair<int, int>> heap;  // (value, chunk_index)

    // Initialize heap with first element from each chunk
    for (size_t i = 0; i < chunks.size(); ++i) {
        if (!chunks[i].empty()) {
            heap.push_back({chunks[i][0], i});
        }
    }

    // Create min-heap
    auto heap_comp = [](const auto& a, const auto& b) { return a.first > b.first; };
    std::make_heap(heap.begin(), heap.end(), heap_comp);

    std::vector<int> chunk_indices(chunks.size(), 0);

    // Merge using k-way merge
    while (!heap.empty()) {
        auto smallest = heap.front();
        std::pop_heap(heap.begin(), heap.end(), heap_comp);
        heap.pop_back();

        result.push_back(smallest.first);

        int chunk_idx = smallest.second;
        chunk_indices[chunk_idx]++;

        if (chunk_indices[chunk_idx] < chunks[chunk_idx].size()) {
            heap.push_back({chunks[chunk_idx][chunk_indices[chunk_idx]], chunk_idx});
            std::push_heap(heap.begin(), heap.end(), heap_comp);
        }
    }

    data = result;
    std::cout << "Final sorted data size: " << data.size() << std::endl;
}

/**
 * @brief Parallel Merge Sort (conceptual implementation)
 * Demonstrates how merge sort can be parallelized
 */
void parallel_merge_sort_concept(std::vector<int>& arr) {
    std::cout << "Parallel Merge Sort Concept:" << std::endl;
    std::cout << "1. Divide array into chunks" << std::endl;
    std::cout << "2. Sort each chunk on separate thread" << std::endl;
    std::cout << "3. Merge sorted chunks using parallel merge" << std::endl;
    std::cout << "4. This can significantly speed up sorting on multi-core systems" << std::endl;
}

/**
 * @brief Test function to demonstrate merge sort variants
 */
void test_merge_sort() {
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

        // Test recursive merge sort
        auto arr1 = test_arrays[i];
        print_array(arr1, "Original");
        int comps1 = merge_sort(arr1);
        print_array(arr1, "Recursive merge sort");
        std::cout << "Comparisons: " << comps1 << std::endl;

        // Test iterative merge sort
        auto arr2 = test_arrays[i];
        int comps2 = merge_sort_iterative(arr2);
        std::cout << "Iterative comparisons: " << comps2 << std::endl;

        // Test in-place merge sort
        auto arr3 = test_arrays[i];
        merge_sort_inplace(arr3, 0, arr3.size() - 1);
        std::cout << "In-place merge sort completed" << std::endl;

        // Count inversions
        if (!test_arrays[i].empty()) {
            auto arr4 = test_arrays[i];
            long long inversions = count_inversions_merge_sort(arr4, 0, arr4.size() - 1);
            std::cout << "Inversions: " << inversions << std::endl;
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

    // Merge Sort
    auto arr1 = test_data;
    auto start = std::chrono::high_resolution_clock::now();
    int comps1 = merge_sort(arr1);
    auto end = std::chrono::high_resolution_clock::now();
    auto time1 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    // Quick Sort (for comparison)
    auto arr2 = test_data;
    start = std::chrono::high_resolution_clock::now();
    std::sort(arr2.begin(), arr2.end());
    end = std::chrono::high_resolution_clock::now();
    auto time2 = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "Merge Sort: " << time1.count() << " μs, " << comps1 << " comparisons" << std::endl;
    std::cout << "std::sort (QuickSort): " << time2.count() << " μs" << std::endl;
}

/**
 * @brief Demonstrate external merge sort
 */
void test_external_merge_sort() {
    // Create large dataset
    std::vector<int> large_data(100);
    for (int i = 0; i < 100; ++i) {
        large_data[i] = rand() % 1000;
    }

    std::cout << "\nOriginal data (first 20): ";
    for (int i = 0; i < 20; ++i) {
        std::cout << large_data[i] << " ";
    }
    std::cout << std::endl;

    external_merge_sort_simulation(large_data, 20);

    std::cout << "Sorted data (first 20): ";
    for (int i = 0; i < 20; ++i) {
        std::cout << large_data[i] << " ";
    }
    std::cout << std::endl;
}