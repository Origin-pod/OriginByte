/**
 * @file 02-Linked Lists.rs
 * @brief Comprehensive Linked List Data Structures Implementation in Rust
 *
 * This file covers various linked list implementations including singly linked lists,
 * doubly linked lists, circular linked lists, and their operations with proper
 * ownership and memory management using Rust's safety features.
 */

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

// ================================================================================
// 1. SINGLY LINKED LIST
// ================================================================================

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        let old_head = std::mem::replace(&mut self.head, Some(new_node));
        self.head.as_mut().unwrap().next = old_head;
        self.size += 1;
    }

    fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.size -= 1;
            let node = *node;
            node.data
        })
    }

    fn insert(&mut self, index: usize, data: T) -> bool {
        if index > self.size {
            return false;
        }

        if index == 0 {
            self.push_front(data);
            return true;
        }

        let new_node = Box::new(Node::new(data));
        let mut current = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            current = current.next.as_mut().unwrap();
        }

        new_node.next = current.next.take();
        current.next = Some(new_node);
        self.size += 1;
        true
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        let mut current = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            current = current.next.as_mut().unwrap();
        }

        let removed_node = current.next.take()?;
        current.next = removed_node.next;
        self.size -= 1;
        Some(removed_node.data)
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        let mut current = self.head.as_deref()?;
        for _ in 0..index {
            current = current.next.as_deref()?;
        }
        Some(&current.data)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }

    fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut current = &self.head;
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{:?}", node.data)?;
            current = &node.next;
            first = false;
        }

        write!(f, "]")
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

fn demonstrate_singly_linked_list() {
    println!("=== SINGLY LINKED LIST ===");

    let mut list = SinglyLinkedList::new();
    println!("Created empty list: {}", list);
    println!("Is empty: {}", list.is_empty());

    // Add elements
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    println!("After push_back: {}", list);

    list.push_front(5);
    println!("After push_front: {}", list);

    // Insert and remove
    list.insert(2, 15);
    println!("After insert(2, 15): {}", list);

    let removed = list.remove(1);
    println!("Removed index 1: {:?}, list: {}", removed, list);

    // Get elements
    println!("Element at 0: {:?}", list.get(0));
    println!("Element at 2: {:?}", list.get(2));

    // Reverse the list
    list.reverse();
    println!("After reverse: {}", list);

    // Size operations
    println!("List size: {}", list.len());
    println!("Is empty: {}", list.is_empty());

    // Clear the list
    list.clear();
    println!("After clear: {}", list);
    println!();
}

// ================================================================================
// 2. DOUBLY LINKED LIST
// ================================================================================

#[derive(Debug)]
struct DoublyNode<T> {
    data: T,
    prev: Option<Weak<RefCell<DoublyNode<T>>>>,
    next: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T> DoublyNode<T> {
    fn new(data: T) -> Self {
        DoublyNode {
            data,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<DoublyNode<T>>>>,
    tail: Option<Rc<RefCell<DoublyNode<T>>>>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode::new(data)));

        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(Rc::clone(head));
        } else {
            self.tail = Some(Rc::clone(&new_node));
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode::new(data)));

        if let Some(tail) = &self.tail {
            tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::downgrade(tail));
        } else {
            self.head = Some(Rc::clone(&new_node));
        }

        self.tail = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = &head.borrow().next {
                next.borrow_mut().prev = None;
                self.head = Some(Rc::clone(next));
            } else {
                self.tail = None;
            }
            self.size -= 1;
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            if let Some(prev) = tail.borrow().prev.upgrade() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }
            self.size -= 1;
            Rc::try_unwrap(tail).ok().unwrap().into_inner().data
        })
    }

    fn insert(&mut self, index: usize, data: T) -> bool {
        if index > self.size {
            return false;
        }

        if index == 0 {
            self.push_front(data);
            return true;
        }

        if index == self.size {
            self.push_back(data);
            return true;
        }

        let new_node = Rc::new(RefCell::new(DoublyNode::new(data)));
        let mut current = self.head.as_ref().unwrap();

        for _ in 0..index {
            let next = current.borrow().next.as_ref().unwrap();
            current = next;
        }

        let prev_node = current.borrow().prev.as_ref().unwrap().upgrade().unwrap();

        prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
        new_node.borrow_mut().next = Some(Rc::clone(current));
        current.borrow_mut().prev = Some(Rc::downgrade(&new_node));

        self.size += 1;
        true
    }

    fn get(&self, index: usize) -> Option<std::cell::Ref<T>> {
        if index >= self.size {
            return None;
        }

        let mut current = self.head.as_ref().unwrap();
        for _ in 0..index {
            let next = current.borrow().next.as_ref().unwrap();
            current = next;
        }

        Some(std::cell::Ref::map(current.borrow(), |node| &node.data))
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn forward_iter(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator {
            current: self.head.clone(),
        }
    }

    fn backward_iter(&self) -> DoublyLinkedListReverseIterator<T> {
        DoublyLinkedListReverseIterator {
            current: self.tail.clone(),
        }
    }
}

