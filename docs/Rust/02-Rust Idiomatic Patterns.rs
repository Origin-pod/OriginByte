/**
 * @file 02-Rust Idiomatic Patterns.rs
 * @brief Comprehensive guide to idiomatic Rust patterns and best practices
 *
 * This file covers the most important Rust idioms and patterns that experienced
 * Rust developers use. These patterns leverage the language's features to write
 * efficient, maintainable, and modern Rust code.
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{self, Write, Read};
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::{Any, TypeId};
use std::marker::PhantomData;

// ================================================================================
// 1. OWNERSHIP AND RAII (AUTOMATIC RESOURCE MANAGEMENT)
// ================================================================================

struct RaiiExample {
    data: Vec<i32>,
    file: Option<File>,
}

impl RaiiExample {
    fn new(size: usize, filename: &str) -> io::Result<Self> {
        let file = File::create(filename)?;
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);

        println!("Resources acquired: vector of capacity {} and file handle", size);
        Ok(RaiiExample {
            data,
            file: Some(file),
        })
    }

    fn process(&mut self, index: usize, value: i32) -> io::Result<()> {
        if index < self.data.len() {
            self.data[index] = value;
            if let Some(ref mut file) = self.file {
                writeln!(file, "Processed index {} with value {}", index, value)?;
            }
        }
        Ok(())
    }

    fn get_data(&self) -> &[i32] {
        &self.data
    }
}

impl Drop for RaiiExample {
    fn drop(&mut self) {
        println!("Resources automatically released via Drop trait");
        // File is automatically closed when dropped
    }
}

fn demonstrate_raii() {
    println!("=== OWNERSHIP AND RAII (Automatic Resource Management) ===");

    {
        let mut example = RaiiExample::new(100, "output.txt").expect("Failed to create RAII example");
        example.process(0, 42).expect("Failed to process");
        example.process(99, 100).expect("Failed to process");
        println!("First element: {}", example.get_data()[0]);
    } // Resources automatically released here when example goes out of scope

    println!();
}

// ================================================================================
// 2. SMART POINTERS AND OWNERSHIP SEMANTICS
// ================================================================================

struct Widget {
    name: String,
}

impl Widget {
    fn new(name: &str) -> Self {
        println!("Widget '{}' created", name);
        Widget { name: name.to_string() }
    }

    fn do_something(&self) {
        println!("Widget '{}' doing something", self.name);
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        println!("Widget '{}' destroyed", self.name);
    }
}

fn demonstrate_smart_pointers() {
    println!("=== SMART POINTERS AND OWNERSHIP SEMANTICS ===");

    // Box<T>: heap allocation, single ownership
    {
        let boxed_widget = Box::new(Widget::new("boxed"));
        boxed_widget.do_something();
        // Box is automatically dropped when it goes out of scope
    }

    // Rc<T>: reference counting, shared ownership (single-threaded)
    {
        let shared_widget = Rc::new(Widget::new("shared"));
        println!("Reference count: {}", Rc::strong_count(&shared_widget));

        {
            let another_ref = Rc::clone(&shared_widget);
            println!("Reference count: {}", Rc::strong_count(&shared_widget));
            another_ref.do_something();
        }

        println!("Reference count after scope: {}", Rc::strong_count(&shared_widget));
    }

    // Arc<T>: atomic reference counting, shared ownership (multi-threaded)
    {
        let arc_widget = Arc::new(Widget::new("atomic"));
        let arc_clone = Arc::clone(&arc_widget);

        thread::spawn(move || {
            arc_clone.do_something();
        }).join().unwrap();
    }

    // RefCell<T>: interior mutability with runtime borrow checking
    {
        let mutable_widget = RefCell::new(Widget::new("mutable"));
        mutable_widget.borrow().do_something(); // Immutable borrow

        mutable_widget.borrow_mut().do_something(); // Mutable borrow
    }

    println!();
}

// ================================================================================
// 3. MOVE SEMANTICS AND BORROWING
// ================================================================================

#[derive(Debug)]
struct MovableStruct {
    data: Vec<i32>,
    name: String,
}

impl MovableStruct {
    fn new(size: usize, name: &str) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        MovableStruct {
            data,
            name: name.to_string(),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

// Implement Copy and Clone for small structs
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn demonstrate_move_semantics() {
    println!("=== MOVE SEMANTICS AND BORROWING ===");

    // Move semantics
    let obj1 = MovableStruct::new(1000, "original");
    let obj2 = obj1; // obj1 is moved to obj2

    // println!("Original size: {}", obj1.size()); // This would cause compile error
    println!("Moved size: {}", obj2.size());

    // Copy semantics
    let point1 = Point { x: 1, y: 2 };
    let point2 = point1; // point1 is copied
    println!("Original point: {:?}", point1);
    println!("Copied point: {:?}", point2);

    // Borrowing
    let obj3 = MovableStruct::new(500, "borrowed");
    let immutable_ref = &obj3; // Immutable borrow
    println!("Borrowed size: {}", immutable_ref.size());

    let mut obj4 = MovableStruct::new(200, "mutable");
    let mutable_ref = &mut obj4; // Mutable borrow
    println!("Mutable borrow size: {}", mutable_ref.size());

    // Function parameter passing
    let obj5 = MovableStruct::new(300, "function");
    take_ownership(obj5); // obj5 is moved
    // println!("After function call: {}", obj5.size()); // Compile error

    let obj6 = MovableStruct::new(400, "borrow_function");
    borrow_immutable(&obj6); // obj6 is borrowed
    println!("After borrow: {}", obj6.size()); // Still valid

    println!();
}

fn take_ownership(obj: MovableStruct) {
    println!("Took ownership of object with size: {}", obj.size());
}

fn borrow_immutable(obj: &MovableStruct) {
    println!("Borrowed object with size: {}", obj.size());
}

// ================================================================================
// 4. ITERATORS AND FUNCTIONAL PROGRAMMING
// ================================================================================

fn demonstrate_iterators_functional() {
    println!("=== ITERATORS AND FUNCTIONAL PROGRAMMING ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Iterator adaptors
    let doubled: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);

    // Filter
    let evens: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    println!("Even numbers: {:?}", evens);

    // Chain operations
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(3)
        .collect();
    println!("First 3 even squares: {:?}", result);

    // Fold/reduce
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);

    // Custom fold
    let concatenated: String = numbers.iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, x| acc + &x + ",");
    println!("Concatenated: {}", concatenated);

    // Iterator consumption patterns
    println!("Find first > 5: {:?}", numbers.iter().find(|&&x| x > 5));
    println!("All positive: {}", numbers.iter().all(|&x| x > 0));
    println!("Any even: {}", numbers.iter().any(|&x| x % 2 == 0));
    println!("Count evens: {}", numbers.iter().filter(|&&x| x % 2 == 0).count());

    println!();
}

// ================================================================================
// 5. CLOSURES AND FUNCTIONAL PROGRAMMING
// ================================================================================

fn demonstrate_closures() {
    println!("=== CLOSURES AND FUNCTIONAL PROGRAMMING ===");

    // Basic closure
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("Closure add(3, 4): {}", add(3, 4));

    // Closure with environment capture
    let multiplier = 3;
    let multiply = |x: i32| x * multiplier;
    println!("Closure multiply(5): {}", multiply(5));

    // Closure as function parameter
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_operation(&numbers, |x| x * 2);
    println!("Doubled: {:?}", doubled);

    // Closure with move semantics
    let data = vec![1, 2, 3];
    let thread = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("Sum in thread: {}", sum);
    });
    thread.join().unwrap();

    // Closure returning closures
    let make_adder = |x| move |y| x + y;
    let add_5 = make_adder(5);
    println!("Add 5 to 10: {}", add_5(10));

    // Higher-order functions
    let numbers = vec![1, 2, 3, 4, 5];
    let result = compose(|x| x * 2, |x| x + 1)(3);
    println!("Compose result: {}", result);

    println!();
}

fn apply_operation(numbers: &[i32], operation: impl Fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(|&x| operation(x)).collect()
}

fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

// ================================================================================
// 6. ERROR HANDLING WITH RESULT AND OPTION
// ================================================================================

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt_safe(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn demonstrate_error_handling() {
    println!("=== ERROR HANDLING WITH RESULT AND OPTION ===");

    // Result type
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    // Option type
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x > 3);
    println!("Found number > 3: {:?}", found);

    // Chaining operations
    let result = divide(10.0, 2.0)
        .and_then(|x| sqrt_safe(x))
        .map(|x| x * 2.0);
    println!("Chained operation result: {:?}", result);

    // Using ? operator
    match complex_calculation(10.0, 2.0, 16.0) {
        Ok(result) => println!("Complex calculation: {}", result),
        Err(e) => println!("Complex calculation failed: {:?}", e),
    }

    // Error handling with unwrap_or
    let safe_result = divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("Safe result: {}", safe_result);

    println!();
}

fn complex_calculation(a: f64, b: f64, c: f64) -> Result<f64, MathError> {
    let step1 = divide(a, b)?;
    let step2 = sqrt_safe(c)?;
    Ok(step1 * step2)
}

// ================================================================================
// 7. TRAITS AND GENERIC PROGRAMMING
// ================================================================================

trait Summary {
    fn summarize(&self) -> String;

    fn summarize_verbose(&self) -> String {
        format!("(Summary: {})", self.summarize())
    }
}

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

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Generic function with trait bounds
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Generic struct with trait bounds
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is first = {}", self.first);
        } else {
            println!("The largest member is second = {}", self.second);
        }
    }
}

fn demonstrate_traits_generics() {
    println!("=== TRAITS AND GENERIC PROGRAMMING ===");

    let article = Article {
        title: String::from("Rust is Awesome"),
        author: String::from("Jane Doe"),
        content: String::from("Rust provides memory safety..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Learning Rust!"),
    };

    notify(&article);
    notify(&tweet);

    // Generic struct
    let pair_of_ints = Pair::new(5, 10);
    pair_of_ints.cmp_display();

    let pair_of_chars = Pair::new('a', 'z');
    pair_of_chars.cmp_display();

    // Trait objects
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(article),
        Box::new(tweet),
    ];

    for item in &items {
        println!("Item: {}", item.summarize_verbose());
    }

    println!();
}

// ================================================================================
// 8. CONST FUNCTIONS AND COMPILE-TIME COMPUTATION
// ================================================================================

const fn const_factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => n * const_factorial(n - 1),
    }
}

const fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// Generic array with const generics
struct ConstArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> ConstArray<T, N>
where
    T: Default + Copy,
{
    const fn new() -> Self {
        ConstArray {
            data: [T::default(); N],
        }
    }

    fn len(&self) -> usize {
        N
    }
}

fn demonstrate_const_functions() {
    println!("=== CONST FUNCTIONS AND COMPILE-TIME COMPUTATION ===");

    const FACT5: usize = const_factorial(5);
    println!("Factorial of 5 (compile-time): {}", FACT5);

    const IS_17_PRIME: bool = is_prime(17);
    println!("Is 17 prime (compile-time): {}", IS_17_PRIME);

    // Const generics
    let array: ConstArray<i32, 5> = ConstArray::new();
    println!("Array length: {}", array.len());

    println!();
}

// ================================================================================
// 9. PATTERN MATCHING AND DESTRUCTURING
// ================================================================================

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn demonstrate_pattern_matching() {
    println!("=== PATTERN MATCHING AND DESTRUCTURING ===");

    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for message in &messages {
        match message {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }

    // Destructuring structs
    let point = Point3D { x: 1, y: 2, z: 3 };
    let Point3D { x: a, y: b, z: c } = point;
    println!("Destructured: ({}, {}, {})", a, b, c);

    // If let patterns
    let some_option = Some(5);
    if let Some(value) = some_option {
        println!("Got value: {}", value);
    }

    // While let patterns
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    println!();
}

// ================================================================================
// 10. CONCURRENCY AND THREAD SAFETY
// ================================================================================

fn demonstrate_concurrency() {
    println!("=== CONCURRENCY AND THREAD SAFETY ===");

    // Basic thread creation
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(30));
    }

    handle.join().unwrap();

    // Shared state with Arc<Mutex<>>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
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
        for i in 1..5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    println!();
}

// ================================================================================
// 11. SLICES AND VIEWS
// ================================================================================

fn demonstrate_slices_views() {
    println!("=== SLICES AND VIEWS ===");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Different slice views
    let slice = &data[2..8]; // Elements 2 through 7
    println!("Slice: {:?}", slice);

    let slice_from = &data[3..]; // Elements 3 through end
    println!("Slice from: {:?}", slice_from);

    let slice_to = &data[..6]; // Elements 0 through 5
    println!("Slice to: {:?}", slice_to);

    let slice_all = &data[..]; // All elements
    println!("Slice all: {:?}", slice_all);

    // String slices
    let string = String::from("Hello, Rust!");
    let hello = &string[..5];
    let rust = &string[7..11];
    println!("String slices: '{}' and '{}'", hello, rust);

    // Iterating over slices
    for (i, &value) in slice.iter().enumerate() {
        println!("Slice[{}] = {}", i, value);
    }

    println!();
}

// ================================================================================
// 12. TYPE ERASURE WITH TRAIT OBJECTS
// ================================================================================

trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("Drawing triangle with base {} and height {}", self.base, self.height);
    }
}

fn demonstrate_type_erasure() {
    println!("=== TYPE ERASURE WITH TRAIT OBJECTS ===");

    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
        Box::new(Triangle { base: 4.0, height: 6.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }

    println!();
}

// ================================================================================
// 13. MACROS AND METAPROGRAMMING
// ================================================================================

// Simple macro
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

// Macro with parameters
macro_rules! create_vector {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($element);)*
            v
        }
    };
}

// Macro for creating struct
macro_rules! create_struct {
    ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {
        struct $struct_name {
            $($field_name: $field_type),*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                $struct_name {
                    $($field_name),*
                }
            }
        }
    };
}

create_struct!(Person {
    name: String,
    age: u32
});

fn demonstrate_macros() {
    println!("=== MACROS AND METAPROGRAMMING ===");

    say_hello!();

    let numbers = create_vector![1, 2, 3, 4, 5];
    println!("Created vector: {:?}", numbers);

    let strings = create_vector!["hello", "world", "rust"];
    println!("Created string vector: {:?}", strings);

    let person = Person::new(String::from("Alice"), 30);
    println!("Created person: {} ({} years)", person.name, person.age);

    println!();
}

// ================================================================================
// 14. ZERO-COST ABSTRACTIONS AND INLINE ASSEMBLY
// ================================================================================

// Generic function with zero-cost abstraction
fn generic_max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Inlined function for performance
#[inline]
fn always_inline() -> i32 {
    42
}

// Const fn for compile-time evaluation
const fn compile_time_computation(x: i32) -> i32 {
    x * 2 + 1
}

fn demonstrate_zero_cost_abstractions() {
    println!("=== ZERO-COST ABSTRACTIONS ===");

    // Generic functions compile to efficient code
    let max_int = generic_max(10, 20);
    let max_float = generic_max(3.14, 2.71);
    let max_string = generic_max("hello", "world");

    println!("Max int: {}", max_int);
    println!("Max float: {}", max_float);
    println!("Max string: {}", max_string);

    // Compile-time computation
    const COMPUTED: i32 = compile_time_computation(21);
    println!("Compile-time computation: {}", COMPUTED);

    // Inline function
    println!("Inline function result: {}", always_inline());

    println!();
}

// ================================================================================
// 15. ADVANCED PATTERNS: NEWTYPE PATTERN AND SMART CONSTRUCTORS
// ================================================================================

// Newtype pattern for type safety
struct UserId(i32);
struct UserName(String);

impl UserId {
    fn new(id: i32) -> Result<Self, &'static str> {
        if id > 0 {
            Ok(UserId(id))
        } else {
            Err("User ID must be positive")
        }
    }

    fn value(&self) -> i32 {
        self.0
    }
}

impl UserName {
    fn new(name: &str) -> Result<Self, &'static str> {
        if !name.is_empty() && name.len() <= 50 {
            Ok(UserName(name.to_string()))
        } else {
            Err("User name must be non-empty and <= 50 characters")
        }
    }

    fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
struct User {
    id: UserId,
    name: UserName,
}

impl User {
    fn new(id: i32, name: &str) -> Result<Self, &'static str> {
        Ok(User {
            id: UserId::new(id)?,
            name: UserName::new(name)?,
        })
    }
}

fn demonstrate_newtype_pattern() {
    println!("=== NEWTYPE PATTERN AND SMART CONSTRUCTORS ===");

    match User::new(1, "Alice") {
        Ok(user) => println!("Created user: ID={}, Name={}",
                           user.id.value(), user.name.value()),
        Err(e) => println!("Failed to create user: {}", e),
    }

    match User::new(-1, "") {
        Ok(user) => println!("Created user: ID={}, Name={}",
                           user.id.value(), user.name.value()),
        Err(e) => println!("Failed to create user: {}", e),
    }

    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE RUST IDIOMATIC PATTERNS           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_raii();
    demonstrate_smart_pointers();
    demonstrate_move_semantics();
    demonstrate_iterators_functional();
    demonstrate_closures();
    demonstrate_error_handling();
    demonstrate_traits_generics();
    demonstrate_const_functions();
    demonstrate_pattern_matching();
    demonstrate_concurrency();
    demonstrate_slices_views();
    demonstrate_type_erasure();
    demonstrate_macros();
    demonstrate_zero_cost_abstractions();
    demonstrate_newtype_pattern();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("These patterns represent modern, idiomatic Rust that every");
    println!("experienced Rust developer should know and use regularly.");
    println!();
    println!("Key takeaways:");
    println!("1. Ownership system prevents common memory bugs");
    println!("2. Use smart pointers for appropriate ownership patterns");
    println!("3. Leverage iterators and functional programming");
    println!("4. Embrace Result and Option for error handling");
    println!("5. Use traits for generic, type-safe code");
    println!("6. Apply pattern matching for expressive control flow");
    println!("7. Use concurrent primitives for thread-safe code");
    println!("8. Leverage macros for metaprogramming");
    println!("9. Apply newtype pattern for type safety");
    println!("10. Trust the compiler for zero-cost abstractions");
    println!();
    println!("Rust advantages:");
    println!("✓ Memory safety without garbage collection");
    println!("✓ Concurrency without data races");
    println!("✓ Zero-cost abstractions");
    println!("✓ Rich type system");
    println!("✓ Excellent error handling");
    println!("✓ Powerful macro system");
}

/*
================================================================================
IDIOMATIC RUST PATTERNS SUMMARY:
================================================================================

1. OWNERSHIP AND MEMORY MANAGEMENT:
   - RAII through ownership system and Drop trait
   - Smart pointers: Box, Rc, Arc, RefCell
   - Move semantics and borrowing
   - Automatic memory management without garbage collection

2. FUNCTIONAL PROGRAMMING:
   - Iterators and iterator adaptors
   - Closures with capture semantics
   - Higher-order functions
   - Chain operations for data processing

3. ERROR HANDLING:
   - Result<T, E> for recoverable errors
   - Option<T> for nullable values
   - ? operator for error propagation
   - Compile-time guarantees against null pointer exceptions

4. TYPE SYSTEM:
   - Traits for shared behavior
   - Generic programming with trait bounds
   - Trait objects for runtime polymorphism
   - Zero-cost abstractions

5. PATTERN MATCHING:
   - Exhaustive matching with match expressions
   - Destructuring of structs and enums
   - if let and while let patterns
   - Guard clauses for complex conditions

6. CONCURRENCY:
   - Fearless concurrency with ownership
   - Arc<Mutex<T>> for shared state
   - Channels for message passing
   - Thread-safe data structures

7. METAPROGRAMMING:
   - Declarative macros with macro_rules!
   - Procedural macros for advanced code generation
   - Compile-time function evaluation
   - Code generation and boilerplate reduction

8. DESIGN PATTERNS:
   - Newtype pattern for type safety
   - Smart constructors for validation
   - Type erasure with trait objects
   - Zero-cost abstractions

9. BEST PRACTICES:
   - Prefer iterators over loops
   - Use Result/Option instead of exceptions/nulls
   - Leverage the borrow checker
   - Write idiomatic, expressive code

10. ECOSYSTEM:
    - Cargo for build management
    - Crates.io for package distribution
    - Excellent tooling (rustfmt, clippy)
    - Rich standard library

COMPILATION:
rustc 02-Rust_Idiomatic_Patterns.rs -o patterns_demo
or
cargo build --release
================================================================================
*/