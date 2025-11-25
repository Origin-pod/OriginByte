#pragma once
#include <iostream>
#include <stdexcept>

/**
 * @brief Singly Linked List Implementation
 *
 * A linked list is a linear data structure where elements are stored in nodes.
 * Each node contains data and a pointer to the next node in the sequence.
 *
 * Time Complexity:
 * - Access: O(n)
 * - Insert at beginning: O(1)
 * - Insert at end: O(n)
 * - Delete: O(n) (need to find the node first)
 * - Search: O(n)
 */
template <typename T>
class SinglyLinkedList {
private:
    struct Node {
        T data;
        Node* next;

        Node(const T& value) : data(value), next(nullptr) {}
    };

    Node* head;     // Pointer to first node
    Node* tail;     // Pointer to last node
    size_t count;   // Number of nodes in list

public:
    /**
     * @brief Constructor - initialize empty list
     */
    SinglyLinkedList() : head(nullptr), tail(nullptr), count(0) {}

    /**
     * @brief Copy constructor
     */
    SinglyLinkedList(const SinglyLinkedList& other) : head(nullptr), tail(nullptr), count(0) {
        Node* current = other.head;
        while (current != nullptr) {
            push_back(current->data);
            current = current->next;
        }
    }

    /**
     * @brief Destructor - free all allocated memory
     */
    ~SinglyLinkedList() {
        clear();
    }

    /**
     * @brief Add element to the beginning of list
     * @param value Element to add
     */
    void push_front(const T& value) {
        Node* new_node = new Node(value);
        new_node->next = head;
        head = new_node;

        if (tail == nullptr) {
            tail = new_node;
        }

        count++;
    }

    /**
     * @brief Add element to the end of list
     * @param value Element to add
     */
    void push_back(const T& value) {
        Node* new_node = new Node(value);

        if (tail == nullptr) {
            head = tail = new_node;
        } else {
            tail->next = new_node;
            tail = new_node;
        }

        count++;
    }

    /**
     * @brief Remove first element
     */
    void pop_front() {
        if (head == nullptr) {
            throw std::out_of_range("List is empty");
        }

        Node* temp = head;
        head = head->next;

        if (head == nullptr) {
            tail = nullptr;
        }

        delete temp;
        count--;
    }

    /**
     * @brief Remove last element
     */
    void pop_back() {
        if (head == nullptr) {
            throw std::out_of_range("List is empty");
        }

        if (head == tail) {
            delete head;
            head = tail = nullptr;
        } else {
            Node* current = head;
            while (current->next != tail) {
                current = current->next;
            }

            delete tail;
            tail = current;
            tail->next = nullptr;
        }

        count--;
    }

    /**
     * @brief Insert element at specific position
     * @param index Position to insert at (0-based)
     * @param value Element to insert
     */
    void insert(size_t index, const T& value) {
        if (index > count) {
            throw std::out_of_range("Index out of bounds");
        }

        if (index == 0) {
            push_front(value);
        } else if (index == count) {
            push_back(value);
        } else {
            Node* current = head;
            for (size_t i = 0; i < index - 1; ++i) {
                current = current->next;
            }

            Node* new_node = new Node(value);
            new_node->next = current->next;
            current->next = new_node;
            count++;
        }
    }

    /**
     * @brief Remove element at specific position
     * @param index Position to remove from (0-based)
     */
    void remove(size_t index) {
        if (index >= count) {
            throw std::out_of_range("Index out of bounds");
        }

        if (index == 0) {
            pop_front();
        } else if (index == count - 1) {
            pop_back();
        } else {
            Node* current = head;
            for (size_t i = 0; i < index - 1; ++i) {
                current = current->next;
            }

            Node* temp = current->next;
            current->next = temp->next;
            delete temp;
            count--;
        }
    }

    /**
     * @brief Get first element
     */
    T& front() {
        if (head == nullptr) {
            throw std::out_of_range("List is empty");
        }
        return head->data;
    }

    /**
     * @brief Get last element
     */
    T& back() {
        if (tail == nullptr) {
            throw std::out_of_range("List is empty");
        }
        return tail->data;
    }

