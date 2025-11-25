/**
 * @file 01-Rust Basic Concepts.rs
 * @brief Comprehensive guide to fundamental Rust concepts
 *
 * This file covers all the essential Rust concepts that every developer should know,
 * from basic syntax to advanced features. It's organized in a logical learning progression
 * with practical examples and best practices.
 */

use std::collections::HashMap;
use std::mem;

// ================================================================================
// 1. BASIC SYNTAX AND DATA TYPES
// ================================================================================

fn demonstrate_basic_syntax() {
    println!("=== BASIC SYNTAX AND DATA TYPES ===");

    // Primitive types
    let integer_var: i32 = 42;                    // 32-bit signed integer
    let short_var: i16 = 10;                      // 16-bit signed integer
    let long_var: i64 = 100000;                   // 64-bit signed integer
    let long_long_var: i128 = 123456789;          // 128-bit signed integer

    let float_var: f32 = 3.14;                    // 32-bit floating point
    let double_var: f64 = 3.14159265;             // 64-bit floating point

    let char_var: char = 'A';                     // Unicode scalar value
    let bool_var: bool = true;                    // Boolean value

    // Type inference
    let auto_int = 42;                            // i32 by default
    let auto_double = 3.14;                       // f64 by default

    // Unsigned integers
    let unsigned_var: u32 = 100;                  // Only non-negative values

    // Constants
    const CONSTANT_VAR: i32 = 42;                 // Compile-time constant (identified by "const")

    // Shadowing: re-declaring a variable with the same name in a narrower scope,
    // effectively hiding the previous declaration and creating a new one.
    //
    // Mutable variable can be reassigned by assigning a new value to it.
    // Non-mutable variable cannot be reassigned, but it can be shadowed by a new
    // non-mutable variable declaration or a mutable variable declaration.
    let mut mutable_var = 5;                      // Mutable variable can be reassigned
    let non_mutable_var = 5;                      // Non-mutable variable cannot be reassigned

    // Reassigning a mutable variable
    mutable_var = mutable_var + 1;                // Mutable variable can be reassigned by assigning a new value

    // Shadowing a non-mutable variable
    let non_mutable_var = 10;                     // Non-mutable variable cannot be reassigned, but it can be shadowed

    // Shadowing a mutable variable
    let mut mutable_var = 20;                    // Mutable variable can be shadowed by a new declaration in the same scope

    println!("Integer: {}", integer_var);
    println!("Float: {}", float_var);
    println!("Char: {}", char_var);
    println!("Bool: {}", bool_var);
    println!();
}

// ================================================================================
// 2. OPERATORS
// ================================================================================

fn demonstrate_operators() {
    println!("=== OPERATORS ===");

    let a = 10;
    let b = 3;

    // Arithmetic operators
    println!("Arithmetic: {} + {} = {}", a, b, a + b);
    println!("Arithmetic: {} - {} = {}", a, b, a - b);
    println!("Arithmetic: {} * {} = {}", a, b, a * b);
    println!("Arithmetic: {} / {} = {}", a, b, a / b);
    println!("Arithmetic: {} % {} = {}", a, b, a % b);

    // Comparison operators
    println!("Comparison: {} > {} is {}", a, b, a > b);
    println!("Comparison: {} == {} is {}", a, b, a == b);
    println!("Comparison: {} != {} is {}", a, b, a != b);

    // Logical operators
    let x = true;
    let y = false;
    println!("Logical: {} && {} = {}", x, y, x && y);
    println!("Logical: {} || {} = {}", x, y, x || y);
    println!("Logical: !{} = {}", x, !x);

    // Bitwise operators
    let m = 0b1010;
    let n = 0b1100;
    println!("Bitwise AND: {:b} & {:b} = {:b}", m, n, m & n);
    println!("Bitwise OR: {:b} | {:b} = {:b}", m, n, m | n);
    println!("Bitwise XOR: {:b} ^ {:b} = {:b}", m, n, m ^ n);

    // Assignment operators (compound assignment)
    let mut d = 10;
    d += 5;  // d = d + 5
    d *= 2;  // d = d * 2
    println!("Assignment: result is {}", d);
    println!();
}

// ================================================================================
// 3. CONTROL FLOW
// ================================================================================

