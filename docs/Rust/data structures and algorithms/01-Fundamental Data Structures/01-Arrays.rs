/**
 * @file 01-Arrays.rs
 * @brief Comprehensive Array Data Structures Implementation in Rust
 *
 * This file covers various array-based data structures and their operations,
 * including static arrays, dynamic vectors, multi-dimensional arrays, and
 * specialized array implementations for different use cases.
 */

use std::fmt;

// ================================================================================
// 1. BASIC ARRAY OPERATIONS
// ================================================================================

fn demonstrate_basic_arrays() {
    println!("=== BASIC ARRAY OPERATIONS ===");

    // Fixed-size arrays (stack allocated)
    let fixed_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Fixed array: {:?}", fixed_array);
    println!("First element: {}", fixed_array[0]);
    println!("Array length: {}", fixed_array.len());

    // Array initialization with default values
    let default_array: [i32; 10] = [0; 10];
    println!("Default array (first 3): {:?}", &default_array[..3]);

    // Array slicing
    let slice = &fixed_array[1..4]; // Elements 1, 2, 3
    println!("Slice: {:?}", slice);

    // Array iteration
    println!("Iterating over array:");
    for (i, &value) in fixed_array.iter().enumerate() {
        println!("  [{}] = {}", i, value);
    }

    // Array methods
    println!("Sum: {}", fixed_array.iter().sum::<i32>());
    println!("Max: {:?}", fixed_array.iter().max());
    println!("Contains 3: {}", fixed_array.contains(&3));

    println!();
}

// ================================================================================
// 2. DYNAMIC ARRAY (VECTOR) OPERATIONS
// ================================================================================

fn demonstrate_vector_operations() {
    println!("=== DYNAMIC ARRAY (VECTOR) OPERATIONS ===");

    // Creating vectors
    let mut vec1 = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5];
    let vec3 = vec![0; 10]; // Vector with 10 zeros

    println!("Vec1 (empty): {:?}", vec1);
    println!("Vec2 (initialized): {:?}", vec2);
    println!("Vec3 (zeros, first 3): {:?}", &vec3[..3]);

    // Adding elements
    vec1.push(10);
    vec1.push(20);
    vec1.push(30);
    println!("Vec1 after pushing: {:?}", vec1);

    // Removing elements
    let last_element = vec2.pop();
    println!("Popped from vec2: {:?}", last_element);
    println!("Vec2 after pop: {:?}", vec2);

    // Inserting at specific position
    vec2.insert(2, 99);
    println!("Vec2 after inserting 99 at index 2: {:?}", vec2);

    // Removing at specific position
    let removed = vec2.remove(2);
    println!("Removed from vec2: {}", removed);
    println!("Vec2 after removal: {:?}", vec2);

    // Accessing elements
    println!("Element at index 1: {}", vec2[1]);
    println!("Element at index 1 (safe): {:?}", vec2.get(1));

    // Vector capacity and length
    println!("Vec2 length: {}", vec2.len());
    println!("Vec2 capacity: {}", vec2.capacity());

    // Resizing vectors
    vec2.resize(10, 0);
    println!("Vec2 after resize: {:?}", vec2);

    println!();
}

// ================================================================================
// 3. MULTI-DIMENSIONAL ARRAYS
// ================================================================================

fn demonstrate_multidimensional_arrays() {
    println!("=== MULTI-DIMENSIONAL ARRAYS ===");

    // 2D array using nested arrays
    let matrix_2d: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("2D Matrix:");
    for row in &matrix_2d {
        println!("  {:?}", row);
    }

    // 2D array using Vec<Vec<T>>
    let mut dynamic_matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    println!("\nDynamic Matrix:");
    for row in &dynamic_matrix {
        println!("  {:?}", row);
    }

    // Accessing elements
    println!("Element at [1][2]: {}", matrix_2d[1][2]);

    // Modifying dynamic matrix
    dynamic_matrix[1][2] = 99;
    println!("Modified matrix:");
    for row in &dynamic_matrix {
        println!("  {:?}", row);
    }

    // Adding row to dynamic matrix
    dynamic_matrix.push(vec![10, 11, 12]);
    println!("Matrix after adding row:");
    for row in &dynamic_matrix {
        println!("  {:?}", row);
    }

    println!();
}

// ================================================================================
// 4. CIRCULAR BUFFER (RING BUFFER)
// ================================================================================

struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn push(&mut self, item: T) -> bool {
        if self.size == self.capacity {
            return false; // Buffer is full
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None; // Buffer is empty
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        item
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn len(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: fmt::Debug> fmt::Debug for CircularBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("[");
        for i in 0..self.size {
            let index = (self.head + i) % self.capacity;
            if let Some(ref item) = self.buffer[index] {
                result.push_str(&format!("{:?}", item));
                if i < self.size - 1 {
                    result.push_str(", ");
                }
            }
        }
        result.push(']');
        write!(f, "{}", result)
    }
}

fn demonstrate_circular_buffer() {
    println!("=== CIRCULAR BUFFER (RING BUFFER) ===");

    let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

    println!("Empty buffer: {:?}", buffer);
    println!("Is empty: {}", buffer.is_empty());
    println!("Is full: {}", buffer.is_full());

    // Add elements
    for i in 1..=5 {
        let pushed = buffer.push(i);
        println!("Pushed {}: {} (buffer: {:?})", i, pushed, buffer);
    }

    println!("Buffer full: {}", buffer.is_full());

    // Try to add to full buffer
    let extra_push = buffer.push(6);
    println!("Push extra element: {}", extra_push);

    // Remove elements
    while let Some(item) = buffer.pop() {
        println!("Popped: {} (buffer: {:?})", item, buffer);
    }

    println!("Buffer empty again: {}", buffer.is_empty());

    println!();
}

// ================================================================================
// 5. SLICE WINDOW OPERATIONS
// ================================================================================

fn demonstrate_slice_windows() {
    println!("=== SLICE WINDOW OPERATIONS ===");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Windows of size 3
    println!("Windows of size 3:");
    for window in data.windows(3) {
        println!("  {:?} -> sum: {}", window, window.iter().sum::<i32>());
    }

    // Chunks of size 3
    println!("\nChunks of size 3:");
    for chunk in data.chunks(3) {
        println!("  {:?}", chunk);
    }

    // Sliding window with custom size
    fn sliding_sum(data: &[i32], window_size: usize) -> Vec<i32> {
        let mut result = Vec::new();
        for window in data.windows(window_size) {
            result.push(window.iter().sum());
        }
        result
    }

    let sums = sliding_sum(&data, 4);
    println!("\nSliding sums of size 4: {:?}", sums);

    println!();
}

// ================================================================================
// 6. MATRIX OPERATIONS
// ================================================================================

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    fn from_data(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    fn row(&self, index: usize) -> &[f64] {
        &self.data[index]
    }

    fn col(&self, index: usize) -> Vec<f64> {
        self.data.iter().map(|row| row[index]).collect()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:8.2}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn demonstrate_matrix_operations() {
    println!("=== MATRIX OPERATIONS ===");

    let mut matrix_a = Matrix::from_data(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
    ]);

    let matrix_b = Matrix::from_data(vec![
        vec![7.0, 8.0],
        vec![9.0, 10.0],
        vec![11.0, 12.0],
    ]);

    println!("Matrix A:");
    println!("{}", matrix_a);

    println!("Matrix B:");
    println!("{}", matrix_b);

    // Matrix addition (need same dimensions)
    let matrix_c = Matrix::from_data(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
    ]);
    let sum = matrix_a.add(&matrix_c);
    println!("A + C:");
    println!("{}", sum);

    // Matrix multiplication
    let product = matrix_a.multiply(&matrix_b);
    println!("A × B:");
    println!("{}", product);

    // Matrix transpose
    let transpose = matrix_a.transpose();
    println!("Transpose of A:");
    println!("{}", transpose);

    // Access rows and columns
    println!("Row 1 of A: {:?}", matrix_a.row(1));
    println!("Column 2 of A: {:?}", matrix_a.col(1));

    println!();
}

// ================================================================================
// 7. ADVANCED ARRAY ALGORITHMS
// ================================================================================

fn demonstrate_advanced_algorithms() {
    println!("=== ADVANCED ARRAY ALGORITHMS ===");

    // Prefix sums
    let data = vec![1, 2, 3, 4, 5];
    let prefix_sums: Vec<i32> = data.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("Original: {:?}", data);
    println!("Prefix sums: {:?}", prefix_sums);

    // Difference array
    fn difference_array(arr: &[i32]) -> Vec<i32> {
        if arr.is_empty() {
            return Vec::new();
        }

        let mut diff = vec![0; arr.len()];
        diff[0] = arr[0];
        for i in 1..arr.len() {
            diff[i] = arr[i] - arr[i - 1];
        }
        diff
    }

    let diff = difference_array(&data);
    println!("Difference array: {:?}", diff);

    // Range update using difference array
    fn range_update(arr: &mut [i32], start: usize, end: usize, value: i32) {
        if start < arr.len() {
            arr[start] += value;
        }
        if end + 1 < arr.len() {
            arr[end + 1] -= value;
        }
    }

    fn apply_difference(diff: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(diff.len());
        let mut sum = 0;
        for &val in diff {
            sum += val;
            result.push(sum);
        }
        result
    }

    let mut diff = vec![0; 8];
    range_update(&mut diff, 1, 4, 5);
    range_update(&mut diff, 3, 6, 3);
    let result = apply_difference(&diff);
    println!("Range updates result: {:?}", result);

    // Two-pointer technique
    fn two_sum(arr: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut left = 0;
        let mut right = arr.len() - 1;

        while left < right {
            let sum = arr[left] + arr[right];
            if sum == target {
                return Some((left, right));
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        None
    }

    let sorted_arr = vec![1, 3, 4, 6, 8, 10];
    if let Some((i, j)) = two_sum(&sorted_arr, 13) {
        println!("Two sum found: {} + {} = 13 (indices {} and {})",
                 sorted_arr[i], sorted_arr[j], i, j);
    }

    // Sliding window maximum
    fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
        if nums.is_empty() || k == 0 {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut window: Vec<usize> = Vec::new();

        for i in 0..nums.len() {
            // Remove elements outside current window
            while !window.is_empty() && window[0] <= i - k {
                window.remove(0);
            }

            // Remove elements smaller than current element
            while !window.is_empty() && nums[window[window.len() - 1]] < nums[i] {
                window.pop();
            }

            window.push(i);

            // Add maximum to result
            if i >= k - 1 {
                result.push(nums[window[0]]);
            }
        }
        result
    }

    let window_data = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let max_window = max_sliding_window(&window_data, 3);
    println!("Sliding window maximum: {:?}", max_window);

    println!();
}

// ================================================================================
// 8. MEMORY-EFFICIENT ARRAYS
// ================================================================================

// Bitset implementation
struct BitSet {
    data: Vec<u64>,
    size: usize,
}

impl BitSet {
    fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        BitSet {
            data: vec![0; words],
            size,
        }
    }

    fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.size);
        let word = index / 64;
        let bit = index % 64;

        if value {
            self.data[word] |= 1 << bit;
        } else {
            self.data[word] &= !(1 << bit);
        }
    }

    fn get(&self, index: usize) -> bool {
        assert!(index < self.size);
        let word = index / 64;
        let bit = index % 64;
        (self.data[word] & (1 << bit)) != 0
    }

    fn count_ones(&self) -> usize {
        self.data.iter().map(|&word| word.count_ones() as usize).sum()
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            write!(f, "{}", if self.get(i) { 1 } else { 0 })?;
        }
        Ok(())
    }
}

