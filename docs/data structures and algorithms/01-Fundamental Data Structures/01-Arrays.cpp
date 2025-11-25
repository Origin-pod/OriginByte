#pragma once
#include <vector>
#include <stdexcept>
#include <iostream>

/**
 * @brief Dynamic Array implementation similar to std::vector
 *
 * This is a custom implementation of a dynamic array that automatically
 * resizes when it reaches capacity. It demonstrates the core concepts
 * of dynamic memory management and array operations.
 *
 * Time Complexity:
 * - Access: O(1)
 * - Insert at end: Amortized O(1) - The actual time complexity of the operation is averaged over multiple operations.
 * - Insert at arbitrary position: O(n)
 * - Delete: O(n)
 */

/**
 * @brief A template is a blueprint for creating classes and functions. It allows us to write code that can work with different types without duplicating code.
 *
 * The `typename` keyword is used in templates to specify that a type is being used. It is used instead of `class` when the type is dependent on a template parameter.
 *
 * For example, consider the following template:
 *
 */
template <typename T>
class DynamicArray {
private:
    T* data;           // Pointer to dynamically allocated array
    size_t capacity;   // Total allocated capacity

    /**
     * @brief `size_t` is an unsigned integer type that is used to represent the size of an object. 
     * 
     * It is typically used to represent sizes of objects, indices, or counts. It is defined in the `<cstddef>` header.
     * 
     * In this case, `size_t` is used to store the capacity of the dynamic array, which represents the total number of elements that can be stored in the array.
     */
    size_t size;       // Current number of elements

    /**
     * @brief Resize the internal array to new capacity
     * @param new_capacity The new capacity to resize to
     */
    void resize(size_t new_capacity) {
        // Allocate memory for the new array
        // The `new` keyword is used to dynamically allocate memory on the heap. It returns a pointer to the first element of the allocated memory.
        // `T*` is a pointer to a type T. In this case, `T` is the template parameter of the `DynamicArray` class, which represents the type of elements in the array.
        // `new T[new_capacity]` creates an array of `new_capacity` elements of type `T` and returns a pointer to the first element of the array.
        // The allocated memory is not automatically initialized, so it contains unpredictable values.
        // The allocated memory is automatically deallocated when it is no longer needed by the program, which is typically done using the `delete` keyword.
        T* new_data = new T[new_capacity];

        // Copy existing elements to new array
        for (size_t i = 0; i < size; ++i) {
            new_data[i] = data[i];
        }

        // Free old memory and update pointers
        delete[] data;
        data = new_data;
        capacity = new_capacity;
    }

public:
    /**
     * @brief Constructor with initial capacity
     * @param initial_capacity Starting capacity of the array
     */
    DynamicArray(size_t initial_capacity = 10)
        : capacity(initial_capacity), size(0) {
        data = new T[capacity];
    }

    /**
     * @brief Copy constructor
     */
    DynamicArray(const DynamicArray& other)
        : capacity(other.capacity), size(other.size) {
        data = new T[capacity];
        for (size_t i = 0; i < size; ++i) {
            data[i] = other.data[i];
        }
    }

    /**
     * @brief Destructor - clean up allocated memory
     */
    ~DynamicArray() {
        delete[] data;
    }

    /**
     * @brief Add element to the end of array
     * @param value Element to add
     */
    void push_back(const T& value) {
        if (size >= capacity) {
            resize(capacity * 2);  // Double the capacity
        }
        data[size++] = value;
    }

    /**
     * @brief Remove last element
     */
    void pop_back() {
        if (size == 0) {
            throw std::out_of_range("Array is empty");
        }
        size--;
    }

    /**
     * @brief Insert element at specific position
     * @param index Position to insert at
     * @param value Element to insert
     */
    void insert(size_t index, const T& value) {
        if (index > size) {
            throw std::out_of_range("Index out of bounds");
        }

        if (size >= capacity) {
            resize(capacity * 2);
        }

        // Shift elements to the right
        for (size_t i = size; i > index; --i) {
            data[i] = data[i - 1];
        }

        data[index] = value;
        size++;
    }

    /**
     * @brief Remove element at specific position
     * @param index Position to remove from
     */
    void remove(size_t index) {
        if (index >= size) {
            throw std::out_of_range("Index out of bounds");
        }

        // Shift elements to the left
        for (size_t i = index; i < size - 1; ++i) {
            data[i] = data[i + 1];
        }

        size--;
    }

    /**
     * @brief Get element at specific position (read-only)
     * @param index Position to access
     * @return Const reference to element
     */
    const T& at(size_t index) const {
        if (index >= size) {
            throw std::out_of_range("Index out of bounds");
        }
        return data[index];
    }

    /**
     * @brief Get element at specific position (read-write)
     * @param index Position to access
     * @return Reference to element
     */
    T& at(size_t index) {
        if (index >= size) {
            throw std::out_of_range("Index out of bounds");
        }
        return data[index];
    }

    /**
     * @brief Subscript operator for easy access
     */
    T& operator[](size_t index) {
        return data[index];
    }

    const T& operator[](size_t index) const {
        return data[index];
    }

    /**
     * @brief Get current number of elements
     */
    size_t get_size() const {
        return size;
    }

    /**
     * @brief Get current capacity
     */
    size_t get_capacity() const {
        return capacity;
    }

    /**
     * @brief Check if array is empty
     */
    bool is_empty() const {
        return size == 0;
    }

    /**
     * @brief Clear all elements
     */
    void clear() {
        size = 0;
    }

    /**
     * @brief Find first occurrence of value
     * @param value Value to search for
     * @return Index of first occurrence, or size if not found
     */
    size_t find(const T& value) const {
        for (size_t i = 0; i < size; ++i) {
            if (data[i] == value) {
                return i;
            }
        }
        return size;  // Not found
    }

    /**
     * @brief Print array elements (for debugging)
     */
    void print() const {
        std::cout << "[";
        for (size_t i = 0; i < size; ++i) {
            std::cout << data[i];
            if (i < size - 1) {
                std::cout << ", ";
            }
        }
        std::cout << "]" << std::endl;
    }
};