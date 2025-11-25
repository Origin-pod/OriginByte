#pragma once
#include <vector>
#include <stdexcept>
#include <list>

/**
 * @brief Queue Implementation using Dynamic Array (Circular Buffer)
 *
 * Queue is a FIFO (First In First Out) data structure.
 * Elements are added at the rear and removed from the front.
 *
 * Time Complexity:
 * - Enqueue: Amortized O(1)
 * - Dequeue: O(1)
 * - Front: O(1)
 * - IsEmpty: O(1)
 *
 * Applications:
 * - Task scheduling
 * - Breadth-first search
 * - Printer queues
 * - Message queues
 */
template <typename T>
class Queue {
private:
    std::vector<T> data;     // Internal storage
    size_t front_index;      // Index of front element
    size_t rear_index;       // Index where next element will be inserted
    size_t current_size;     // Current number of elements
    size_t capacity;         // Total capacity of the circular buffer

    /**
     * @brief Resize the internal circular buffer
     * @param new_capacity New capacity to resize to
     */
    void resize(size_t new_capacity) {
        std::vector<T> new_data(new_capacity);

        // Copy elements in order from front to rear
        for (size_t i = 0; i < current_size; ++i) {
            new_data[i] = data[(front_index + i) % capacity];
        }

        data = std::move(new_data);
        front_index = 0;
        rear_index = current_size;
        capacity = new_capacity;
    }

public:
    /**
     * @brief Constructor with initial capacity
     * @param initial_capacity Starting capacity of the queue
     */
    Queue(size_t initial_capacity = 10)
        : front_index(0), rear_index(0), current_size(0), capacity(initial_capacity) {
        data.resize(capacity);
    }

    /**
     * @brief Add element to rear of queue
     * @param value Element to add
     */
    void enqueue(const T& value) {
        if (current_size >= capacity) {
            resize(capacity * 2);
        }

        data[rear_index] = value;
        rear_index = (rear_index + 1) % capacity;
        current_size++;
    }

    /**
     * @brief Remove and return front element
     * @return Element at front of queue
     */
    T dequeue() {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }

        T front_value = data[front_index];
        front_index = (front_index + 1) % capacity;
        current_size--;

        // Optional: Resize down if too empty
        if (current_size > 0 && current_size <= capacity / 4 && capacity > 10) {
            resize(capacity / 2);
        }

        return front_value;
    }

    /**
     * @brief Get front element without removing it
     * @return Const reference to front element
     */
    const T& front() const {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }
        return data[front_index];
    }

    /**
     * @brief Get rear element without removing it
     * @return Const reference to rear element
     */
    const T& rear() const {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }
        return data[(rear_index - 1 + capacity) % capacity];
    }

    /**
     * @brief Check if queue is empty
     */
    bool is_empty() const {
        return current_size == 0;
    }

    /**
     * @brief Get number of elements in queue
     */
    size_t size() const {
        return current_size;
    }

    /**
     * @brief Clear all elements
     */
    void clear() {
        front_index = rear_index = current_size = 0;
    }

    /**
     * @brief Print queue elements (for debugging)
     */
    void print() const {
        std::cout << "Queue (front -> rear): [";
        for (size_t i = 0; i < current_size; ++i) {
            std::cout << data[(front_index + i) % capacity];
            if (i < current_size - 1) {
                std::cout << ", ";
            }
        }
        std::cout << "]" << std::endl;
    }
};

/**
 * @brief Queue Implementation using Linked List
 *
 * This implementation demonstrates how to build a queue using a linked list
 * as the underlying data structure.
 */
template <typename T>
class LinkedListQueue {
private:
    struct Node {
        T data;
        Node* next;
        Node(const T& value) : data(value), next(nullptr) {}
    };

    Node* front_node;   // Pointer to front element
    Node* rear_node;    // Pointer to rear element
    size_t count;       // Number of elements

public:
    /**
     * @brief Constructor - initialize empty queue
     */
    LinkedListQueue() : front_node(nullptr), rear_node(nullptr), count(0) {}

    /**
     * @brief Destructor - free all allocated memory
     */
    ~LinkedListQueue() {
        clear();
    }

    /**
     * @brief Add element to rear of queue
     */
    void enqueue(const T& value) {
        Node* new_node = new Node(value);

        if (rear_node == nullptr) {
            // Queue is empty
            front_node = rear_node = new_node;
        } else {
            rear_node->next = new_node;
            rear_node = new_node;
        }

        count++;
    }

    /**
     * @brief Remove and return front element
     */
    T dequeue() {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }

        Node* temp = front_node;
        T value = temp->data;
        front_node = front_node->next;

        if (front_node == nullptr) {
            rear_node = nullptr;
        }

        delete temp;
        count--;
        return value;
    }

    /**
     * @brief Get front element without removing it
     */
    const T& front() const {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }
        return front_node->data;
    }

    /**
     * @brief Get rear element without removing it
     */
    const T& rear() const {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }
        return rear_node->data;
    }

    bool is_empty() const { return front_node == nullptr; }
    size_t size() const { return count; }

    void clear() {
        while (!is_empty()) {
            dequeue();
        }
    }

    void print() const {
        std::cout << "Queue (front -> rear): [";
        Node* current = front_node;
        bool first = true;
        while (current != nullptr) {
            if (!first) {
                std::cout << ", ";
            }
            std::cout << current->data;
            current = current->next;
            first = false;
        }
        std::cout << "]" << std::endl;
    }
};

