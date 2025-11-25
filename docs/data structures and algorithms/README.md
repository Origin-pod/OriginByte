# Data Structures and Algorithms

This comprehensive collection contains implementations of fundamental data structures and algorithms that every software engineer should know, particularly those with 3 years of experience or more.

## 📁 Organization

This repository is organized into the following categories:

### 1. [Fundamental Data Structures](./01-Fundamental%20Data%20Structures/)
Core data structures that form the foundation of computer science.

- **[Arrays](./01-Fundamental%20Data%20Structures/01-Arrays.cpp)** - Dynamic array implementation with automatic resizing
- **[Linked Lists](./01-Fundamental%20Data%20Structures/02-Linked%20Lists.cpp)** - Singly and doubly linked list implementations
- **[Stacks](./01-Fundamental%20Data%20Structures/03-Stacks.cpp)** - LIFO data structures with multiple implementations
- **[Queues](./01-Fundamental%20Data%20Structures/04-Queues.cpp)** - FIFO data structures including priority queues
- **[Hash Tables](./01-Fundamental%20Data%20Structures/05-Hash%20Tables.cpp)** - Hash map implementations with collision handling

### 2. [Sorting Algorithms](./02-Sorting%20Algorithms/)
Various sorting techniques with different time and space complexities.

- **[Bubble Sort](./02-Sorting%20Algorithms/01-Bubble%20Sort.cpp)** - Simple comparison-based sorting
- **[Selection Sort](./02-Sorting%20Algorithms/02-Selection%20Sort.cpp)** - In-place comparison sorting
- **[Insertion Sort](./02-Sorting%20Algorithms/03-Insertion%20Sort.cpp)** - Efficient for small/nearly sorted datasets
- **[Merge Sort](./02-Sorting%20Algorithms/04-Merge%20Sort.cpp)** - Divide-and-conquer stable sorting
- **[Quick Sort](./02-Sorting%20Algorithms/05-Quick%20Sort.cpp)** - Efficient in-place sorting
- **[Heap Sort](./02-Sorting%20Algorithms/06-Heap%20Sort.cpp)** - Comparison-based sorting using binary heap
- **[Counting Sort](./02-Sorting%20Algorithms/07-Counting%20Sort.cpp)** - Non-comparison sorting for integers

### 3. [Searching Algorithms](./03-Searching%20Algorithms/)
Techniques for finding elements in data structures.

- **[Linear Search](./03-Searching%20Algorithms/01-Linear%20Search.cpp)** - Sequential search through elements
- **[Binary Search](./03-Searching%20Algorithms/02-Binary%20Search.cpp)** - Efficient search in sorted arrays
- **[Jump Search](./03-Searching%20Algorithms/03-Jump%20Search.cpp)** - Jump ahead by fixed steps
- **[Interpolation Search](./03-Searching%20Algorithms/04-Interpolation%20Search.cpp)** - Position estimation for uniformly distributed data

## 🚀 Key Features

### Comprehensive Coverage
- **Time & Space Complexity Analysis** - Each algorithm includes detailed complexity analysis
- **Multiple Variants** - Different implementations with trade-offs
- **Performance Comparisons** - Benchmarks against other algorithms
- **Edge Cases** - Handling of empty arrays, single elements, duplicates
- **Stability Analysis** - Whether algorithms maintain relative order of equal elements

### Practical Implementation
- **Production-Ready Code** - Well-commented, error-handling included
- **Generic Templates** - Type-safe implementations for different data types
- **Memory Management** - Proper allocation/deallocation for custom data structures
- **Optimization Techniques** - Various optimizations and when to use them

### Educational Value
- **Step-by-Step Explanations** - Clear documentation of algorithm logic
- **Visual Aids** - Comments explaining the flow and decision points
- **Real-World Applications** - Practical use cases for each algorithm
- **Test Functions** - Comprehensive testing scenarios

## 📊 Algorithm Complexity Summary

### Sorting Algorithms

| Algorithm | Best | Average | Worst | Space | Stable |
|-----------|------|---------|-------|-------|---------|
| Bubble Sort | O(n) | O(n²) | O(n²) | O(1) | ✅ |
| Selection Sort | O(n²) | O(n²) | O(n²) | O(1) | ❌ |
| Insertion Sort | O(n) | O(n²) | O(n²) | O(1) | ✅ |
| Merge Sort | O(n log n) | O(n log n) | O(n log n) | O(n) | ✅ |
| Quick Sort | O(n log n) | O(n log n) | O(n²) | O(log n) | ❌ |
| Heap Sort | O(n log n) | O(n log n) | O(n log n) | O(1) | ❌ |
| Counting Sort | O(n + k) | O(n + k) | O(n + k) | O(k) | ✅ |

### Searching Algorithms