fn demonstrate_control_flow() {
    println!("=== CONTROL FLOW ===");

    // If expressions
    let age = 25;
    let status = if age < 18 {
        "Minor"
    } else if age < 65 {
        "Adult"
    } else {
        "Senior"
    };
    println!("Status: {}", status);

    // Match expressions (pattern matching)
    let grade = 'B';
    match grade {
        'A' => println!("Excellent!"),
        'B' => println!("Good!"),
        'C' => println!("Average"),
        _ => println!("Need improvement"),
    }

    // Loops
    println!("For loop: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    println!("While loop: ");
    let mut j = 0;
    while j < 5 {
        print!("{} ", j);
        j += 1;
    }
    println!();

    // Loop with break/continue
    println!("Break example: ");
    for i in 0..10 {
        if i == 5 { break; }
        print!("{} ", i);
    }
    println!();

    println!("Continue example: ");
    for i in 0..10 {
        if i % 2 == 0 { continue; }
        print!("{} ", i);
    }
    println!();

    // Loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // Iterators
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Iterator: ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();

    println!();
}

// ================================================================================
// 4. FUNCTIONS
// ================================================================================

// Function definition
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with multiple return values
fn calculate_operations(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}

// Function with generic types
fn maximum<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Function with parameter pattern matching
fn greet(name: &str, title: &str) -> String {
    format!("Hello, {} {}!", title, name)
}

// Function with default parameter (using struct)
struct GreetConfig {
    name: String,
    title: String,
}

impl Default for GreetConfig {
    fn default() -> Self {
        GreetConfig {
            name: String::new(),
            title: "Mr./Ms.".to_string(),
        }
    }
}

fn greet_with_config(config: GreetConfig) -> String {
    format!("Hello, {} {}!", config.title, config.name)
}

// Closures (anonymous functions)
fn demonstrate_functions() {
    println!("=== FUNCTIONS ===");

    // Basic function call
    println!("add(5, 3) = {}", add(5, 3));

    // Multiple return values
    let (sum, diff, product) = calculate_operations(10, 5);
    println!("Operations: sum={}, diff={}, product={}", sum, diff, product);

    // Generic function
    println!("maximum(10, 20) = {}", maximum(10, 20));
    println!("maximum(3.14, 2.71) = {}", maximum(3.14, 2.71));

    // Function pointers
    let operation: fn(i32, i32) -> i32 = add;
    println!("Function pointer: {}", operation(7, 8));

    // Closures
    let add_closure = |a: i32, b: i32| a + b;
    println!("Closure add(7, 8) = {}", add_closure(7, 8));

    // Closure with environment capture
    let multiplier = 3;
    let multiply_by = |x: i32| x * multiplier;
    println!("Closure multiply_by(4) = {}", multiply_by(4));

    // Higher-order functions
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    println!();
}

// ================================================================================
// 5. ARRAYS AND STRINGS
// ================================================================================

fn demonstrate_arrays_and_strings() {
    println!("=== ARRAYS AND STRINGS ===");

    // Fixed-size arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("First element: {}", array[0]);
    println!("Array length: {}", array.len());

    // Slices
    let slice = &array[1..4];
    println!("Slice: {:?}", slice);

    // Vectors (dynamic arrays)
    let mut vec = vec![10, 20, 30, 40, 50];
    vec.push(60);  // Add element
    println!("Vector: {:?}", vec);
    println!("Vector size: {}", vec.len());

    // String types
    let string_literal = "Hello, World!";  // &'static str
    let string_slice = "Hello";             // &str
    let owned_string = String::from("World");
    let another_string = " World".to_string();

    let concatenated = format!("{}{}", string_slice, another_string);
    println!("String: {}", concatenated);
    println!("String length: {}", concatenated.len());
    println!("First character: {}", concatenated.chars().next().unwrap());
    println!("Substring: {}", &concatenated[0..5]);

    // String methods
    let text = "Hello, Rust Programming!";
    println!("Original: {}", text);
    println!("Contains 'Rust': {}", text.contains("Rust"));
    println!("Replace: {}", text.replace("Rust", "Awesome"));

    // Character iteration
    println!("Characters: ");
    for ch in concatenated.chars() {
        print!("{} ", ch);
    }
    println!();

    println!();
}

// ================================================================================
// 6. OWNERSHIP, BORROWING, AND REFERENCES
// ================================================================================

fn demonstrate_ownership_borrowing() {
    println!("=== OWNERSHIP, BORROWING, AND REFERENCES ===");

    // Ownership transfer
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // This would cause compile error
    println!("s2: {}", s2);

    // Borrowing - immutable references
    let s3 = String::from("world");
    let len = calculate_length(&s3);  // Pass immutable reference
    println!("Length of '{}' is {}", s3, len);  // s3 is still valid

    // Borrowing - mutable references
    let mut s4 = String::from("hello");
    change_string(&mut s4);
    println!("After change: {}", s4);

    // Multiple immutable references
    let s5 = String::from("multiple");
    let r1 = &s5;
    let r2 = &s5;
    println!("r1: {}, r2: {}", r1, r2);

    // Dangling references are prevented at compile time
    let reference_to_nothing = dangle();
    println!("Reference: {}", reference_to_nothing);

    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("dangle");
    s  // Return ownership, not reference
}

// ================================================================================
// 7. STRUCTS AND ENUMS
// ================================================================================

#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (static method)
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // Method
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method taking mutable reference
    fn set_dimensions(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    // Associated function (no &self)
    fn square(size: f64) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Enums with data
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum with methods
#[derive(Debug, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending(String),
}

impl Status {
    fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }
}

