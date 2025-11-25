#pragma once
#include <vector>
#include <stdexcept>

/**
 * @brief Stack Implementation using Dynamic Array
 *
 * Stack is a LIFO (Last In First Out) data structure.
 * Elements are added and removed from the same end (top).
 *
 * Time Complexity:
 * - Push: Amortized O(1)
 * - Pop: O(1)
 * - Peek/Top: O(1)
 * - IsEmpty: O(1)
 *
 * Applications:
 * - Function call stack
 * - Expression evaluation
 * - Undo/Redo operations
 * - Backtracking algorithms
 */
template <typename T>
class Stack {
private:
    std::vector<T> data;  // Internal storage

public:
    /**
     * @brief Constructor - initialize empty stack
     */
    Stack() {}

    /**
     * @brief Add element to top of stack
     * @param value Element to add
     */
    void push(const T& value) {
        data.push_back(value);
    }

    /**
     * @brief Remove and return top element
     * @return Element at top of stack
     */
    T pop() {
        if (is_empty()) {
            throw std::out_of_range("Stack is empty");
        }

        T top_value = data.back();
        data.pop_back();
        return top_value;
    }

    /**
     * @brief Get top element without removing it
     * @return Const reference to top element
     */
    const T& top() const {
        if (is_empty()) {
            throw std::out_of_range("Stack is empty");
        }
        return data.back();
    }

    /**
     * @brief Get top element (non-const version)
     */
    T& top() {
        if (is_empty()) {
            throw std::out_of_range("Stack is empty");
        }
        return data.back();
    }

    /**
     * @brief Check if stack is empty
     */
    bool is_empty() const {
        return data.empty();
    }

    /**
     * @brief Get number of elements in stack
     */
    size_t size() const {
        return data.size();
    }

    /**
     * @brief Clear all elements
     */
    void clear() {
        data.clear();
    }

    /**
     * @brief Print stack elements (for debugging)
     */
    void print() const {
        std::cout << "Stack (top -> bottom): [";
        for (int i = data.size() - 1; i >= 0; --i) {
            std::cout << data[i];
            if (i > 0) {
                std::cout << ", ";
            }
        }
        std::cout << "]" << std::endl;
    }
};

/**
 * @brief Stack Implementation using Linked List
 *
 * This implementation demonstrates how to build a stack using a linked list
 * as the underlying data structure.
 */
template <typename T>
class LinkedListStack {
private:
    struct Node {
        T data;
        Node* next;
        Node(const T& value) : data(value), next(nullptr) {}
    };

    Node* top_node;  // Pointer to top element
    size_t count;    // Number of elements

public:
    /**
     * @brief Constructor - initialize empty stack
     */
    LinkedListStack() : top_node(nullptr), count(0) {}

    /**
     * @brief Destructor - free all allocated memory
     */
    ~LinkedListStack() {
        clear();
    }

    /**
     * @brief Add element to top of stack
     */
    void push(const T& value) {
        Node* new_node = new Node(value);
        new_node->next = top_node;
        top_node = new_node;
        count++;
    }

    /**
     * @brief Remove and return top element
     */
    T pop() {
        if (is_empty()) {
            throw std::out_of_range("Stack is empty");
        }

        Node* temp = top_node;
        T value = temp->data;
        top_node = top_node->next;
        delete temp;
        count--;
        return value;
    }

    /**
     * @brief Get top element without removing it
     */
    const T& top() const {
        if (is_empty()) {
            throw std::out_of_range("Stack is empty");
        }
        return top_node->data;
    }

    bool is_empty() const { return top_node == nullptr; }
    size_t size() const { return count; }

    void clear() {
        while (!is_empty()) {
            pop();
        }
    }

    void print() const {
        std::cout << "Stack (top -> bottom): [";
        Node* current = top_node;
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
 * @brief Stack Applications and Examples
 */

/**
 * @brief Check if parentheses are balanced
 * @param expression String containing parentheses
 * @return true if balanced, false otherwise
 */
bool is_balanced_parentheses(const std::string& expression) {
    Stack<char> stack;

    for (char ch : expression) {
        if (ch == '(' || ch == '[' || ch == '{') {
            stack.push(ch);
        } else if (ch == ')' || ch == ']' || ch == '}') {
            if (stack.is_empty()) {
                return false;
            }

            char top = stack.pop();
            if ((ch == ')' && top != '(') ||
                (ch == ']' && top != '[') ||
                (ch == '}' && top != '{')) {
                return false;
            }
        }
    }

    return stack.is_empty();
}

/**
 * @brief Evaluate postfix expression
 * @param expression Postfix expression as string
 * @return Result of evaluation
 */
int evaluate_postfix(const std::string& expression) {
    Stack<int> stack;

    for (char ch : expression) {
        if (isdigit(ch)) {
            stack.push(ch - '0');
        } else if (ch == ' ') {
            continue;
        } else {
            // Operator
            if (stack.size() < 2) {
                throw std::invalid_argument("Invalid postfix expression");
            }

            int operand2 = stack.pop();
            int operand1 = stack.pop();
            int result;

            switch (ch) {
                case '+': result = operand1 + operand2; break;
                case '-': result = operand1 - operand2; break;
                case '*': result = operand1 * operand2; break;
                case '/': result = operand1 / operand2; break;
                default: throw std::invalid_argument("Unsupported operator");
            }

            stack.push(result);
        }
    }

    if (stack.size() != 1) {
        throw std::invalid_argument("Invalid postfix expression");
    }

    return stack.pop();
}

/**
 * @brief Reverse a string using stack
 * @param str String to reverse
 * @return Reversed string
 */
std::string reverse_string(const std::string& str) {
    Stack<char> stack;

    // Push all characters onto stack
    for (char ch : str) {
        stack.push(ch);
    }

    // Pop characters to build reversed string
    std::string reversed;
    while (!stack.is_empty()) {
        reversed += stack.pop();
    }

    return reversed;
}

/**
 * @brief Find next greater element for each element in array
 * @param arr Input array
 * @return Vector containing next greater elements
 */
std::vector<int> next_greater_elements(const std::vector<int>& arr) {
    std::vector<int> result(arr.size(), -1);
    Stack<int> stack;  // Stack of indices

    for (size_t i = 0; i < arr.size(); ++i) {
        // While current element is greater than element at stack top
        while (!stack.is_empty() && arr[i] > arr[stack.top()]) {
            int index = stack.pop();
            result[index] = arr[i];
        }
        stack.push(i);
    }

    return result;
}