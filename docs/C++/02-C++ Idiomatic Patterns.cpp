/**
 * @file 02-C++ Idiomatic Patterns.cpp
 * @brief Comprehensive guide to idiomatic C++ patterns and best practices
 *
 * This file covers the most important C++ idioms and patterns that experienced
 * C++ developers use. These patterns leverage the language's features to write
 * efficient, maintainable, and modern C++ code.
 */

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <algorithm>
#include <numeric>
#include <map>
#include <unordered_map>
#include <set>
#include <unordered_set>
#include <tuple>
#include <optional>
#include <variant>
#include <chrono>
#include <functional>
#include <iterator>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <cassert>
#include <type_traits>

// ================================================================================
// 1. RAII (RESOURCE ACQUISITION IS INITIALIZATION)
// ================================================================================

class RAIIExample {
private:
    std::unique_ptr<int[]> data;
    size_t size;
    std::ofstream file;

public:
    // Constructor acquires all resources
    RAIIExample(size_t n) : size(n), data(std::make_unique<int[]>(n)),
                           file("output.txt", std::ios::app) {
        if (!file.is_open()) {
            throw std::runtime_error("Failed to open file");
        }
        std::cout << "Resources acquired: array of size " << n << " and file handle" << std::endl;
    }

    // Destructor automatically releases resources
    ~RAIIExample() {
        std::cout << "Resources automatically released" << std::endl;
        // No need for manual cleanup - smart pointers and file handle close automatically
    }

    void process(size_t index, int value) {
        if (index < size) {
            data[index] = value;
            file << "Processed index " << index << " with value " << value << std::endl;
        }
    }

    // Copy operations deleted to prevent double resource management
    RAIIExample(const RAIIExample&) = delete;
    RAIIExample& operator=(const RAIIExample&) = delete;

    // Move operations allowed for efficient transfer
    RAIIExample(RAIIExample&& other) noexcept
        : data(std::move(other.data)), size(other.size), file(std::move(other.file)) {
        other.size = 0;
    }

    RAIIExample& operator=(RAIIExample&& other) noexcept {
        if (this != &other) {
            data = std::move(other.data);
            size = other.size;
            file = std::move(other.file);
            other.size = 0;
        }
        return *this;
    }
};

void demonstrate_raii() {
    std::cout << "=== RAII (Resource Acquisition Is Initialization) ===" << std::endl;

    {
        RAIIExample example(100);
        example.process(0, 42);
        example.process(99, 100);
    } // Resources automatically released here

    std::cout << std::endl;
}

// ================================================================================
// 2. SMART POINTERS AND OWNERSHIP SEMANTICS
// ================================================================================

class Widget {
public:
    Widget() { std::cout << "Widget created" << std::endl; }
    ~Widget() { std::cout << "Widget destroyed" << std::endl; }

    void doSomething() { std::cout << "Widget doing something" << std::endl; }
};

void demonstrate_smart_pointers() {
    std::cout << "=== SMART POINTERS AND OWNERSHIP SEMANTICS ===" << std::endl;

    // unique_ptr: exclusive ownership, most efficient
    {
        std::unique_ptr<Widget> uniqueWidget = std::make_unique<Widget>();
        uniqueWidget->doSomething();

        // Transfer ownership
        std::unique_ptr<Widget> anotherWidget = std::move(uniqueWidget);
        anotherWidget->doSomething();
        // uniqueWidget is now nullptr
    }

    // shared_ptr: shared ownership with reference counting
    {
        std::shared_ptr<Widget> sharedWidget1 = std::make_shared<Widget>();
        std::cout << "Reference count: " << sharedWidget1.use_count() << std::endl;

        {
            std::shared_ptr<Widget> sharedWidget2 = sharedWidget1;
            std::cout << "Reference count: " << sharedWidget1.use_count() << std::endl;
            sharedWidget2->doSomething();
        }

        std::cout << "Reference count after scope: " << sharedWidget1.use_count() << std::endl;
    }

    // weak_ptr: non-owning reference to avoid circular dependencies
    {
        std::shared_ptr<Widget> sharedWidget = std::make_shared<Widget>();
        std::weak_ptr<Widget> weakWidget = sharedWidget;

        std::cout << "Weak pointer expired? " << weakWidget.expired() << std::endl;

        if (auto locked = weakWidget.lock()) {
            locked->doSomething();
            std::cout << "Successfully locked weak pointer" << std::endl;
        }
    }

    std::cout << std::endl;
}

