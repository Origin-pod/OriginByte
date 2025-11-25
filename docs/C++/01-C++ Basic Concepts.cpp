/**
 * @file 01-C++ Basic Concepts.cpp
 * @brief Comprehensive guide to fundamental C++ concepts
 *
 * This file covers all the essential C++ concepts that every developer should know,
 * from basic syntax to advanced features. It's organized in a logical learning progression
 * with practical examples and best practices.
 */

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <stdexcept>
#include <typeinfo>
#include <cmath>

// ================================================================================
// 1. BASIC SYNTAX AND DATA TYPES
// ================================================================================

void demonstrate_basic_syntax() {
    std::cout << "=== BASIC SYNTAX AND DATA TYPES ===" << std::endl;

    // Fundamental data types
    int integer_var = 42;                    // 4 bytes typically
    short short_var = 10;                    // 2 bytes
    long long_var = 100000L;                 // 4 or 8 bytes
    long long long_long_var = 123456789LL;   // 8 bytes

    float float_var = 3.14f;                 // 4 bytes, single precision
    double double_var = 3.14159265;          // 8 bytes, double precision
    long double long_double_var = 3.141592653589793L; // Extended precision

    char char_var = 'A';                     // 1 byte
    bool bool_var = true;                    // 1 byte

    // Type modifiers
    unsigned int unsigned_var = 100;         // Only non-negative values
    signed int signed_var = -50;             // Can be positive or negative (default)
    const int constant_var = 42;             // Cannot be modified

    // Auto type deduction (C++11)
    auto auto_int = 42;                      // Deduced as int
    auto auto_double = 3.14;                 // Deduced as double

    std::cout << "Integer: " << integer_var << std::endl;
    std::cout << "Float: " << float_var << std::endl;
    std::cout << "Char: " << char_var << std::endl;
    std::cout << "Bool: " << bool_var << std::endl;
    std::cout << std::endl;
}

// ================================================================================
// 2. OPERATORS
// ================================================================================

void demonstrate_operators() {
    std::cout << "=== OPERATORS ===" << std::endl;

    int a = 10, b = 3;

    // Arithmetic operators
    std::cout << "Arithmetic: " << a << " + " << b << " = " << (a + b) << std::endl;
    std::cout << "Arithmetic: " << a << " - " << b << " = " << (a - b) << std::endl;
    std::cout << "Arithmetic: " << a << " * " << b << " = " << (a * b) << std::endl;
    std::cout << "Arithmetic: " << a << " / " << b << " = " << (a / b) << std::endl;
    std::cout << "Arithmetic: " << a << " % " << b << " = " << (a % b) << std::endl;

    // Comparison operators
    std::cout << "Comparison: " << a << " > " << b << " is " << (a > b) << std::endl;
    std::cout << "Comparison: " << a << " == " << b << " is " << (a == b) << std::endl;
    std::cout << "Comparison: " << a << " != " << b << " is " << (a != b) << std::endl;

    // Logical operators
    bool x = true, y = false;
    std::cout << "Logical: " << x << " && " << y << " = " << (x && y) << std::endl;
    std::cout << "Logical: " << x << " || " << y << " = " << (x || y) << std::endl;
    std::cout << "Logical: !" << x << " = " << (!x) << std::endl;

    // Increment/Decrement operators
    int c = 5;
    std::cout << "Increment: " << c++ << " (post-increment)" << std::endl;
    std::cout << "Increment: " << ++c << " (pre-increment)" << std::endl;

    // Assignment operators
    int d = 10;
    d += 5;  // d = d + 5
    d *= 2;  // d = d * 2
    std::cout << "Assignment: result is " << d << std::endl;
    std::cout << std::endl;
}

// ================================================================================
// 3. CONTROL FLOW
// ================================================================================