| Algorithm | Best | Average | Worst | Space | Prerequisites |
|-----------|------|---------|-------|-------|---------------|
| Linear Search | O(1) | O(n) | O(n) | O(1) | None |
| Binary Search | O(1) | O(log n) | O(log n) | O(1) | Sorted array |
| Jump Search | O(1) | O(√n) | O(√n) | O(1) | Sorted array |
| Interpolation Search | O(1) | O(log log n) | O(n) | O(1) | Uniform distribution |

### Data Structures

| Data Structure | Access | Search | Insertion | Deletion | Space |
|----------------|--------|--------|-----------|----------|-------|
| Array | O(1) | O(n) | O(n) | O(n) | O(n) |
| Linked List | O(n) | O(n) | O(1) | O(1) | O(n) |
| Stack | O(n) | O(n) | O(1) | O(1) | O(n) |
| Queue | O(n) | O(n) | O(1) | O(1) | O(n) |
| Hash Table | O(1) | O(1) | O(1) | O(1) | O(n) |

## 🛠️ How to Use

### Prerequisites
- C++11 or higher compiler
- Basic understanding of C++ templates and data structures

### Compilation
```bash
# Compile individual files
g++ -std=c++11 -o program filename.cpp

# Compile with optimizations
g++ -std=c++11 -O2 -o program filename.cpp

# Compile with debugging symbols
g++ -std=c++11 -g -o program filename.cpp
```

### Example Usage
```cpp
#include "01-Fundamental Data Structures/01-Arrays.cpp"
#include "02-Sorting Algorithms/01-Bubble Sort.cpp"

int main() {
    // Create and use dynamic array
    DynamicArray<int> arr;
    arr.push_back(64);
    arr.push_back(34);
    arr.push_back(25);

    // Sort using bubble sort
    std::vector<int> vec = {64, 34, 25, 12, 22, 11, 90};
    bubble_sort_basic(vec);

    return 0;
}
```

## 🧪 Testing

Each implementation includes comprehensive test functions that demonstrate:
- Basic functionality
- Edge cases
- Performance characteristics
- Comparison with other algorithms

Run the test functions to see the algorithms in action:

```cpp
// Test sorting algorithms
test_bubble_sort();
test_quick_sort();
compare_sorting_performance();

// Test searching algorithms
test_binary_search();
compare_search_performance();
```

## 🎯 Learning Path

### For Beginners
1. Start with **Linear Data Structures** (Arrays, Linked Lists, Stacks, Queues)
2. Learn **Linear Search** and **Binary Search**
3. Practice **Simple Sorting** (Bubble, Selection, Insertion)
4. Move to **Advanced Sorting** (Merge, Quick, Heap)

### For Intermediate Developers
1. Study **Hash Tables** and collision handling
2. Master all **Searching Algorithms**
3. Understand **Time/Space Complexity** trade-offs
4. Practice **Optimization Techniques**

### For Advanced Developers
1. Implement **Custom Data Structures**
2. Optimize for **Specific Use Cases**
3. Study **Memory Management**
4. Explore **Parallel Algorithms**

## 🔧 Advanced Topics Covered

### Algorithm Optimizations
- Median-of-three pivot selection in Quick Sort
- Adaptive block sizing in Jump Search
- Sentinel value optimization in Linear Search
- Hybrid sorting algorithms

### Edge Case Handling
- Empty and single-element arrays
- Arrays with all identical elements
- Negative numbers and floating-point values
- Large datasets and memory constraints

### Performance Analysis
- Comparison counting for efficiency analysis
- Timing benchmarks
- Memory usage profiling
- Scalability testing

## 📈 Real-World Applications

### Data Structures
- **Arrays**: Dynamic memory management, buffers
- **Linked Lists**: Navigation systems, undo/redo functionality
- **Stacks**: Function call stacks, expression evaluation
- **Queues**: Task scheduling, message processing
- **Hash Tables**: Database indexing, caching systems

### Sorting Algorithms
- **Quick Sort**: General-purpose sorting, in-memory sorting
- **Merge Sort**: External sorting, stable sorting requirements
- **Heap Sort**: Memory-constrained environments
- **Counting Sort**: Integer sorting with limited range

### Searching Algorithms
- **Binary Search**: Database queries, dictionary lookups
- **Interpolation Search**: Phone book searches, uniform data
- **Jump Search**: File system searches, large datasets

## 🤝 Contributing

This is a comprehensive learning resource. Feel free to:
- Add more algorithm variants
- Improve existing implementations
- Add more test cases
- Suggest optimizations
- Report issues

## 📚 References

- Introduction to Algorithms by Cormen, Leiserson, Rivest, Stein
- Algorithm Design Manual by Steven Skiena
- Cracking the Coding Interview by Gayle Laakmann McDowell
- GeeksforGeeks Algorithm Articles
- MIT OpenCourseWare - Introduction to Algorithms

---

**Note**: This collection is designed for educational purposes and to help developers understand the fundamental concepts of data structures and algorithms. Each implementation includes detailed comments explaining the logic, complexity analysis, and practical considerations.