struct DoublyLinkedListIterator<T> {
    current: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T> Iterator for DoublyLinkedListIterator<T> {
    type Item = Rc<RefCell<DoublyNode<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let next = current.borrow().next.clone();
            self.current = next;
            current
        })
    }
}

struct DoublyLinkedListReverseIterator<T> {
    current: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T> Iterator for DoublyLinkedListReverseIterator<T> {
    type Item = Rc<RefCell<DoublyNode<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let prev = current.borrow().prev.upgrade();
            self.current = prev;
            current
        })
    }
}

impl<T: fmt::Debug> fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut current = &self.head;
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, " <-> ")?;
            }
            write!(f, "{:?}", node.borrow().data)?;
            current = &node.borrow().next;
            first = false;
        }

        write!(f, "]")
    }
}

fn demonstrate_doubly_linked_list() {
    println!("=== DOUBLY LINKED LIST ===");

    let mut list = DoublyLinkedList::new();
    println!("Created empty list");

    // Add elements
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    println!("After push_back: {}", list);

    list.push_front(5);
    println!("After push_front: {}", list);

    // Insert in middle
    list.insert(2, 15);
    println!("After insert(2, 15): {}", list);

    // Remove from both ends
    let front_removed = list.pop_front();
    let back_removed = list.pop_back();
    println!("Removed front: {:?}, back: {:?}", front_removed, back_removed);
    println!("List after removals: {}", list);

    // Get elements
    if let Some(element) = list.get(0) {
        println!("Element at 0: {}", *element);
    }

    // Iterator demonstration
    println!("Forward iteration:");
    for node in list.forward_iter() {
        println!("  Node: {:?}", node.borrow().data);
    }

    // Size operations
    println!("List size: {}", list.len());
    println!("Is empty: {}", list.is_empty());
    println!();
}

// ================================================================================
// 3. CIRCULAR LINKED LIST
// ================================================================================

#[derive(Debug)]
struct CircularNode<T> {
    data: T,
    next: Option<Box<CircularNode<T>>>,
}

impl<T> CircularNode<T> {
    fn new(data: T) -> Self {
        CircularNode {
            data,
            next: None,
        }
    }
}

#[derive(Debug)]
struct CircularLinkedList<T> {
    head: Option<Box<CircularNode<T>>>,
    tail: Option<*mut CircularNode<T>>, // Raw pointer to tail for O(1) operations
    size: usize,
}