/**
 * @brief Priority Queue Implementation
 *
 * Elements are served based on priority rather than insertion order.
 * Lower priority numbers are served first (min-heap behavior).
 */
template <typename T>
class PriorityQueue {
private:
    struct PriorityNode {
        T data;
        int priority;
        PriorityNode* next;

        PriorityNode(const T& value, int prio) : data(value), priority(prio), next(nullptr) {}
    };

    PriorityNode* head;  // Head of priority-sorted list
    size_t count;        // Number of elements

public:
    /**
     * @brief Constructor - initialize empty priority queue
     */
    PriorityQueue() : head(nullptr), count(0) {}

    /**
     * @brief Destructor - free all allocated memory
     */
    ~PriorityQueue() {
        clear();
    }

    /**
     * @brief Add element with specified priority
     * @param value Element to add
     * @param priority Priority level (lower numbers = higher priority)
     */
    void enqueue(const T& value, int priority) {
        PriorityNode* new_node = new PriorityNode(value, priority);

        // Insert in correct position based on priority
        if (head == nullptr || priority < head->priority) {
            new_node->next = head;
            head = new_node;
        } else {
            PriorityNode* current = head;
            while (current->next != nullptr && current->next->priority <= priority) {
                current = current->next;
            }
            new_node->next = current->next;
            current->next = new_node;
        }

        count++;
    }

    /**
     * @brief Remove and return highest priority element
     */
    T dequeue() {
        if (is_empty()) {
            throw std::out_of_range("Priority queue is empty");
        }

        PriorityNode* temp = head;
        T value = temp->data;
        head = head->next;
        delete temp;
        count--;
        return value;
    }

    /**
     * @brief Get highest priority element without removing it
     */
    const T& front() const {
        if (is_empty()) {
            throw std::out_of_range("Priority queue is empty");
        }
        return head->data;
    }

    bool is_empty() const { return head == nullptr; }
    size_t size() const { return count; }

    void clear() {
        while (!is_empty()) {
            dequeue();
        }
    }

    void print() const {
        std::cout << "PriorityQueue (priority -> data): [";
        PriorityNode* current = head;
        bool first = true;
        while (current != nullptr) {
            if (!first) {
                std::cout << ", ";
            }
            std::cout << "(" << current->priority << "->" << current->data << ")";
            current = current->next;
            first = false;
        }
        std::cout << "]" << std::endl;
    }
};

/**
 * @brief Queue Applications and Examples
 */

/**
 * @brief Generate first n numbers in binary sequence using queue
 * @param n Number of binary numbers to generate
 * @return Vector containing binary numbers as strings
 */
std::vector<std::string> generate_binary_numbers(int n) {
    Queue<std::string> queue;
    std::vector<std::string> result;

    if (n <= 0) return result;

    queue.enqueue("1");

    for (int i = 0; i < n; ++i) {
        std::string current = queue.dequeue();
        result.push_back(current);

        queue.enqueue(current + "0");
        queue.enqueue(current + "1");
    }

    return result;
}

/**
 * @brief Simulate task scheduling using queue
 */
class TaskScheduler {
private:
    struct Task {
        std::string name;
        int duration;

        Task(const std::string& n, int d) : name(n), duration(d) {}
    };

    Queue<Task> task_queue;
    int current_time;

public:
    TaskScheduler() : current_time(0) {}

    void add_task(const std::string& name, int duration) {
        task_queue.enqueue(Task(name, duration));
    }

    void process_tasks() {
        std::cout << "Processing tasks:" << std::endl;

        while (!task_queue.is_empty()) {
            Task current = task_queue.dequeue();
            std::cout << "Time " << current_time << ": Processing " << current.name
                      << " (duration: " << current.duration << ")" << std::endl;
            current_time += current.duration;
        }

        std::cout << "All tasks completed at time " << current_time << std::endl;
    }
};

/**
 * @brief Implement queue using two stacks
 * This is a common interview question demonstrating how to implement
 * a queue using only stack operations.
 */
template <typename T>
class QueueWithTwoStacks {
private:
    Stack<T> stack1;  // For enqueue operations
    Stack<T> stack2;  // For dequeue operations

    /**
     * @brief Transfer elements from stack1 to stack2 if stack2 is empty
     */
    void transfer() {
        while (!stack1.is_empty()) {
            stack2.push(stack1.pop());
        }
    }

public:
    void enqueue(const T& value) {
        stack1.push(value);
    }

    T dequeue() {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }

        if (stack2.is_empty()) {
            transfer();
        }

        return stack2.pop();
    }

    const T& front() {
        if (is_empty()) {
            throw std::out_of_range("Queue is empty");
        }

        if (stack2.is_empty()) {
            transfer();
        }

        return stack2.top();
    }

    bool is_empty() const {
        return stack1.is_empty() && stack2.is_empty();
    }

    size_t size() const {
        return stack1.size() + stack2.size();
    }
};