/**
 * @file 03-Stacks.rs
 * @brief Comprehensive Stack Data Structures Implementation in Rust
 *
 * This file covers various stack implementations including array-based stacks,
 * linked list-based stacks, and specialized stack applications with proper
 * memory management and performance considerations.
 */

use std::collections::VecDeque;

// ================================================================================
// 1. ARRAY-BASED STACK (VECTOR IMPLEMENTATION)
// ================================================================================

#[derive(Debug, Clone)]
struct ArrayStack<T> {
    data: Vec<T>,
    capacity: Option<usize>,
}

impl<T> ArrayStack<T> {
    fn new() -> Self {
        ArrayStack {
            data: Vec::new(),
            capacity: None,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        ArrayStack {
            data: Vec::with_capacity(capacity),
            capacity: Some(capacity),
        }
    }

    fn push(&mut self, item: T) -> bool {
        if let Some(cap) = self.capacity {
            if self.data.len() >= cap {
                return false; // Stack is full
            }
        }
        self.data.push(item);
        true
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for ArrayStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

fn demonstrate_array_stack() {
    println!("=== ARRAY-BASED STACK ===");

    let mut stack = ArrayStack::new();
    println!("Created empty stack: {}", stack);
    println!("Is empty: {}", stack.is_empty());

    // Push elements
    println!("Pushing elements:");
    for i in 1..=5 {
        let pushed = stack.push(i);
        println!("  Pushed {}: {} (stack: {})", i, pushed, stack);
    }

    // Peek at top element
    println!("Top element: {:?}", stack.peek());

    // Pop elements
    println!("Popping elements:");
    while let Some(item) = stack.pop() {
        println!("  Popped: {} (stack: {})", item, stack);
    }

    // Stack with capacity
    let mut bounded_stack = ArrayStack::with_capacity(3);
    println!("\nBounded stack (capacity 3):");
    for i in 1..=5 {
        let pushed = bounded_stack.push(i);
        println!("  Pushed {}: {}", i, pushed);
    }

    println!("Bounded stack: {}", bounded_stack);
    println!();
}

// ================================================================================
// 2. LINKED LIST-BASED STACK
// ================================================================================

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

#[derive(Debug)]
struct LinkedStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedStack<T> {
    fn new() -> Self {
        LinkedStack {
            head: None,
            size: 0,
        }
    }

    fn push(&mut self, item: T) {
        let new_node = Box::new(Node::new(item));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.size -= 1;
            let node = *node;
            node.data
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    fn iter(&self) -> LinkedStackIterator<T> {
        LinkedStackIterator {
            current: &self.head,
        }
    }
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct LinkedStackIterator<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedStackIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(&node.data)
            }
            None => None,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for LinkedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}

fn demonstrate_linked_stack() {
    println!("=== LINKED LIST-BASED STACK ===");

    let mut stack = LinkedStack::new();
    println!("Created empty stack: {}", stack);

    // Push elements
    println!("Pushing elements:");
    for ch in ['A', 'B', 'C', 'D', 'E'] {
        stack.push(ch);
        println!("  Pushed '{}' (stack: {})", ch, stack);
    }

    // Peek at top element
    println!("Top element: {:?}", stack.peek());

    // Iterator demonstration
    println!("Iterating over stack:");
    for item in stack.iter() {
        println!("  {:?}", item);
    }

    // Pop elements
    println!("Popping elements:");
    while let Some(item) = stack.pop() {
        println!("  Popped '{}' (stack: {})", item, stack);
    }

    println!("Stack empty: {}", stack.is_empty());
    println!();
}

// ================================================================================
// 3. MIN STACK (STACK WITH MINIMUM OPERATIONS)
// ================================================================================

#[derive(Debug)]
struct MinStack<T: Ord + Clone> {
    data: Vec<T>,
    mins: Vec<T>,
}

impl<T: Ord + Clone> MinStack<T> {
    fn new() -> Self {
        MinStack {
            data: Vec::new(),
            mins: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item.clone());
        if self.mins.is_empty() || item <= *self.mins.last().unwrap() {
            self.mins.push(item);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.data.pop() {
            if Some(&item) == self.mins.last() {
                self.mins.pop();
            }
            Some(item)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn min(&self) -> Option<&T> {
        self.mins.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Ord + Clone + std::fmt::Debug> std::fmt::Display for MinStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "] (min: {:?})", self.min())
    }
}

fn demonstrate_min_stack() {
    println!("=== MIN STACK ===");

    let mut stack = MinStack::new();

    let values = [5, 3, 7, 1, 4, 1, 8];
    println!("Pushing values: {:?}", values);

    for &val in &values {
        stack.push(val);
        println!("Pushed {}, stack: {}, min: {:?}", val, stack, stack.min());
    }

    println!("\nPopping elements:");
    while let Some(item) = stack.pop() {
        println!("Popped {}, stack: {}, min: {:?}", item, stack, stack.min());
    }
    println!();
}

// ================================================================================
// 4. STACK WITH MAXIMUM OPERATIONS
// ================================================================================

#[derive(Debug)]
struct MaxStack<T: Ord + Clone> {
    data: Vec<T>,
    maxs: Vec<T>,
}

impl<T: Ord + Clone> MaxStack<T> {
    fn new() -> Self {
        MaxStack {
            data: Vec::new(),
            maxs: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item.clone());
        if self.maxs.is_empty() || item >= *self.maxs.last().unwrap() {
            self.maxs.push(item);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.data.pop() {
            if Some(&item) == self.maxs.last() {
                self.maxs.pop();
            }
            Some(item)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn max(&self) -> Option<&T> {
        self.maxs.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn demonstrate_max_stack() {
    println!("=== MAX STACK ===");

    let mut stack = MaxStack::new();

    let values = [5, 3, 7, 1, 4, 7, 8];
    println!("Pushing values: {:?}", values);

    for &val in &values {
        stack.push(val);
        println!("Pushed {}, max: {:?}", val, stack.max());
    }

    println!("\nPopping elements:");
    while let Some(item) = stack.pop() {
        println!("Popped {}, max: {:?}", item, stack.max());
    }
    println!();
}

// ================================================================================
// 5. DEQUE-BASED STACK (WRAPPER AROUND VECDEQUE)
// ================================================================================

#[derive(Debug)]
struct DequeStack<T> {
    data: VecDeque<T>,
}

impl<T> DequeStack<T> {
    fn new() -> Self {
        DequeStack {
            data: VecDeque::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.data.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    fn peek(&self) -> Option<&T> {
        self.data.back()
    }

    fn peek_front(&self) -> Option<&T> {
        self.data.front()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> Default for DequeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn demonstrate_deque_stack() {
    println!("=== DEQUE-BASED STACK ===");

    let mut stack = DequeStack::new();

    // Push elements
    for i in 1..=6 {
        stack.push(i);
    }

    println!("Stack: back = {:?}", stack.peek());
    println!("Front (oldest): {:?}", stack.peek_front());
    println!("Length: {}", stack.len());
    println!("Capacity: {}", stack.capacity());

    // Pop elements
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }

    println!("Stack empty: {}", stack.is_empty());
    println!();
}

// ================================================================================
// 6. GENERIC STACK TRAIT
// ================================================================================

trait Stack<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Stack<T> for LinkedStack<T> {
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.peek()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

fn demonstrate_generic_stack() {
    println!("=== GENERIC STACK TRAIT ===");

    fn stack_operations<S: Stack<i32> + std::fmt::Debug>(stack: &mut S, name: &str) {
        println!("Testing {}:", name);

        // Push elements
        for i in 1..=4 {
            stack.push(i);
        }

        println!("  Stack: {:?}", stack);
        println!("  Top: {:?}", stack.peek());
        println!("  Length: {}", stack.len());
        println!("  Is empty: {}", stack.is_empty());

        // Pop elements
        while !stack.is_empty() {
            println!("  Popped: {:?}", stack.pop());
        }
    }

    let mut array_stack = ArrayStack::new();
    let mut linked_stack = LinkedStack::new();

    stack_operations(&mut array_stack, "ArrayStack");
    stack_operations(&mut linked_stack, "LinkedStack");
    println!();
}

// ================================================================================
// 7. STACK APPLICATIONS
// ================================================================================

// Parentheses matching
fn is_balanced_parentheses(s: &str) -> bool {
    let mut stack = ArrayStack::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

// Postfix expression evaluation
fn evaluate_postfix(expr: &str) -> Option<i32> {
    let mut stack = ArrayStack::new();

    for token in expr.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
        } else {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                "^" => a.pow(b as u32),
                _ => return None,
            };
            stack.push(result);
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

// Infix to postfix conversion
fn infix_to_postfix(expr: &str) -> String {
    let mut stack = ArrayStack::new();
    let mut result = Vec::new();

    for token in expr.split_whitespace() {
        if let Ok(_) = token.parse::<i32>() {
            result.push(token);
        } else if token == "(" {
            stack.push(token);
        } else if token == ")" {
            while let Some(op) = stack.pop() {
                if op == "(" {
                    break;
                }
                result.push(op);
            }
        } else {
            while let Some(top) = stack.peek() {
                if *top == "(" || precedence(top) < precedence(token) {
                    break;
                }
                result.push(stack.pop().unwrap());
            }
            stack.push(token);
        }
    }

    while let Some(op) = stack.pop() {
        result.push(op);
    }

    result.join(" ")
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

// Next greater element
fn next_greater_elements(arr: &[i32]) -> Vec<Option<i32>> {
    let mut stack = ArrayStack::new();
    let mut result = vec![None; arr.len()];

    for i in (0..arr.len()).rev() {
        while let Some(&top) = stack.peek() {
            if top <= arr[i] {
                stack.pop();
            } else {
                break;
            }
        }

        result[i] = stack.peek().copied();
        stack.push(arr[i]);
    }

    result
}

// Stock span problem
fn stock_span(prices: &[i32]) -> Vec<usize> {
    let mut stack = ArrayStack::new();
    let mut spans = Vec::new();

    for i in 0..prices.len() {
        while let Some(&top) = stack.peek() {
            if prices[top] <= prices[i] {
                stack.pop();
            } else {
                break;
            }
        }

        let span = if stack.is_empty() {
            i + 1
        } else {
            i - stack.peek().unwrap()
        };

        spans.push(span);
        stack.push(i);
    }

    spans
}

fn demonstrate_stack_applications() {
    println!("=== STACK APPLICATIONS ===");

    // Parentheses matching
    let expressions = [
        "((a+b)*c)",
        "([{}])",
        "([)]",
        "{[()]}",
        "(((",
    ];

    println!("Parentheses matching:");
    for expr in &expressions {
        println!("  {}: {}", expr, is_balanced_parentheses(expr));
    }

    // Postfix evaluation
    let postfix_expr = "3 4 + 2 * 7 /";
    if let Some(result) = evaluate_postfix(postfix_expr) {
        println!("\nPostfix evaluation:");
        println!("  {} = {}", postfix_expr, result);
    }

    // Infix to postfix
    let infix_expr = "3 + 4 * 2 / ( 1 - 5 )";
    let postfix = infix_to_postfix(infix_expr);
    println!("\nInfix to postfix:");
    println!("  {}", infix_expr);
    println!("  {}", postfix);

    // Next greater element
    let arr = [4, 5, 2, 25];
    let nge = next_greater_elements(&arr);
    println!("\nNext greater element:");
    println!("  Array: {:?}", arr);
    println!("  NGE:   {:?}", nge);

    // Stock span problem
    let prices = [100, 80, 60, 70, 60, 75, 85];
    let spans = stock_span(&prices);
    println!("\nStock span problem:");
    println!("  Prices: {:?}", prices);
    println!("  Spans:  {:?}", spans);
    println!();
}

// ================================================================================
// 8. STACK WITH CUSTOM COMPARATOR
// ================================================================================

#[derive(Debug)]
struct CustomStack<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    data: Vec<T>,
    comparator: F,
}

impl<T, F> CustomStack<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    fn new(comparator: F) -> Self {
        CustomStack {
            data: Vec::new(),
            comparator,
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn find_extreme(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }

        let mut extreme = &self.data[0];
        for item in &self.data[1..] {
            if (self.comparator)(item, extreme) {
                extreme = item;
            }
        }
        Some(extreme)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

fn demonstrate_custom_stack() {
    println!("=== CUSTOM STACK WITH COMPARATOR ===");

    // Stack that keeps track of minimum using custom comparator
    let mut min_stack = CustomStack::new(|a: &i32, b: &i32| a < b);

    let values = [5, 3, 7, 1, 4];
    for &val in &values {
        min_stack.push(val);
        println!("Pushed {}, min: {:?}", val, min_stack.find_extreme());
    }

    // Stack that keeps track of maximum using custom comparator
    let mut max_stack = CustomStack::new(|a: &i32, b: &i32| a > b);

    for &val in &values {
        max_stack.push(val);
        println!("Pushed {}, max: {:?}", val, max_stack.find_extreme());
    }

    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE STACK DATA STRUCTURES             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_array_stack();
    demonstrate_linked_stack();
    demonstrate_min_stack();
    demonstrate_max_stack();
    demonstrate_deque_stack();
    demonstrate_generic_stack();
    demonstrate_stack_applications();
    demonstrate_custom_stack();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers:");
    println!("• Array-based stacks with capacity management");
    println!("• Linked list-based stacks with dynamic sizing");
    println!("• Specialized stacks (min/max operations)");
    println!("• Deque-based stack implementation");
    println!("• Generic stack trait for polymorphism");
    println!("• Practical stack applications");
    println!("• Custom comparator stacks");
    println!();
    println!("Key concepts demonstrated:");
    println!("✅ LIFO (Last In, First Out) principle");
    println!("✅ Memory management with different backing structures");
    println!("✅ O(1) push and pop operations");
    println!("✅ Iterator implementations");
    println!("✅ Generic programming with traits");
    println!("✅ Real-world algorithm applications");
    println!("✅ Performance considerations");
}

/*
================================================================================
STACK DATA STRUCTURES SUMMARY:
================================================================================

1. ARRAY-BASED STACK:
   - Backed by Vec<T> for efficient memory usage
   - O(1) amortized push/pop operations
   - Cache-friendly memory layout
   - Capacity management options

2. LINKED LIST-BASED STACK:
   - Unbounded dynamic sizing
   - O(1) push/pop with no reallocation
   - Memory overhead per node
   - Good for unknown growth patterns

3. SPECIALIZED STACKS:
   - MinStack: O(1) minimum element retrieval
   - MaxStack: O(1) maximum element retrieval
   - Custom comparator stacks
   - Space-optimized with auxiliary structures

4. APPLICATIONS:
   - Parentheses matching
   - Expression evaluation (postfix/infix)
   - Function call simulation
   - Undo/redo operations
   - Graph algorithms (DFS)

5. RUST-SPECIFIC FEATURES:
   - Generic implementations with traits
   - Memory safety without garbage collection
   - Zero-cost abstractions
   - Iterator pattern support
   - Compile-time type checking

6. PERFORMANCE CHARACTERISTICS:
   - Push: O(1) amortized
   - Pop: O(1)
   - Peek: O(1)
   - Search: O(n)
   - Space: O(n)

7. THREAD SAFETY:
   - Not inherently thread-safe
   - Can be wrapped in Arc<Mutex<>> for concurrent access
   - Lock-free implementations possible with atomic operations

COMPILATION:
rustc 03-Stacks.rs -o stacks_demo

USAGE:
./stacks_demo
================================================================================
*/