// ================================================================================
// 3. MOVE SEMANTICS AND PERFECT FORWARDING
// ================================================================================

class MovableClass {
private:
    std::unique_ptr<int[]> data;
    size_t size;

public:
    // Constructor
    explicit MovableClass(size_t n) : size(n), data(std::make_unique<int[]>(n)) {
        std::cout << "Constructed with size " << size << std::endl;
    }

    // Copy constructor
    MovableClass(const MovableClass& other) : size(other.size),
                                            data(std::make_unique<int[]>(other.size)) {
        std::copy(other.data.get(), other.data.get() + size, data.get());
        std::cout << "Copy constructed" << std::endl;
    }

    // Move constructor
    MovableClass(MovableClass&& other) noexcept : data(std::move(other.data)), size(other.size) {
        other.size = 0;
        std::cout << "Move constructed" << std::endl;
    }

    // Copy assignment
    MovableClass& operator=(const MovableClass& other) {
        if (this != &other) {
            data = std::make_unique<int[]>(other.size);
            size = other.size;
            std::copy(other.data.get(), other.data.get() + size, data.get());
            std::cout << "Copy assigned" << std::endl;
        }
        return *this;
    }

    // Move assignment
    MovableClass& operator=(MovableClass&& other) noexcept {
        if (this != &other) {
            data = std::move(other.data);
            size = other.size;
            other.size = 0;
            std::cout << "Move assigned" << std::endl;
        }
        return *this;
    }

    size_t getSize() const { return size; }
};

// Perfect forwarding example
template<typename T>
std::unique_ptr<T> make_unique_wrapper(T&& arg) {
    return std::make_unique<T>(std::forward<T>(arg));
}

void demonstrate_move_semantics() {
    std::cout << "=== MOVE SEMANTICS AND PERFECT FORWARDING ===" << std::endl;

    MovableClass obj1(1000);
    MovableClass obj2 = obj1;           // Copy
    MovableClass obj3 = std::move(obj1); // Move (obj1 is now empty)

    // Perfect forwarding with std::forward
    auto widget = make_unique_wrapper(Widget());

    std::cout << std::endl;
}

// ================================================================================
// 4. RANGE-BASED FOR LOOPS AND ITERATORS
// ================================================================================