// Option type (null handling)
fn demonstrate_structs_enums() {
    println!("=== STRUCTS AND ENUMS ===");

    // Create and use structs
    let mut rect1 = Rectangle::new(5.0, 3.0);
    let rect2 = Rectangle::square(4.0);

    println!("Rectangle 1: {:?}", rect1);
    println!("Rectangle 1 area: {}", rect1.area());
    println!("Rectangle 2 area: {}", rect2.area());

    rect1.set_dimensions(6.0, 4.0);
    println!("Rectangle 1 after resize: {:?}", rect1);

    // Enums
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);

    // Enum methods
    let status = Status::Active;
    let pending = Status::Pending(String::from("Processing"));

    println!("Status is active: {}", status.is_active());
    println!("Status: {:?}", pending);

    // Pattern matching with enums
    match status {
        Status::Active => println!("User is active"),
        Status::Inactive => println!("User is inactive"),
        Status::Pending(message) => println!("User is pending: {}", message),
    }

    // Option type for null safety
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    // Working with Option
    if let Some(number) = some_number {
        println!("The number is: {}", number);
    }

    println!();
}

// ================================================================================
// 8. ERROR HANDLING
// ================================================================================

use std::fs::File;
use std::io::{self, Read};

fn demonstrate_error_handling() {
    println!("=== ERROR HANDLING ===");

    // Result type
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Using ? operator for error propagation
    match read_file_contents() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using unwrap and expect (caution in production code)
    let safe_result = divide(10.0, 2.0).unwrap_or(f64::INFINITY);
    println!("Safe result: {}", safe_result);

    // Panic on unrecoverable errors
    // panic!("This is a panic!");

    println!();
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn read_file_contents() -> Result<String, io::Error> {
    let mut f = File::open("test.txt")?;  // ? returns error if File::open fails
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;     // ? returns error if read fails
    Ok(contents)
}

// ================================================================================
// 9. TRAITS
// ================================================================================

// Define a trait
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summarize_verbose(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

// Implement trait for a struct
#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

// Generic function with trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait objects
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

fn demonstrate_traits() {
    println!("=== TRAITS ===");

    let article = Article {
        title: String::from("Rust is Awesome"),
        author: String::from("Jane Doe"),
        content: String::from("Rust provides memory safety without garbage collection."),
    };

    // Use trait methods
    println!("Article summary: {}", article.summarize());
    println!("Verbose summary: {}", article.summarize_verbose());

    // Generic function with trait bounds
    notify(&article);

    // Trait objects
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }

    // Common built-in traits
    let article2 = article.clone();  // Clone trait
    println!("Cloned article: {:?}", article2);

    println!();
}

// ================================================================================
// 10. COLLECTIONS
// ================================================================================

fn demonstrate_collections() {
    println!("=== COLLECTIONS ===");

    // Vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("Vector: {:?}", vec);
    println!("Third element: {}", vec[2]);

    // Strings
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("Rust");

    println!("String: {}", s);

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores: {:?}", scores);

    // Access values
    if let Some(score) = scores.get("Blue") {
        println!("Blue team score: {}", score);
    }

    // Iterate over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Entry API
    scores.entry(String::from("Yellow")).or_insert(25);
    println!("Updated scores: {:?}", scores);

    println!();
}

// ================================================================================
// 11. LIFETIMES
// ================================================================================

fn demonstrate_lifetimes() {
    println!("=== LIFETIMES ===");

    let string1 = String::from("long string");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is {}", result);

    // Struct with lifetimes
    let important_excerpt = ImportantExcerpt {
        part: &string1[..5],
    };

    println!("Important excerpt: {}", important_excerpt.announce_and_return_part());

    println!();
}

// Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self) -> &str {
        println!("Please read this important part!");
        self.part
    }
}