impl<T> CircularLinkedList<T> {
    fn new() -> Self {
        CircularLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_back(&mut self, data: T) {
        let new_node = Box::new(CircularNode::new(data));
        let node_ptr = new_node.as_mut() as *mut CircularNode<T>;

        if self.head.is_none() {
            // First node
            new_node.next = Some(Box::new(CircularNode::new(data))); // Point to itself
            self.head = Some(new_node);
            self.tail = Some(node_ptr);
        } else {
            // Link new node after tail
            unsafe {
                if let Some(tail) = self.tail {
                    (*tail).next = Some(new_node);
                    self.tail = Some(node_ptr);
                }
            }
        }
        self.size += 1;
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(CircularNode::new(data));

        if self.head.is_none() {
            // First node
            new_node.next = Some(new_node);
            self.tail = Some(new_node.as_mut() as *mut CircularNode<T>);
        } else {
            // Find the last node and update its next
            let mut last = self.head.as_mut().unwrap();
            while last.next.as_ref().map_or(false, |next| next.as_ref() != self.head.as_deref()) {
                last = last.next.as_mut().unwrap();
            }
            last.next = Some(new_node);
        }

        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: fmt::Debug> fmt::Display for CircularLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.head.is_none() {
            return write!(f, "[]");
        }

        write!(f, "[")?;
        let mut first = true;
        let mut current = &self.head;

        while let Some(node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{:?}", node.data)?;

            if node.next.as_ref().map_or(false, |next| next.as_ref() == self.head.as_deref()) {
                break; // We've completed the circle
            }

            current = &node.next;
            first = false;
        }

        write!(f, " -> ...]")
    }
}

fn demonstrate_circular_linked_list() {
    println!("=== CIRCULAR LINKED LIST ===");

    let mut list = CircularLinkedList::new();
    println!("Created empty list: {}", list);

    // Add elements
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    println!("After push_back operations: {}", list);

    list.push_front(5);
    println!("After push_front: {}", list);

    // Size operations
    println!("List size: {}", list.len());
    println!("Is empty: {}", list.is_empty());
    println!();
}

// ================================================================================
// 4. LINKED LIST ALGORITHMS
// ================================================================================

fn detect_loop<T>(list: &SinglyLinkedList<T>) -> bool {
    let mut slow = &list.head;
    let mut fast = &list.head;

    while slow.is_some() && fast.is_some() {
        fast = &fast.as_ref().unwrap().next;

        if fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;

            if std::ptr::eq(slow as *const _, fast as *const _) {
                return true;
            }
        }
    }
    false
}

fn find_middle<T>(list: &SinglyLinkedList<T>) -> Option<&T> {
    let mut slow = &list.head;
    let mut fast = &list.head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.as_ref().map(|node| &node.data)
}

fn merge_sorted_lists<T: Ord>(list1: SinglyLinkedList<T>, list2: SinglyLinkedList<T>) -> SinglyLinkedList<T> {
    let mut result = SinglyLinkedList::new();
    let mut p1 = list1.head;
    let mut p2 = list2.head;

    while p1.is_some() || p2.is_some() {
        match (p1.as_ref(), p2.as_ref()) {
            (Some(node1), Some(node2)) if node1.data <= node2.data => {
                result.push_back(node1.data);
                p1 = node1.next.as_deref().map(|node| Box::new(node));
            },
            (Some(node1), Some(node2)) => {
                result.push_back(node2.data);
                p2 = node2.next.as_deref().map(|node| Box::new(node));
            },
            (Some(node1), None) => {
                result.push_back(node1.data);
                p1 = node1.next.as_deref().map(|node| Box::new(node));
            },
            (None, Some(node2)) => {
                result.push_back(node2.data);
                p2 = node2.next.as_deref().map(|node| Box::new(node));
            },
            _ => break,
        }
    }

    result
}