fn demonstrate_memory_efficient_arrays() {
    println!("=== MEMORY-EFFICIENT ARRAYS ===");

    // BitSet demonstration
    let mut bitset = BitSet::new(20);
    bitset.set(2, true);
    bitset.set(5, true);
    bitset.set(7, true);
    bitset.set(12, true);
    bitset.set(15, true);

    println!("Bitset: {}", bitset);
    println!("Bit 7 is set: {}", bitset.get(7));
    println!("Bit 8 is set: {}", bitset.get(8));
    println!("Total bits set: {}", bitset.count_ones());

    // Memory usage comparison
    use std::mem;

    let regular_vec: Vec<bool> = vec![false; 1000];
    let bitset_1000 = BitSet::new(1000);

    println!("\nMemory usage for 1000 boolean values:");
    println!("Regular Vec<bool>: {} bytes", mem::size_of_val(&regular_vec));
    println!("BitSet: {} bytes", mem::size_of_val(&bitset_1000));

    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE ARRAY DATA STRUCTURES             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_basic_arrays();
    demonstrate_vector_operations();
    demonstrate_multidimensional_arrays();
    demonstrate_circular_buffer();
    demonstrate_slice_windows();
    demonstrate_matrix_operations();
    demonstrate_advanced_algorithms();
    demonstrate_memory_efficient_arrays();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers:");
    println!("• Basic array operations and methods");
    println!("• Dynamic vectors with capacity management");
    println!("• Multi-dimensional arrays and matrices");
    println!("• Circular buffer for efficient FIFO operations");
    println!("• Slice window operations for sliding window algorithms");
    println!("• Matrix operations (addition, multiplication, transpose)");
    println!("• Advanced algorithms (prefix sums, two-pointer, sliding window)");
    println!("• Memory-efficient structures like BitSet");
    println!();
    println!("Key concepts demonstrated:");
    println!("✅ Stack vs heap allocation");
    println!("✅ Memory management and ownership");
    println!("✅ Iterator patterns and functional programming");
    println!("✅ Performance considerations");
    println!("✅ Type safety and bounds checking");
    println!("✅ Zero-cost abstractions");
}

/*
================================================================================
ARRAY DATA STRUCTURES SUMMARY:
================================================================================

1. BASIC ARRAYS:
   - Fixed-size arrays with type safety
   - Stack allocation for performance
   - Compile-time size checking
   - Slice operations for views

2. DYNAMIC VECTORS:
   - Heap-allocated growable arrays
   - Automatic capacity management
   - Amortized O(1) push/pop operations
   - Rich set of methods and iterators

3. MULTI-DIMENSIONAL STRUCTURES:
   - Nested arrays for matrices
   - Dynamic resizing capabilities
   - Mathematical operations support
   - Row/column access patterns

4. SPECIALIZED STRUCTURES:
   - Circular buffers for FIFO queues
   - Bitsets for memory efficiency
   - Sliding window operations
   - Range update capabilities

5. PERFORMANCE CONSIDERATIONS:
   - Cache-friendly memory layout
   - Contiguous memory allocation
   - SIMD optimization potential
   - Bounds checking vs unsafe operations

6. RUST-SPECIFIC FEATURES:
   - Ownership and borrowing
   - Slice types for views
   - Iterator trait for traversal
   - Generic implementations
   - Memory safety guarantees

COMPILATION:
rustc 01-Arrays.rs -o arrays_demo

USAGE:
./arrays_demo
================================================================================
*/