void demonstrate_range_based_loops() {
    std::cout << "=== RANGE-BASED FOR LOOPS AND ITERATORS ===" << std::endl;

    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    // Read-only access
    std::cout << "Read-only: ";
    for (const int& num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // Modify elements
    std::cout << "Modified: ";
    for (int& num : numbers) {
        num *= 2;
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // Using iterators directly
    std::cout << "Using iterators: ";
    for (auto it = numbers.begin(); it != numbers.end(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << std::endl;

    // Reverse iteration
    std::cout << "Reverse iteration: ";
    for (auto it = numbers.rbegin(); it != numbers.rend(); ++it) {
        std::cout << *it << " ";
    }
    std::cout << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 5. STL ALGORITHMS AND FUNCTIONAL PROGRAMMING
// ================================================================================

void demonstrate_stl_algorithms() {
    std::cout << "=== STL ALGORITHMS AND FUNCTIONAL PROGRAMMING ===" << std::endl;

    std::vector<int> numbers = {5, 2, 8, 1, 9, 3, 7, 4, 6};

    // Sorting
    std::vector<int> sorted = numbers;
    std::sort(sorted.begin(), sorted.end());
    std::cout << "Sorted: ";
    for (int num : sorted) std::cout << num << " ";
    std::cout << std::endl;

    // Finding
    auto it = std::find(numbers.begin(), numbers.end(), 8);
    if (it != numbers.end()) {
        std::cout << "Found 8 at position: " << std::distance(numbers.begin(), it) << std::endl;
    }

    // Counting
    int count_evens = std::count_if(numbers.begin(), numbers.end(),
                                  [](int x) { return x % 2 == 0; });
    std::cout << "Number of even elements: " << count_evens << std::endl;

    // Transform
    std::vector<int> squared;
    std::transform(numbers.begin(), numbers.end(), std::back_inserter(squared),
                   [](int x) { return x * x; });
    std::cout << "Squared: ";
    for (int num : squared) std::cout << num << " ";
    std::cout << std::endl;

    // Accumulate
    int sum = std::accumulate(numbers.begin(), numbers.end(), 0);
    std::cout << "Sum: " << sum << std::endl;

    // Min/Max
    auto [min_it, max_it] = std::minmax_element(numbers.begin(), numbers.end());
    std::cout << "Min: " << *min_it << ", Max: " << *max_it << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 6. LAMBDA EXPRESSIONS AND FUNCTIONAL OBJECTS
// ================================================================================

void demonstrate_lambdas() {
    std::cout << "=== LAMBDA EXPRESSIONS AND FUNCTIONAL OBJECTS ===" << std::endl;

    // Basic lambda
    auto add = [](int a, int b) { return a + b; };
    std::cout << "Lambda add(3, 4): " << add(3, 4) << std::endl;

    // Lambda with capture
    int multiplier = 3;
    auto multiply = [multiplier](int x) { return x * multiplier; };
    std::cout << "Lambda multiply(5): " << multiply(5) << std::endl;

    // Lambda with mutable capture
    int counter = 0;
    auto increment = [counter]() mutable { return ++counter; };
    std::cout << "Mutable lambda calls: " << increment() << ", " << increment() << std::endl;
    std::cout << "Original counter unchanged: " << counter << std::endl;

    // Lambda as predicate
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    numbers.erase(std::remove_if(numbers.begin(), numbers.end(),
                                [](int x) { return x % 2 == 0; }),
                  numbers.end());
    std::cout << "After removing evens: ";
    for (int num : numbers) std::cout << num << " ";
    std::cout << std::endl;

    // Generic lambda (C++14)
    auto generic_add = [](auto a, auto b) { return a + b; };
    std::cout << "Generic lambda: " << generic_add(3.5, 2.5) << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 7. TEMPLATES AND TYPE TRAITS
// ================================================================================

// Template with SFINAE (Substitution Failure Is Not An Error)
template<typename T>
typename std::enable_if<std::is_integral<T>::value, T>::type
safe_divide(T a, T b) {
    if (b == 0) throw std::invalid_argument("Division by zero");
    return a / b;
}

template<typename T>
typename std::enable_if<std::is_floating_point<T>::value, T>::type
safe_divide(T a, T b) {
    if (std::abs(b) < std::numeric_limits<T>::epsilon()) {
        throw std::invalid_argument("Division by zero");
    }
    return a / b;
}

// Template with concepts (C++20 concept alternative)
template<typename T>
requires std::is_arithmetic_v<T>
T add_numbers(T a, T b) {
    return a + b;
}

void demonstrate_templates_and_traits() {
    std::cout << "=== TEMPLATES AND TYPE TRAITS ===" << std::endl;

    // Type traits
    std::cout << "Is int integral? " << std::is_integral<int>::value << std::endl;
    std::cout << "Is double integral? " << std::is_integral<double>::value << std::endl;
    std::cout << "Is double floating point? " << std::is_floating_point<double>::value << std::endl;

    // SFINAE in action
    try {
        std::cout << "Safe divide (int): " << safe_divide(10, 2) << std::endl;
        std::cout << "Safe divide (double): " << safe_divide(10.0, 2.0) << std::endl;
    } catch (const std::exception& e) {
        std::cout << "Error: " << e.what() << std::endl;
    }

    // Concepts-style template
    std::cout << "Add numbers: " << add_numbers(3.14, 2.71) << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 8. CONSTEXPR AND COMPILE-TIME COMPUTATION
// ================================================================================

constexpr int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

constexpr bool is_prime(int n) {
    if (n <= 1) return false;
    for (int i = 2; i * i <= n; ++i) {
        if (n % i == 0) return false;
    }
    return true;
}

template<int N>
struct CompileTimeArray {
    constexpr CompileTimeArray() {
        for (int i = 0; i < N; ++i) {
            data[i] = factorial(i);
        }
    }
    int data[N];
};

void demonstrate_constexpr() {
    std::cout << "=== CONSTEXPR AND COMPILE-TIME COMPUTATION ===" << std::endl;

    // Compile-time computation
    constexpr int fact5 = factorial(5);
    std::cout << "Factorial of 5 (compile-time): " << fact5 << std::endl;

    constexpr bool is_17_prime = is_prime(17);
    std::cout << "Is 17 prime (compile-time): " << is_17_prime << std::endl;

    // Compile-time array initialization
    constexpr CompileTimeArray<5> ct_array;
    std::cout << "Compile-time factorials: ";
    for (int i = 0; i < 5; ++i) {
        std::cout << ct_array.data[i] << " ";
    }
    std::cout << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 9. OPTIONAL, VARIANT, AND ANY
// ================================================================================

void demonstrate_optional_variant_any() {
    std::cout << "=== OPTIONAL, VARIANT, AND ANY ===" << std::endl;

    // std::optional
    std::optional<int> maybe_value;
    std::cout << "Optional has value: " << maybe_value.has_value() << std::endl;

    maybe_value = 42;
    if (maybe_value) {
        std::cout << "Optional value: " << *maybe_value << std::endl;
    }

    // std::variant (type-safe union)
    std::variant<int, double, std::string> var;
    var = 42;
    std::cout << "Variant holds int: " << std::get<int>(var) << std::endl;

    var = 3.14;
    std::cout << "Variant holds double: " << std::get<double>(var) << std::endl;

    var = std::string("Hello");
    std::cout << "Variant holds string: " << std::get<std::string>(var) << std::endl;

    // Variant visitor pattern
    auto visitor = [](auto&& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, int>) {
            std::cout << "Int: " << arg << std::endl;
        } else if constexpr (std::is_same_v<T, double>) {
            std::cout << "Double: " << arg << std::endl;
        } else if constexpr (std::is_same_v<T, std::string>) {
            std::cout << "String: " << arg << std::endl;
        }
    };

    std::visit(visitor, var);

    // std::any
    std::any any_value = 42;
    std::cout << "Any holds int: " << std::any_cast<int>(any_value) << std::endl;

    any_value = std::string("Hello");
    std::cout << "Any holds string: " << std::any_cast<std::string>(any_value) << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 10. MODERN C++ I/O AND STRING MANIPULATION
// ================================================================================

void demonstrate_modern_io() {
    std::cout << "=== MODERN C++ I/O AND STRING MANIPULATION ===" << std::endl;

    // String streams
    std::stringstream ss;
    ss << "The answer is " << 42 << " and pi is " << 3.14159;
    std::string result = ss.str();
    std::cout << "String stream result: " << result << std::endl;

    // String formatting with iomanip
    std::cout << std::fixed << std::setprecision(2);
    std::cout << "Formatted number: " << 3.14159 << std::endl;

    std::cout << std::setw(10) << std::left << "Hello:"
              << std::setw(10) << std::right << "World!" << std::endl;

    // String view (C++17) - non-owning reference to string
    std::string_view sv = "Hello, String View!";
    std::cout << "String view: " << sv << std::endl;

    // File I/O with RAII
    {
        std::ofstream outfile("test.txt");
        outfile << "Hello, File I/O!" << std::endl;
    } // File automatically closed

    std::ifstream infile("test.txt");
    std::string line;
    if (std::getline(infile, line)) {
        std::cout << "Read from file: " << line << std::endl;
    }

    std::cout << std::endl;
}

// ================================================================================
// 11. THREADS AND CONCURRENCY (BASIC)
// ================================================================================

#include <thread>
#include <mutex>
#include <atomic>
#include <future>

void demonstrate_concurrency() {
    std::cout << "=== THREADS AND CONCURRENCY ===" << std::endl;

    // Atomic operations
    std::atomic<int> counter(0);

    auto increment = [&counter]() {
        for (int i = 0; i < 1000; ++i) {
            counter++;
        }
    };

    std::thread t1(increment);
    std::thread t2(increment);

    t1.join();
    t2.join();

    std::cout << "Atomic counter: " << counter << std::endl;

    // Future and promise
    std::promise<int> promise;
    std::future<int> future = promise.get_future();

    std::thread producer([&promise]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        promise.set_value(42);
    });

    std::cout << "Waiting for future value..." << std::endl;
    int value = future.get();
    std::cout << "Future value: " << value << std::endl;

    producer.join();

    std::cout << std::endl;
}

// ================================================================================
// 12. CONST-CORRECTNESS AND QUALIFIERS
// ================================================================================

class ConstCorrectness {
private:
    mutable int cache_valid = 0;
    mutable int cached_value = 0;
    int expensive_value;

    int expensive_computation() const {
        std::cout << "Performing expensive computation..." << std::endl;
        return expensive_value * 2;
    }

public:
    explicit ConstCorrectness(int val) : expensive_value(val) {}

    // Const member function - doesn't modify object state
    int getValue() const { return expensive_value; }

    // Const function with mutable cache
    int getCachedValue() const {
        if (!cache_valid) {
            cached_value = expensive_computation();
            cache_valid = 1;
        }
        return cached_value;
    }

    // Non-const version can call const version
    void setValue(int val) {
        expensive_value = val;
        cache_valid = 0;  // Invalidate cache
    }
};

void demonstrate_const_correctness() {
    std::cout << "=== CONST-CORRECTNESS AND QUALIFIERS ===" << std::endl;

    const ConstCorrectness obj(21);

    // Can call const member functions
    std::cout << "Value: " << obj.getValue() << std::endl;
    std::cout << "Cached value: " << obj.getCachedValue() << std::endl;
    std::cout << "Cached value again: " << obj.getCachedValue() << std::endl;

    // const references for efficiency
    std::vector<int> large_vector(1000, 42);
    const auto& cref = large_vector;  // No copy, cannot modify
    std::cout << "First element via const reference: " << cref[0] << std::endl;

    std::cout << std::endl;
}

// ================================================================================
// 13. PIMPL IDIOM (POINTER TO IMPLEMENTATION)
// ================================================================================

// Header file interface
class PimplExample {
private:
    class Impl;
    std::unique_ptr<Impl> pimpl;

public:
    PimplExample();
    ~PimplExample();

    // Copy operations
    PimplExample(const PimplExample& other);
    PimplExample& operator=(const PimplExample& other);

    // Move operations
    PimplExample(PimplExample&& other) noexcept;
    PimplExample& operator=(PimplExample&& other) noexcept;

    void publicMethod();
    int getValue() const;
};

// Implementation file
class PimplExample::Impl {
private:
    int value;
    std::string name;

public:
    Impl() : value(42), name("PimplExample") {}

    void doSomething() {
        std::cout << "Implementation doing something with value: " << value << std::endl;
    }

    int getValue() const { return value; }
    void setValue(int val) { value = val; }
};

PimplExample::PimplExample() : pimpl(std::make_unique<Impl>()) {}
PimplExample::~PimplExample() = default;

PimplExample::PimplExample(const PimplExample& other)
    : pimpl(std::make_unique<Impl>(*other.pimpl)) {}

PimplExample& PimplExample::operator=(const PimplExample& other) {
    if (this != &other) {
        pimpl = std::make_unique<Impl>(*other.pimpl);
    }
    return *this;
}

PimplExample::PimplExample(PimplExample&& other) noexcept = default;
PimplExample& PimplExample::operator=(PimplExample&& other) noexcept = default;

void PimplExample::publicMethod() {
    pimpl->doSomething();
}

int PimplExample::getValue() const {
    return pimpl->getValue();
}

void demonstrate_pimpl() {
    std::cout << "=== PIMPL IDIOM ===" << std::endl;

    PimplExample example;
    example.publicMethod();
    std::cout << "Value: " << example.getValue() << std::endl;

    // Copy constructor
    PimplExample copy = example;
    copy.publicMethod();

    std::cout << std::endl;
}

// ================================================================================
// 14. TYPE ERASURE
// ================================================================================

class Drawable {
public:
    virtual ~Drawable() = default;
    virtual void draw() const = 0;
    virtual std::unique_ptr<Drawable> clone() const = 0;
};

template<typename T>
class DrawableModel : public Drawable {
private:
    T object;

public:
    explicit DrawableModel(T obj) : object(std::move(obj)) {}

    void draw() const override {
        object.draw();
    }

    std::unique_ptr<Drawable> clone() const override {
        return std::make_unique<DrawableModel<T>>(object);
    }
};

class Circle {
private:
    double radius;

public:
    explicit Circle(double r) : radius(r) {}

    void draw() const {
        std::cout << "Drawing circle with radius " << radius << std::endl;
    }
};

class Square {
private:
    double side;

public:
    explicit Square(double s) : side(s) {}

    void draw() const {
        std::cout << "Drawing square with side " << side << std::endl;
    }
};

void demonstrate_type_erasure() {
    std::cout << "=== TYPE ERASURE ===" << std::endl;

    std::vector<std::unique_ptr<Drawable>> shapes;

    shapes.push_back(std::make_unique<DrawableModel<Circle>>(Circle(5.0)));
    shapes.push_back(std::make_unique<DrawableModel<Square>>(Square(3.0)));

    for (const auto& shape : shapes) {
        shape->draw();
    }

    std::cout << std::endl;
}

// ================================================================================
// 15. CRTP (CURIOUSLY RECURSIVE TEMPLATE PATTERN)
// ================================================================================

template<typename Derived>
class Base {
public:
    void interface() {
        static_cast<Derived*>(this)->implementation();
    }

    void baseMethod() {
        std::cout << "Base method called" << std::endl;
    }
};

class Derived1 : public Base<Derived1> {
public:
    void implementation() {
        std::cout << "Derived1 implementation" << std::endl;
        baseMethod();
    }
};

class Derived2 : public Base<Derived2> {
public:
    void implementation() {
        std::cout << "Derived2 implementation" << std::endl;
        baseMethod();
    }
};

void demonstrate_crtp() {
    std::cout << "=== CRTP (Curiously Recurring Template Pattern) ===" << std::endl;

    Derived1 d1;
    Derived2 d2;

    d1.interface();
    d2.interface();

    std::cout << std::endl;
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

int main() {
    std::cout << "╔══════════════════════════════════════════════════════════════╗" << std::endl;
    std::cout << "║           COMPREHENSIVE C++ IDIOMATIC PATTERNS            ║" << std::endl;
    std::cout << "╚══════════════════════════════════════════════════════════════╝" << std::endl;
    std::cout << std::endl;

    try {
        demonstrate_raii();
        demonstrate_smart_pointers();
        demonstrate_move_semantics();
        demonstrate_range_based_loops();
        demonstrate_stl_algorithms();
        demonstrate_lambdas();
        demonstrate_templates_and_traits();
        demonstrate_constexpr();
        demonstrate_optional_variant_any();
        demonstrate_modern_io();
        demonstrate_concurrency();
        demonstrate_const_correctness();
        demonstrate_pimpl();
        demonstrate_type_erasure();
        demonstrate_crtp();

        std::cout << "╔══════════════════════════════════════════════════════════════╗" << std::endl;
        std::cout << "║                    CONCLUSION                           ║" << std::endl;
        std::cout << "╚══════════════════════════════════════════════════════════════╝" << std::endl;
        std::cout << "These patterns represent modern, idiomatic C++ that every" << std::endl;
        std::cout << "experienced C++ developer should know and use regularly." << std::endl;
        std::cout << std::endl;
        std::cout << "Key takeaways:" << std::endl;
        std::cout << "1. Use RAII for automatic resource management" << std::endl;
        std::cout << "2. Prefer smart pointers over raw pointers" << std::endl;
        std::cout << "3. Leverage move semantics for efficiency" << std::endl;
        std::cout << "4. Use STL algorithms instead of manual loops" << std::endl;
        std::cout << "5. Embrace functional programming with lambdas" << std::endl;
        std::cout << "6. Use templates for generic, type-safe code" << std::endl;
        std::cout << "7. Apply constexpr for compile-time optimizations" << std::endl;
        std::cout << "8. Use type-safe alternatives (optional, variant)" << std::endl;
        std::cout << "9. Apply const-correctness rigorously" << std::endl;
        std::cout << "10. Use design patterns for clean architecture" << std::endl;

    }
    catch (const std::exception& e) {
        std::cerr << "Error occurred: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}

/*
================================================================================
IDIOMATIC C++ PATTERNS SUMMARY:
================================================================================

1. RESOURCE MANAGEMENT:
   - RAII: Acquire resources in constructors, release in destructors
   - Smart pointers: unique_ptr, shared_ptr, weak_ptr
   - Move semantics: Efficient resource transfer

2. MODERN C++ FEATURES:
   - Auto type deduction and range-based for loops
   - Lambda expressions for functional programming
   - constexpr for compile-time computation
   - Optional, variant, and any for type-safe alternatives

3. STL PATTERNS:
   - Algorithms over manual loops
   - Iterators for uniform access
   - Function objects and predicates
   - Container choices based on requirements

4. DESIGN PATTERNS:
   - PIMPL: Encapsulation and compilation time reduction
   - Type erasure: Runtime polymorphism without inheritance
   - CRTP: Static polymorphism and compile-time optimization

5. BEST PRACTICES:
   - const-correctness: Mark functions const when possible
   - Move semantics: Use std::move for transfers
   - Perfect forwarding: Preserve value categories
   - SFINAE and concepts: Template constraints

6. CONCURRENCY:
   - Atomic operations for lock-free programming
   - Mutexes for thread synchronization
   - Futures and promises for async operations
   - Thread-safe design patterns

COMPILATION:
g++ -std=c++17 -Wall -Wextra -O2 02-C++_Idiomatic_Patterns.cpp -o patterns_demo -pthread
================================================================================
*/