// ================================================================================
// 12. SMART POINTERS
// ================================================================================

use std::rc::Rc;
use std::cell::RefCell;

struct SmartPointerDemo {
    data: String,
}

impl SmartPointerDemo {
    fn new(data: &str) -> Self {
        SmartPointerDemo {
            data: data.to_string(),
        }
    }

    fn say_hello(&self) {
        println!("Hello from SmartPointerDemo: {}", self.data);
    }
}

impl Drop for SmartPointerDemo {
    fn drop(&mut self) {
        println!("SmartPointerDemo with data '{}' is being dropped", self.data);
    }
}

fn demonstrate_smart_pointers() {
    println!("=== SMART POINTERS ===");

    // Box<T> - heap allocation
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }  // b is automatically freed here

    // Rc<T> - reference counting
    {
        let a = Rc::new(SmartPointerDemo::new("shared"));
        a.say_hello();

        let b = Rc::clone(&a);
        let c = Rc::clone(&a);

        println!("Reference count: {}", Rc::strong_count(&a));

        b.say_hello();
        c.say_hello();
    }

    // RefCell<T> - interior mutability
    {
        let value = RefCell::new(RefCell::new(42));
        *value.borrow_mut() = 100;  // Mutable borrow
        println!("RefCell value: {}", *value.borrow());
    }

    println!();
}

// ================================================================================
// 13. CONCURRENCY
// ================================================================================

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn demonstrate_concurrency() {
    println!("=== CONCURRENCY ===");

    // Basic thread creation
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap();

    // Shared state with Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());

    // Channels for communication
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    println!();
}

// ================================================================================
// 14. MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE RUST BASIC CONCEPTS GUIDE           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_basic_syntax();
    demonstrate_operators();
    demonstrate_control_flow();
    demonstrate_functions();
    demonstrate_arrays_and_strings();
    demonstrate_ownership_borrowing();
    demonstrate_structs_enums();
    demonstrate_error_handling();
    demonstrate_traits();
    demonstrate_collections();
    demonstrate_lifetimes();
    demonstrate_smart_pointers();
    demonstrate_concurrency();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This guide covers the fundamental Rust concepts that form");
    println!("the foundation for advanced Rust programming.");
    println!();
    println!("Next steps:");
    println!("1. Practice these concepts with small projects");
    println!("2. Study Rust idiomatic patterns (see next file)");
    println!("3. Explore the ecosystem (crates.io)");
    println!("4. Learn advanced topics (async, macros, FFI, etc.)");
    println!();
    println!("Key Rust advantages:");
    println!("✓ Memory safety without garbage collection");
    println!("✓ Concurrency without data races");
    println!("✓ Rich type system with pattern matching");
    println!("✓ Zero-cost abstractions");
    println!("✓ Excellent tooling and package management");
}

/*
================================================================================
KEY TAKEAWAYS FROM THIS GUIDE:
================================================================================

1. OWNERSHIP SYSTEM:
   - Each value has an owner
   - Only one owner at a time
   - Owner is dropped when out of scope
   - Prevents common memory bugs

2. MEMORY SAFETY:
   - Borrow checker prevents dangling pointers
   - No null references (Option<T> instead)
   - No data races in concurrent code
   - RAII through ownership system

3. TYPE SYSTEM:
   - Strong static typing with inference
   - Expressive enums and pattern matching
   - Traits for shared behavior
   - Generics for code reuse

4. ERROR HANDLING:
   - Result<T, E> for recoverable errors
   - Option<T> for nullable values
   - Panic for unrecoverable errors
   - No exceptions, compile-time guarantees

5. CONCURRENCY:
   - Fearless concurrency with ownership
   - Message passing with channels
   - Shared state with Arc<Mutex<T>>
   - No data races at compile time

6. MODERN FEATURES:
   - Pattern matching
   - Closures and iterators
   - Smart pointers
   - Macro system
   - Excellent tooling (Cargo)

7. COMPILATION:
   rustc 01-Rust_Basic_Concepts.rs -o rust_demo
   or
   cargo build

8. LEARNING RESOURCES:
   - The Rust Book (https://doc.rust-lang.org/book/)
   - Rust by Example (https://doc.rust-lang.org/rust-by-example/)
   - Rustlings (https://github.com/rust-lang/rustlings/)
================================================================================
*/