    /**
     * @brief Check if list is empty
     */
    bool is_empty() const {
        return head == nullptr;
    }

    /**
     * @brief Get number of elements
     */
    size_t size() const {
        return count;
    }

    /**
     * @brief Search for value in list
     * @param value Value to search for
     * @return Index of first occurrence, or count if not found
     */
    size_t find(const T& value) const {
        Node* current = head;
        size_t index = 0;

        while (current != nullptr) {
            if (current->data == value) {
                return index;
            }
            current = current->next;
            index++;
        }

        return count;  // Not found
    }

    /**
     * @brief Clear all elements
     */
    void clear() {
        while (head != nullptr) {
            Node* temp = head;
            head = head->next;
            delete temp;
        }
        tail = nullptr;
        count = 0;
    }

    /**
     * @brief Print list elements (for debugging)
     */
    void print() const {
        Node* current = head;
        std::cout << "[";
        while (current != nullptr) {
            std::cout << current->data;
            if (current->next != nullptr) {
                std::cout << " -> ";
            }
            current = current->next;
        }
        std::cout << "]" << std::endl;
    }
};

/**
 * @brief Doubly Linked List Implementation
 *
 * Each node has pointers to both next and previous nodes,
 * allowing bidirectional traversal.
 *
 * Time Complexity:
 * - Access: O(n)
 * - Insert at beginning/end: O(1)
 * - Insert at arbitrary position: O(n)
 * - Delete: O(n) (but easier than singly linked)
 */
template <typename T>
class DoublyLinkedList {
private:
    struct Node {
        T data;
        Node* next;
        Node* prev;

        Node(const T& value) : data(value), next(nullptr), prev(nullptr) {}
    };

    Node* head;
    Node* tail;
    size_t count;

public:
    /**
     * @brief Constructor - initialize empty list
     */
    DoublyLinkedList() : head(nullptr), tail(nullptr), count(0) {}

    /**
     * @brief Destructor - free all allocated memory
     */
    ~DoublyLinkedList() {
        clear();
    }

    /**
     * @brief Add element to the beginning
     */
    void push_front(const T& value) {
        Node* new_node = new Node(value);

        if (head == nullptr) {
            head = tail = new_node;
        } else {
            new_node->next = head;
            head->prev = new_node;
            head = new_node;
        }

        count++;
    }

    /**
     * @brief Add element to the end
     */
    void push_back(const T& value) {
        Node* new_node = new Node(value);

        if (tail == nullptr) {
            head = tail = new_node;
        } else {
            new_node->prev = tail;
            tail->next = new_node;
            tail = new_node;
        }

        count++;
    }

    /**
     * @brief Remove first element
     */
    void pop_front() {
        if (head == nullptr) {
            throw std::out_of_range("List is empty");
        }

        Node* temp = head;
        head = head->next;

        if (head != nullptr) {
            head->prev = nullptr;
        } else {
            tail = nullptr;
        }

        delete temp;
        count--;
    }

    /**
     * @brief Remove last element
     */
    void pop_back() {
        if (tail == nullptr) {
            throw std::out_of_range("List is empty");
        }

        Node* temp = tail;
        tail = tail->prev;

        if (tail != nullptr) {
            tail->next = nullptr;
        } else {
            head = nullptr;
        }

        delete temp;
        count--;
    }

    /**
     * @brief Print list forward
     */
    void print_forward() const {
        Node* current = head;
        std::cout << "[";
        while (current != nullptr) {
            std::cout << current->data;
            if (current->next != nullptr) {
                std::cout << " <-> ";
            }
            current = current->next;
        }
        std::cout << "]" << std::endl;
    }

    /**
     * @brief Print list backward
     */
    void print_backward() const {
        Node* current = tail;
        std::cout << "[";
        while (current != nullptr) {
            std::cout << current->data;
            if (current->prev != nullptr) {
                std::cout << " <-> ";
            }
            current = current->prev;
        }
        std::cout << "]" << std::endl;
    }

    size_t size() const { return count; }
    bool is_empty() const { return head == nullptr; }
    void clear() { while (!is_empty()) pop_front(); }
};