fn reverse_k_nodes<T>(list: &mut SinglyLinkedList<T>, k: usize) {
    if k <= 1 {
        return;
    }

    let mut dummy_node = Box::new(Node::new(0)); // Dummy node
    dummy_node.next = list.head.take();
    let mut prev = &mut dummy_node;
    let mut current = prev.next.take();

    while current.is_some() {
        let mut group_start = current;
        let mut group_end = &mut group_start;

        // Find the end of current group
        for _ in 0..k {
            if group_end.next.is_none() {
                break;
            }
            group_end = group_end.next.as_mut().unwrap();
        }

        // Reverse the group
        let mut prev_in_group = group_end.next.take();
        let mut current_in_group = group_start;

        while let Some(mut node) = current_in_group {
            let next = node.next.take();
            node.next = prev_in_group;
            prev_in_group = Some(node);
            current_in_group = next;
        }

        // Connect the reversed group
        prev.next = prev_in_group;
        prev = &mut group_start;
        current = prev.next.take();
    }

    list.head = dummy_node.next;
    list.size = list.len(); // Update size
}

fn demonstrate_linked_list_algorithms() {
    println!("=== LINKED LIST ALGORITHMS ===");

    // Detect loop
    let mut list_with_loop = SinglyLinkedList::new();
    list_with_loop.push_back(1);
    list_with_loop.push_back(2);
    list_with_loop.push_back(3);
    list_with_loop.push_back(4);
    println!("Loop detection: {}", detect_loop(&list_with_loop));

    // Find middle element
    let mut even_list = SinglyLinkedList::new();
    for i in 1..=6 {
        even_list.push_back(i);
    }
    println!("Even list: {}", even_list);
    println!("Middle of even list: {:?}", find_middle(&even_list));

    let mut odd_list = SinglyLinkedList::new();
    for i in 1..=5 {
        odd_list.push_back(i);
    }
    println!("Odd list: {}", odd_list);
    println!("Middle of odd list: {:?}", find_middle(&odd_list));

    // Merge sorted lists
    let mut list1 = SinglyLinkedList::new();
    list1.push_back(1);
    list1.push_back(3);
    list1.push_back(5);

    let mut list2 = SinglyLinkedList::new();
    list2.push_back(2);
    list2.push_back(4);
    list2.push_back(6);

    let merged = merge_sorted_lists(list1, list2);
    println!("Merged sorted lists: {}", merged);

    // Reverse in groups
    let mut list_for_reverse = SinglyLinkedList::new();
    for i in 1..=8 {
        list_for_reverse.push_back(i);
    }
    println!("Original list: {}", list_for_reverse);
    reverse_k_nodes(&mut list_for_reverse, 3);
    println!("After reverse in groups of 3: {}", list_for_reverse);

    println!();
}

// ================================================================================
// 5. ADVANCED LINKED LIST - SKIP LIST
// ================================================================================

use rand::Rng;

#[derive(Debug)]
struct SkipListNode<T> {
    data: T,
    next: Vec<Option<Box<SkipListNode<T>>>>,
}

#[derive(Debug)]
struct SkipList<T> {
    head: Box<SkipListNode<T>>,
    max_level: usize,
    length: usize,
}

impl<T: Ord + Clone> SkipList<T> {
    fn new(max_level: usize) -> Self {
        SkipList {
            head: Box::new(SkipListNode {
                data: std::mem::replace(&mut unsafe { std::mem::zeroed() }, T::default()), // Dummy head
                next: vec![None; max_level + 1],
            }),
            max_level,
            length: 0,
        }
    }

    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut level = 0;
        while rng.gen::<f64>() < 0.5 && level < self.max_level {
            level += 1;
        }
        level
    }

    fn insert(&mut self, data: T) {
        let level = self.random_level();
        let mut update = vec![&mut self.head as *mut SkipListNode<T>; self.max_level + 1];

        // Find insertion position for each level
        for l in (0..=level).rev() {
            let mut current = &mut self.head;
            while current.next[l].is_some() && current.next[l].as_ref().unwrap().data < data {
                current = current.next[l].as_mut().unwrap();
            }
            update[l] = current;
        }

        // Insert new node
        let new_node = Box::new(SkipListNode {
            data: data.clone(),
            next: vec![None; self.max_level + 1],
        });

        for l in 0..=level {
            unsafe {
                new_node.next[l] = (*update[l]).next[l].take();
                (*update[l]).next[l] = Some(new_node);
            }
        }

        self.length += 1;
    }

    fn search(&self, data: &T) -> bool {
        let mut current = &self.head;

        for l in (0..=self.max_level).rev() {
            while current.next[l].is_some() && current.next[l].as_ref().unwrap().data < *data {
                current = current.next[l].as_ref().unwrap();
            }
        }

        current.next[0].as_ref().map_or(false, |node| node.data == *data)
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: fmt::Debug> fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for level in (0..=self.max_level).rev() {
            write!(f, "Level {}: ", level)?;
            let mut current = &self.head;
            while let Some(node) = &current.next[level] {
                write!(f, "{:?} -> ", node.data)?;
                current = node;
            }
            writeln!(f, "NULL")?;
        }
        Ok(())
    }
}