void demonstrate_control_flow() {
    std::cout << "=== CONTROL FLOW ===" << std::endl;

    // If-else statements
    int age = 25;
    if (age < 18) {
        std::cout << "Minor" << std::endl;
    } else if (age < 65) {
        std::cout << "Adult" << std::endl;
    } else {
        std::cout << "Senior" << std::endl;
    }

    // Switch statement
    char grade = 'B';
    switch (grade) {
        case 'A':
            std::cout << "Excellent!" << std::endl;
            break;
        case 'B':
            std::cout << "Good!" << std::endl;
            break;
        case 'C':
            std::cout << "Average" << std::endl;
            break;
        default:
            std::cout << "Need improvement" << std::endl;
    }

    // Loops
    std::cout << "For loop: ";
    for (int i = 0; i < 5; i++) {
        std::cout << i << " ";
    }
    std::cout << std::endl;

    std::cout << "While loop: ";
    int j = 0;
    while (j < 5) {
        std::cout << j << " ";
        j++;
    }
    std::cout << std::endl;

    std::cout << "Do-while loop: ";
    int k = 0;
    do {
        std::cout << k << " ";
        k++;
    } while (k < 5);
    std::cout << std::endl;

    // Range-based for loop (C++11)
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::cout << "Range-based for: ";
    for (int num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // Break and continue
    std::cout << "Break example: ";
    for (int i = 0; i < 10; i++) {
        if (i == 5) break;
        std::cout << i << " ";
    }
    std::cout << std::endl;

    std::cout << "Continue example: ";
    for (int i = 0; i < 10; i++) {
        if (i % 2 == 0) continue;
        std::cout << i << " ";
    }
    std::cout << std::endl;
    std::cout << std::endl;
}

// ================================================================================
// 4. FUNCTIONS
// ================================================================================

// Function declaration and definition
int add(int a, int b) {
    return a + b;
}

// Function with default parameters
void greet(const std::string& name, const std::string& title = "Mr./Ms.") {
    std::cout << "Hello, " << title << " " << name << "!" << std::endl;
}

// Function overloading
int multiply(int a, int b) {
    return a * b;
}

double multiply(double a, double b) {
    return a * b;
}

// Inline function
inline int square(int x) {
    return x * x;
}

// Function templates
template<typename T>
T maximum(T a, T b) {
    return (a > b) ? a : b;
}

// Lambda expressions (C++11)
void demonstrate_functions() {
    std::cout << "=== FUNCTIONS ===" << std::endl;

    // Basic function call
    std::cout << "add(5, 3) = " << add(5, 3) << std::endl;

    // Default parameters
    greet("Smith");
    greet("Johnson", "Dr.");

    // Function overloading
    std::cout << "multiply(4, 5) = " << multiply(4, 5) << std::endl;
    std::cout << "multiply(4.5, 2.5) = " << multiply(4.5, 2.5) << std::endl;

    // Template function
    std::cout << "maximum(10, 20) = " << maximum(10, 20) << std::endl;
    std::cout << "maximum(3.14, 2.71) = " << maximum(3.14, 2.71) << std::endl;

    // Lambda expressions
    auto add_lambda = [](int a, int b) { return a + b; };
    std::cout << "Lambda add(7, 8) = " << add_lambda(7, 8) << std::endl;

    // Lambda with capture
    int multiplier = 3;
    auto multiply_by = [multiplier](int x) { return x * multiplier; };
    std::cout << "Lambda multiply_by(4) = " << multiply_by(4) << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 5. ARRAYS AND STRINGS
// ================================================================================

void demonstrate_arrays_and_strings() {
    std::cout << "=== ARRAYS AND STRINGS ===" << std::endl;

    // C-style arrays
    int c_array[5] = {1, 2, 3, 4, 5};
    std::cout << "C-style array: ";
    for (int i = 0; i < 5; i++) {
        std::cout << c_array[i] << " ";
    }
    std::cout << std::endl;

    // std::array (C++11)
    std::array<int, 5> std_array = {1, 2, 3, 4, 5};
    std::cout << "std::array: ";
    for (int num : std_array) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // std::vector (dynamic array)
    std::vector<int> vec = {10, 20, 30, 40, 50};
    vec.push_back(60);  // Add element
    std::cout << "std::vector: ";
    for (int num : vec) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    std::cout << "Vector size: " << vec.size() << std::endl;

    // C-style strings
    char c_string[] = "Hello, World!";
    std::cout << "C-style string: " << c_string << std::endl;

    // std::string
    std::string str1 = "Hello";
    std::string str2 = " World";
    std::string str3 = str1 + str2;  // Concatenation

    std::cout << "std::string: " << str3 << std::endl;
    std::cout << "String length: " << str3.length() << std::endl;
    std::cout << "First character: " << str3[0] << std::endl;
    std::cout << "Substring: " << str3.substr(0, 5) << std::endl;

    // String methods
    std::string text = "Hello, C++ Programming!";
    std::cout << "Original: " << text << std::endl;
    std::cout << "Contains 'C++': " << (text.find("C++") != std::string::npos) << std::endl;
    std::cout << "Replace: " << text.replace(7, 3, "Awesome") << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 6. POINTERS AND REFERENCES
// ================================================================================

void demonstrate_pointers_and_references() {
    std::cout << "=== POINTERS AND REFERENCES ===" << std::endl;

    // Pointers
    int var = 42;
    int* ptr = &var;  // Pointer to var

    std::cout << "Variable value: " << var << std::endl;
    std::cout << "Variable address: " << &var << std::endl;
    std::cout << "Pointer value (address): " << ptr << std::endl;
    std::cout << "Pointer dereference: " << *ptr << std::endl;
    std::cout << "Pointer address: " << &ptr << std::endl;

    // Null pointer
    int* null_ptr = nullptr;  // Modern C++ (C++11)
    std::cout << "Null pointer is: " << (null_ptr == nullptr) << std::endl;

    // Pointer arithmetic
    int arr[5] = {10, 20, 30, 40, 50};
    int* arr_ptr = arr;
    std::cout << "Array via pointer: ";
    for (int i = 0; i < 5; i++) {
        std::cout << *(arr_ptr + i) << " ";  // arr_ptr[i] also works
    }
    std::cout << std::endl;

    // Dynamic memory allocation
    int* dynamic_ptr = new int(100);
    std::cout << "Dynamic value: " << *dynamic_ptr << std::endl;
    delete dynamic_ptr;  // Don't forget to free memory

    // Dynamic array allocation
    int* dynamic_array = new int[5]{1, 2, 3, 4, 5};
    std::cout << "Dynamic array: ";
    for (int i = 0; i < 5; i++) {
        std::cout << dynamic_array[i] << " ";
    }
    std::cout << std::endl;
    delete[] dynamic_array;  // Free array memory

    // References
    int original = 10;
    int& ref = original;  // Reference to original

    std::cout << "Original: " << original << std::endl;
    std::cout << "Reference: " << ref << std::endl;

    ref = 20;  // Modifies original
    std::cout << "After modification: " << original << std::endl;

    // Reference as function parameter (pass by reference)
    auto swap_ref = [](int& a, int& b) {
        int temp = a;
        a = b;
        b = temp;
    };

    int x = 5, y = 10;
    std::cout << "Before swap: x=" << x << ", y=" << y << std::endl;
    swap_ref(x, y);
    std::cout << "After swap: x=" << x << ", y=" << y << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 7. CLASSES AND OBJECTS (OOP)
// ================================================================================

class Rectangle {
private:
    double width;
    double height;

public:
    // Constructor
    Rectangle(double w, double h) : width(w), height(h) {
        std::cout << "Rectangle created: " << w << " x " << h << std::endl;
    }

    // Default constructor
    Rectangle() : width(0), height(0) {
        std::cout << "Default rectangle created" << std::endl;
    }

    // Destructor
    ~Rectangle() {
        std::cout << "Rectangle destroyed" << std::endl;
    }

    // Getter methods
    double getWidth() const { return width; }
    double getHeight() const { return height; }

    // Setter methods
    void setWidth(double w) { width = w; }
    void setHeight(double h) { height = h; }

    // Member function
    double area() const {
        return width * height;
    }

    // Const member function
    double perimeter() const {
        return 2 * (width + height);
    }

    // Static member
    static int objectCount;

    // Static method
    static int getObjectCount() {
        return objectCount;
    }
};

// Initialize static member
int Rectangle::objectCount = 0;

// Inheritance example
class Shape {
protected:
    std::string name;

public:
    Shape(const std::string& n) : name(n) {}
    virtual ~Shape() {}

    virtual double area() const = 0;  // Pure virtual function
    virtual void display() const {
        std::cout << "Shape: " << name << std::endl;
    }
};

class Circle : public Shape {
private:
    double radius;

public:
    Circle(double r) : Shape("Circle"), radius(r) {}

    double area() const override {
        return M_PI * radius * radius;
    }

    void display() const override {
        std::cout << "Circle with radius " << radius << std::endl;
    }
};

void demonstrate_oop() {
    std::cout << "=== CLASSES AND OBJECTS ===" << std::endl;

    // Create objects
    Rectangle rect1(5.0, 3.0);
    Rectangle rect2;

    std::cout << "Rectangle 1 area: " << rect1.area() << std::endl;
    std::cout << "Rectangle 1 perimeter: " << rect1.perimeter() << std::endl;

    rect2.setWidth(4.0);
    rect2.setHeight(6.0);
    std::cout << "Rectangle 2 area: " << rect2.area() << std::endl;

    // Polymorphism
    Shape* shape = new Circle(2.5);
    shape->display();
    std::cout << "Circle area: " << shape->area() << std::endl;
    delete shape;

    std::cout << std::endl;
}

// ================================================================================
// 8. EXCEPTION HANDLING
// ================================================================================

void demonstrate_exception_handling() {
    std::cout << "=== EXCEPTION HANDLING ===" << std::endl;

    // Basic try-catch
    try {
        int denominator = 0;
        if (denominator == 0) {
            throw std::runtime_error("Division by zero!");
        }
        int result = 10 / denominator;
        std::cout << "Result: " << result << std::endl;
    }
    catch (const std::runtime_error& e) {
        std::cout << "Caught exception: " << e.what() << std::endl;
    }

    // Multiple catch blocks
    try {
        std::string str = "hello";
        char ch = str.at(10);  // Out of bounds
    }
    catch (const std::out_of_range& e) {
        std::cout << "Out of range exception: " << e.what() << std::endl;
    }
    catch (const std::exception& e) {
        std::cout << "Standard exception: " << e.what() << std::endl;
    }
    catch (...) {
        std::cout << "Unknown exception caught" << std::endl;
    }

    std::cout << std::endl;
}

// ================================================================================
// 9. SMART POINTERS (C++11)
// ================================================================================

class SmartPointerDemo {
public:
    SmartPointerDemo() {
        std::cout << "SmartPointerDemo created" << std::endl;
    }

    ~SmartPointerDemo() {
        std::cout << "SmartPointerDemo destroyed" << std::endl;
    }

    void sayHello() {
        std::cout << "Hello from SmartPointerDemo!" << std::endl;
    }
};

void demonstrate_smart_pointers() {
    std::cout << "=== SMART POINTERS ===" << std::endl;

    // unique_ptr - exclusive ownership
    {
        std::unique_ptr<SmartPointerDemo> uniquePtr = std::make_unique<SmartPointerDemo>();
        uniquePtr->sayHello();
    }  // Automatically freed here

    // shared_ptr - shared ownership
    {
        std::shared_ptr<SmartPointerDemo> sharedPtr1 = std::make_shared<SmartPointerDemo>();
        {
            std::shared_ptr<SmartPointerDemo> sharedPtr2 = sharedPtr1;
            std::cout << "Reference count: " << sharedPtr1.use_count() << std::endl;
        }
        std::cout << "Reference count after inner scope: " << sharedPtr1.use_count() << std::endl;
    }

    // weak_ptr - non-owning reference
    std::shared_ptr<int> sharedInt = std::make_shared<int>(42);
    std::weak_ptr<int> weakInt = sharedInt;

    std::cout << "Is weak_ptr expired? " << weakInt.expired() << std::endl;
    if (auto locked = weakInt.lock()) {
        std::cout << "Value via weak_ptr: " << *locked << std::endl;
    }

    std::cout << std::endl;
}

// ================================================================================
// 10. TEMPLATES AND GENERICS
// ================================================================================

// Class template
template<typename T>
class Stack {
private:
    std::vector<T> elements;

public:
    void push(const T& element) {
        elements.push_back(element);
    }

    T pop() {
        if (elements.empty()) {
            throw std::runtime_error("Stack is empty");
        }
        T element = elements.back();
        elements.pop_back();
        return element;
    }

    bool isEmpty() const {
        return elements.empty();
    }
};

// Template specialization
template<>
class Stack<const char*> {
private:
    std::vector<std::string> elements;

public:
    void push(const char* element) {
        elements.push_back(std::string(element));
    }

    std::string pop() {
        if (elements.empty()) {
            throw std::runtime_error("Stack is empty");
        }
        std::string element = elements.back();
        elements.pop_back();
        return element;
    }

    bool isEmpty() const {
        return elements.empty();
    }
};

void demonstrate_templates() {
    std::cout << "=== TEMPLATES AND GENERICS ===" << std::endl;

    // Using class template
    Stack<int> intStack;
    intStack.push(10);
    intStack.push(20);
    intStack.push(30);

    std::cout << "Integer stack: ";
    while (!intStack.isEmpty()) {
        std::cout << intStack.pop() << " ";
    }
    std::cout << std::endl;

    Stack<std::string> stringStack;
    stringStack.push("Hello");
    stringStack.push("World");

    std::cout << "String stack: ";
    while (!stringStack.isEmpty()) {
        std::cout << stringStack.pop() << " ";
    }
    std::cout << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 11. TYPE TRAITS AND CONSTEXPR (C++11/14/17)
// ================================================================================

template<typename T>
constexpr T max_constexpr(T a, T b) {
    return (a > b) ? a : b;
}

void demonstrate_modern_cpp() {
    std::cout << "=== MODERN C++ FEATURES ===" << std::endl;

    // constexpr
    constexpr int compile_time_max = max_constexpr(10, 20);
    std::cout << "Compile-time max: " << compile_time_max << std::endl;

    // decltype
    int x = 5;
    decltype(x) y = 10;  // y has same type as x (int)
    std::cout << "decltype result: " << y << std::endl;

    // Range-based for loop with const reference
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::cout << "Numbers (const reference): ";
    for (const int& num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // nullptr (C++11)
    int* ptr = nullptr;
    std::cout << "nullptr test: " << (ptr == nullptr) << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 12. MAIN FUNCTION - DEMO RUNNER
// ================================================================================

int main() {
    std::cout << "╔══════════════════════════════════════════════════════════════╗" << std::endl;
    std::cout << "║           COMPREHENSIVE C++ BASIC CONCEPTS GUIDE            ║" << std::endl;
    std::cout << "╚══════════════════════════════════════════════════════════════╝" << std::endl;
    std::cout << std::endl;

    try {
        demonstrate_basic_syntax();
        demonstrate_operators();
        demonstrate_control_flow();
        demonstrate_functions();
        demonstrate_arrays_and_strings();
        demonstrate_pointers_and_references();
        demonstrate_oop();
        demonstrate_exception_handling();
        demonstrate_smart_pointers();
        demonstrate_templates();
        demonstrate_modern_cpp();

        std::cout << "╔══════════════════════════════════════════════════════════════╗" << std::endl;
        std::cout << "║                    CONCLUSION                           ║" << std::endl;
        std::cout << "╚══════════════════════════════════════════════════════════════╝" << std::endl;
        std::cout << "This guide covers the fundamental C++ concepts that form" << std::endl;
        std::cout << "the foundation for advanced C++ programming." << std::endl;
        std::cout << std::endl;
        std::cout << "Next steps:" << std::endl;
        std::cout << "1. Practice these concepts with small projects" << std::endl;
        std::cout << "2. Study C++ idiomatic patterns (see next file)" << std::endl;
        std::cout << "3. Learn STL containers and algorithms" << std::endl;
        std::cout << "4. Explore advanced topics (concurrency, templates, etc.)" << std::endl;

    }
    catch (const std::exception& e) {
        std::cerr << "Error occurred: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}

/*
================================================================================
KEY TAKEAWAYS FROM THIS GUIDE:
================================================================================

1. FUNDAMENTALS:
   - C++ is a statically-typed, compiled language
   - Strong type system with automatic type deduction (auto)
   - Rich set of fundamental data types

2. MEMORY MANAGEMENT:
   - Manual management with new/delete
   - Smart pointers for automatic memory management
   - Stack vs heap allocation

3. OBJECT-ORIENTED PROGRAMMING:
   - Classes and objects with encapsulation
   - Inheritance and polymorphism
   - Virtual functions and abstract classes

4. MODERN C++ FEATURES:
   - Smart pointers (unique_ptr, shared_ptr, weak_ptr)
   - Lambda expressions
   - Auto type deduction
   - Range-based for loops
   - constexpr and type traits

5. BEST PRACTICES:
   - Use smart pointers instead of raw pointers
   - Prefer std::array/std::vector over C-style arrays
   - Use std::string instead of C-style strings
   - Leverage RAII (Resource Acquisition Is Initialization)
   - Use exceptions for error handling

6. COMPILATION:
   g++ -std=c++17 -Wall -Wextra -O2 01-C++_Basic_Concepts.cpp -o concepts_demo
================================================================================
*/