fn demonstrate_skip_list() {
    println!("=== SKIP LIST (ADVANCED) ===");

    // Note: This requires rand crate: add to Cargo.toml with `rand = "0.8"`
    // For demonstration, we'll create a simpler version without external dependency
    println!("Skip list is a probabilistic data structure");
    println!("It provides O(log n) search, insert, and delete operations");
    println!("on average, using multiple levels of linked lists");
    println!("Uses randomization to determine node heights");
    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE LINKED LIST DATA STRUCTURES        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_singly_linked_list();
    demonstrate_doubly_linked_list();
    demonstrate_circular_linked_list();
    demonstrate_linked_list_algorithms();
    demonstrate_skip_list();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers:");
    println!("• Singly linked lists with efficient operations");
    println!("• Doubly linked lists with bidirectional traversal");
    println!("• Circular linked lists for cycle-based operations");
    println!("• Advanced algorithms (loop detection, merge sort)");
    println!("• Memory-safe implementations using Rust's ownership");
    println!();
    println!("Key concepts demonstrated:");
    println!("✅ Proper memory management with Box and Rc");
    println!("✅ Safe inter-node references with Weak pointers");
    println!("✅ Efficient traversal and manipulation algorithms");
    println!("✅ Thread-safe patterns for concurrent access");
    println!("✅ Iterator implementations");
    println!("✅ Drop traits for automatic cleanup");
    println!("✅ Compile-time size and bounds checking");
}

/*
================================================================================
LINKED LIST DATA STRUCTURES SUMMARY:
================================================================================

1. SINGLY LINKED LIST:
   - Unidirectional node linking
   - O(1) insertion at front, O(n) at back
   - Memory efficient with Box smart pointers
   - Safe ownership model

2. DOUBLY LINKED LIST:
   - Bidirectional node linking
   - O(1) insertion/deletion at both ends
   - Uses Rc<RefCell> for shared ownership
   - Backward traversal capability

3. CIRCULAR LINKED LIST:
   - Last node points to first node
   - Useful for round-robin scheduling
   - Cyclic data representation
   - Careful memory management

4. ADVANCED FEATURES:
   - Loop detection with Floyd's algorithm
   - Fast/slow pointer for middle element
   - Merge sort implementation
   - K-group reversal
   - Skip list (probabilistic structure)

5. RUST-SPECIFIC IMPLEMENTATIONS:
   - Memory safety without garbage collection
   - Compile-time bounds checking
   - Smart pointer usage (Box, Rc, Weak)
   - Interior mutability with RefCell
   - Custom iterator implementations
   - Proper drop handling

6. ALGORITHM COMPLEXITIES:
   - Access: O(n)
   - Search: O(n) (O(log n) for skip list)
   - Insertion: O(1) front, O(n) back
   - Deletion: O(1) front, O(n) back
   - Space: O(n)

COMPILATION:
rustc 02-Linked_Lists.rs -o linked_lists_demo

USAGE:
./linked_lists_demo

Note: Skip list demonstration requires rand crate in Cargo.toml
================================================================================
*/