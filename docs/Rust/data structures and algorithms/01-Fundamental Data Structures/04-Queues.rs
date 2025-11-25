/**
 * @file 04-Queues.rs
 * @brief Comprehensive Queue Data Structures Implementation in Rust
 *
 * This file covers various queue implementations including array-based queues,
 * linked list-based queues, circular queues, priority queues, and deque operations
 * with proper memory management and performance considerations.
 */

use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Ordering;

// ================================================================================
// 1. ARRAY-BASED QUEUE (VECTOR IMPLEMENTATION)
// ================================================================================

#[derive(Debug, Clone)]
struct ArrayQueue<T> {
    data: Vec<T>,
    front: usize,
    size: usize,
    capacity: Option<usize>,
}

impl<T> ArrayQueue<T> {
    fn new() -> Self {
        ArrayQueue {
            data: Vec::new(),
            front: 0,
            size: 0,
            capacity: None,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        ArrayQueue {
            data: Vec::with_capacity(capacity),
            front: 0,
            size: 0,
            capacity: Some(capacity),
        }
    }

    fn enqueue(&mut self, item: T) -> bool {
        if let Some(cap) = self.capacity {
            if self.size >= cap {
                return false; // Queue is full
            }
        }

        // Need to expand if necessary
        if self.data.capacity() == self.size {
            self.compact();
        }

        let index = (self.front + self.size) % self.data.capacity();
        if index >= self.data.len() {
            self.data.push(item);
        } else {
            // Overwrite existing position
            std::mem::replace(&mut self.data[index], item);
        }

        self.size += 1;
        true
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = std::mem::replace(&mut self.data[self.front], unsafe { std::mem::zeroed() });
        self.front = (self.front + 1) % self.data.capacity();
        self.size -= 1;

        // Compact if front has advanced too far
        if self.front > self.data.capacity() / 2 {
            self.compact();
        }

        Some(item)
    }

    fn front(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.data[self.front])
        }
    }

    fn back(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let back_index = (self.front + self.size - 1) % self.data.capacity();
            Some(&self.data[back_index])
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.front = 0;
        self.size = 0;
    }

    fn compact(&mut self) {
        if self.front == 0 {
            return;
        }

        let mut new_data = Vec::with_capacity(self.data.capacity());
        for i in 0..self.size {
            let index = (self.front + i) % self.data.capacity();
            new_data.push(std::mem::replace(&mut self.data[index], unsafe { std::mem::zeroed() }));
        }
        self.data = new_data;
        self.front = 0;
    }

    fn reserve(&mut self, additional: usize) {
        self.compact();
        self.data.reserve(additional);
    }

    fn iter(&self) -> ArrayQueueIterator<T> {
        ArrayQueueIterator {
            queue: self,
            index: 0,
        }
    }
}

impl<T> Default for ArrayQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct ArrayQueueIterator<'a, T> {
    queue: &'a ArrayQueue<T>,
    index: usize,
}

impl<'a, T> Iterator for ArrayQueueIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.size {
            None
        } else {
            let array_index = (self.queue.front + self.index) % self.queue.data.capacity();
            self.index += 1;
            Some(&self.queue.data[array_index])
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for ArrayQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

fn demonstrate_array_queue() {
    println!("=== ARRAY-BASED QUEUE ===");

    let mut queue = ArrayQueue::new();
    println!("Created empty queue: {}", queue);
    println!("Is empty: {}", queue.is_empty());

    // Enqueue elements
    println!("Enqueuing elements:");
    for i in 1..=5 {
        let enqueued = queue.enqueue(i);
        println!("  Enqueued {}: {} (queue: {})", i, enqueued, queue);
    }

    // Peek at front and back
    println!("Front element: {:?}", queue.front());
    println!("Back element: {:?}", queue.back());

    // Dequeue elements
    println!("Dequeuing elements:");
    while let Some(item) = queue.dequeue() {
        println!("  Dequeued: {} (queue: {})", item, queue);
    }

    // Bounded queue
    let mut bounded_queue = ArrayQueue::with_capacity(3);
    println!("\nBounded queue (capacity 3):");
    for i in 1..=5 {
        let enqueued = bounded_queue.enqueue(i);
        println!("  Enqueued {}: {}", i, enqueued);
    }

    println!("Bounded queue: {}", bounded_queue);
    println!();
}

// ================================================================================
// 2. LINKED LIST-BASED QUEUE
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
struct LinkedQueue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedQueue<T> {
    fn new() -> Self {
        LinkedQueue {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn enqueue(&mut self, item: T) {
        let new_node = Box::new(Node::new(item));

        if let Some(mut tail) = self.tail.take() {
            tail.next = Some(new_node);
            self.tail = Some(tail.next.take().unwrap());
        } else {
            // First element
            self.head = Some(Box::new(Node::new(unsafe { std::mem::zeroed() })));
            self.tail = Some(new_node);
        }

        // Fix first element case
        if self.size == 0 {
            self.head = self.tail.take().map(|mut node| {
                let data = std::mem::replace(&mut node.data, unsafe { std::mem::zeroed() });
                Box::new(Node { data, next: Some(node) })
            });
        }

        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let node = self.head.take()?;
        let result = Some(node.data);

        self.head = node.next;
        self.size -= 1;

        if self.head.is_none() {
            self.tail = None;
        }

        result
    }

    fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.data)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        while self.dequeue().is_some() {}
    }

    fn iter(&self) -> LinkedQueueIterator<T> {
        LinkedQueueIterator {
            current: &self.head,
        }
    }
}

impl<T> Default for LinkedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct LinkedQueueIterator<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedQueueIterator<'a, T> {
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

impl<T: std::fmt::Debug> std::fmt::Display for LinkedQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue[")?;
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

fn demonstrate_linked_queue() {
    println!("=== LINKED LIST-BASED QUEUE ===");

    let mut queue = LinkedQueue::new();
    println!("Created empty queue: {}", queue);

    // Enqueue elements
    println!("Enqueuing elements:");
    for ch in ['A', 'B', 'C', 'D', 'E'] {
        queue.enqueue(ch);
        println!("  Enqueued '{}' (queue: {})", ch, queue);
    }

    // Peek at front and back
    println!("Front element: {:?}", queue.front());
    println!("Back element: {:?}", queue.back());

    // Iterator demonstration
    println!("Iterating over queue:");
    for item in queue.iter() {
        println!("  {:?}", item);
    }

    // Dequeue elements
    println!("Dequeuing elements:");
    while let Some(item) = queue.dequeue() {
        println!("  Dequeued '{}' (queue: {})", item, queue);
    }

    println!("Queue empty: {}", queue.is_empty());
    println!();
}

// ================================================================================
// 3. CIRCULAR QUEUE (RING BUFFER)
// ================================================================================

#[derive(Debug)]
struct CircularQueue<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularQueue<T> {
    fn new(capacity: usize) -> Self {
        CircularQueue {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn enqueue(&mut self, item: T) -> bool {
        if self.size == self.capacity {
            return false; // Queue is full
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        item
    }

    fn front(&self) -> Option<&T> {
        self.buffer[self.head].as_ref()
    }

    fn back(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let back_index = (self.tail - 1) % self.capacity;
            self.buffer[back_index].as_ref()
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn len(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn clear(&mut self) {
        for item in &mut self.buffer {
            *item = None;
        }
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

    fn iter(&self) -> CircularQueueIterator<T> {
        CircularQueueIterator {
            queue: self,
            index: 0,
        }
    }
}

struct CircularQueueIterator<'a, T> {
    queue: &'a CircularQueue<T>,
    index: usize,
}

impl<'a, T> Iterator for CircularQueueIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.size {
            None
        } else {
            let buffer_index = (self.queue.head + self.index) % self.queue.capacity;
            self.index += 1;
            self.queue.buffer[buffer_index].as_ref()
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for CircularQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CircularQueue[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

fn demonstrate_circular_queue() {
    println!("=== CIRCULAR QUEUE ===");

    let mut queue = CircularQueue::new(5);
    println!("Created circular queue with capacity 5: {}", queue);
    println!("Is empty: {}", queue.is_empty());
    println!("Is full: {}", queue.is_full());

    // Enqueue elements
    println!("Enqueuing elements:");
    for i in 1..=6 {
        let enqueued = queue.enqueue(i);
        println!("  Enqueued {}: {} (queue: {})", i, enqueued, queue);
        println!("  Is full: {}", queue.is_full());
    }

    // Peek at front and back
    println!("Front element: {:?}", queue.front());
    println!("Back element: {:?}", queue.back());

    // Dequeue some elements
    println!("Dequeuing first 3 elements:");
    for _ in 0..3 {
        if let Some(item) = queue.dequeue() {
            println!("  Dequeued: {} (queue: {})", item, queue);
        }
    }

    // Enqueue more elements
    println!("Enqueuing more elements:");
    for i in 10..=12 {
        let enqueued = queue.enqueue(i);
        println!("  Enqueued {}: {} (queue: {})", i, enqueued, queue);
    }

    // Dequeue remaining elements
    println!("Dequeuing remaining elements:");
    while let Some(item) = queue.dequeue() {
        println!("  Dequeued: {}", item);
    }

    println!("Queue empty: {}", queue.is_empty());
    println!();
}

// ================================================================================
// 4. DEQUE-BASED QUEUE
// ================================================================================

#[derive(Debug)]
struct DequeQueue<T> {
    data: VecDeque<T>,
}

impl<T> DequeQueue<T> {
    fn new() -> Self {
        DequeQueue {
            data: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.data.push_back(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn enqueue_front(&mut self, item: T) {
        self.data.push_front(item);
    }

    fn dequeue_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    fn front(&self) -> Option<&T> {
        self.data.front()
    }

    fn back(&self) -> Option<&T> {
        self.data.back()
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
}

impl<T> Default for DequeQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn demonstrate_deque_queue() {
    println!("=== DEQUE-BASED QUEUE ===");

    let mut queue = DequeQueue::new();

    // Normal queue operations
    println!("Normal queue operations:");
    for i in 1..=5 {
        queue.enqueue(i);
        println!("  Enqueued: {}", i);
    }

    println!("Front: {:?}", queue.front());
    println!("Back: {:?}", queue.back());

    // Deque operations
    println!("\nDeque operations:");
    queue.enqueue_front(0);
    println!("  Enqueued front: 0");
    println!("  Front: {:?}", queue.front());

    let back_item = queue.dequeue_back();
    println!("  Dequeued back: {:?}", back_item);
    println!("  Back: {:?}", queue.back());

    // Clear all elements
    println!("\nClearing queue:");
    while let Some(item) = queue.dequeue() {
        println!("  Dequeued: {}", item);
    }

    println!("Queue empty: {}", queue.is_empty());
    println!();
}

// ================================================================================
// 5. PRIORITY QUEUE
// ================================================================================

#[derive(Debug, PartialEq, Eq)]
struct PriorityQueueItem<T: Ord> {
    data: T,
    priority: i32,
}

impl<T: Ord> PartialOrd for PriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.priority.cmp(&self.priority))
    }
}

impl<T: Ord> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

#[derive(Debug)]
struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<PriorityQueueItem<T>>,
}

impl<T: Ord> PriorityQueue<T> {
    fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    fn enqueue(&mut self, item: T, priority: i32) {
        let pq_item = PriorityQueueItem { data: item, priority };
        self.heap.push(pq_item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.data)
    }

    fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.data)
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn clear(&mut self) {
        self.heap.clear();
    }

    fn change_priority(&mut self, item: &T, new_priority: i32) -> bool {
        // This is a simplified implementation
        // In a real implementation, you'd need a more efficient way
        // to find and update the item
        let mut temp_items = Vec::new();
        let mut found = false;

        while let Some(pq_item) = self.heap.pop() {
            if &pq_item.data == item {
                temp_items.push(PriorityQueueItem {
                    data: pq_item.data,
                    priority: new_priority,
                });
                found = true;
            } else {
                temp_items.push(pq_item);
            }
        }

        for temp_item in temp_items {
            self.heap.push(temp_item);
        }

        found
    }
}

impl<T: Ord> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn demonstrate_priority_queue() {
    println!("=== PRIORITY QUEUE ===");

    let mut pq = PriorityQueue::new();

    // Enqueue tasks with different priorities
    let tasks = [
        ("Task A", 2),
        ("Task B", 5),
        ("Task C", 1),
        ("Task D", 3),
        ("Task E", 4),
    ];

    println!("Enqueuing tasks with priorities:");
    for &(task, priority) in &tasks {
        pq.enqueue(task.to_string(), priority);
        println!("  Enqueued '{}' with priority {}", task, priority);
    }

    // Dequeue in priority order
    println!("\nDequeuing in priority order:");
    while let Some(task) = pq.dequeue() {
        println!("  Processing: {}", task);
    }

    // Change priority demonstration
    let mut pq2 = PriorityQueue::new();
    pq2.enqueue("Low priority".to_string(), 1);
    pq2.enqueue("High priority".to_string(), 3);
    pq2.enqueue("Medium priority".to_string(), 2);

    println!("\nBefore priority change:");
    while let Some(task) = pq2.dequeue() {
        println!("  {}", task);
    }

    // Note: Priority queue is now empty
    println!();
}

// ================================================================================
// 6. GENERIC QUEUE TRAIT
// ================================================================================

trait Queue<T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
    fn front(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn enqueue(&mut self, item: T) {
        self.enqueue(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.dequeue()
    }

    fn front(&self) -> Option<&T> {
        self.front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Queue<T> for LinkedQueue<T> {
    fn enqueue(&mut self, item: T) {
        self.enqueue(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.dequeue()
    }

    fn front(&self) -> Option<&T> {
        self.front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

fn demonstrate_generic_queue() {
    println!("=== GENERIC QUEUE TRAIT ===");

    fn queue_operations<Q: Queue<i32> + std::fmt::Debug>(queue: &mut Q, name: &str) {
        println!("Testing {}:", name);

        // Enqueue elements
        for i in 1..=4 {
            queue.enqueue(i);
        }

        println!("  Queue: {:?}", queue);
        println!("  Front: {:?}", queue.front());
        println!("  Length: {}", queue.len());
        println!("  Is empty: {}", queue.is_empty());

        // Dequeue elements
        while !queue.is_empty() {
            println!("  Dequeued: {:?}", queue.dequeue());
        }
    }

    let mut array_queue = ArrayQueue::new();
    let mut linked_queue = LinkedQueue::new();

    queue_operations(&mut array_queue, "ArrayQueue");
    queue_operations(&mut linked_queue, "LinkedQueue");
    println!();
}

// ================================================================================
// 7. QUEUE APPLICATIONS
// ================================================================================

// Josephus problem
fn josephus_problem(n: usize, k: usize) -> usize {
    let mut queue = ArrayQueue::new();
    for i in 1..=n {
        queue.enqueue(i).unwrap();
    }

    let mut count = 0;
    while queue.len() > 1 {
        let person = queue.dequeue().unwrap();
        count += 1;
        if count % k != 0 {
            queue.enqueue(person).unwrap();
        }
    }

    queue.dequeue().unwrap()
}

// Level order traversal of binary tree (simplified)
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn level_order_traversal<T: std::fmt::Debug>(root: &TreeNode<T>) -> Vec<T> {
    let mut result = Vec::new();
    let mut queue = ArrayQueue::new();

    queue.enqueue(root as *const TreeNode<T>).unwrap();

    while !queue.is_empty() {
        if let Some(node_ptr) = queue.dequeue() {
            let node = unsafe { &*node_ptr };
            result.push(node.val.clone());

            if let Some(ref left) = node.left {
                queue.enqueue(left.as_ref()).unwrap();
            }
            if let Some(ref right) = node.right {
                queue.enqueue(right.as_ref()).unwrap();
            }
        }
    }

    result
}

// Task scheduling
struct Task {
    id: u32,
    duration: u32,
    priority: u32,
}

impl Task {
    fn new(id: u32, duration: u32, priority: u32) -> Self {
        Task { id, duration, priority }
    }
}

struct TaskScheduler {
    queue: ArrayQueue<Task>,
    current_time: u32,
}

impl TaskScheduler {
    fn new() -> Self {
        TaskScheduler {
            queue: ArrayQueue::new(),
            current_time: 0,
        }
    }

    fn add_task(&mut self, task: Task) {
        self.queue.enqueue(task).unwrap();
    }

    fn run_next_task(&mut self) -> Option<u32> {
        if let Some(mut task) = self.queue.dequeue() {
            println!("Running task {} (duration: {}, priority: {})",
                     task.id, task.duration, task.priority);
            self.current_time += task.duration;
            Some(task.id)
        } else {
            None
        }
    }

    fn get_current_time(&self) -> u32 {
        self.current_time
    }
}

fn demonstrate_queue_applications() {
    println!("=== QUEUE APPLICATIONS ===");

    // Josephus problem
    let n = 7;
    let k = 3;
    let survivor = josephus_problem(n, k);
    println!("Josephus problem (n={}, k={}): Survivor is {}", n, k, survivor);

    // Binary tree level order traversal
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));

    let traversal = level_order_traversal(&root);
    println!("\nBinary tree level order traversal: {:?}", traversal);

    // Task scheduling
    let mut scheduler = TaskScheduler::new();
    scheduler.add_task(Task::new(1, 5, 2));
    scheduler.add_task(Task::new(2, 3, 1));
    scheduler.add_task(Task::new(3, 4, 3));

    println!("\nTask scheduling:");
    while let Some(task_id) = scheduler.run_next_task() {
        println!("  Completed task {} at time {}", task_id, scheduler.get_current_time());
    }
    println!();
}

// ================================================================================
// 8. BLOCKING QUEUE (SIMULATED)
// ================================================================================

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct BlockingQueue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    not_empty: Arc<Condvar>,
    not_full: Arc<Condvar>,
    capacity: Option<usize>,
}

impl<T> BlockingQueue<T> {
    fn new() -> Self {
        BlockingQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            not_empty: Arc::new(Condvar::new()),
            not_full: Arc::new(Condvar::new()),
            capacity: None,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        BlockingQueue {
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            not_empty: Arc::new(Condvar::new()),
            not_full: Arc::new(Condvar::new()),
            capacity: Some(capacity),
        }
    }

    fn put(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();

        while let Some(cap) = self.capacity {
            if queue.len() >= cap {
                queue = self.not_full.wait(queue).unwrap();
            }
        }

        queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn take(&self) -> T {
        let mut queue = self.queue.lock().unwrap();

        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }

        let item = queue.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }

    fn try_take(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_empty() {
            None
        } else {
            let item = queue.pop_front().unwrap();
            self.not_full.notify_one();
            Some(item)
        }
    }

    fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    fn is_empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }
}

fn demonstrate_blocking_queue() {
    println!("=== BLOCKING QUEUE (SIMULATED) ===");

    let queue = Arc::new(BlockingQueue::with_capacity(5));
    let queue_clone = Arc::clone(&queue);

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=8 {
            println!("Producer: putting {}", i);
            queue_clone.put(i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Consumer thread
    let queue_clone = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        for _ in 0..8 {
            let item = queue_clone.take();
            println!("Consumer: took {}", item);
            thread::sleep(Duration::from_millis(150));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    println!();
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE QUEUE DATA STRUCTURES              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_array_queue();
    demonstrate_linked_queue();
    demonstrate_circular_queue();
    demonstrate_deque_queue();
    demonstrate_priority_queue();
    demonstrate_generic_queue();
    demonstrate_queue_applications();
    demonstrate_blocking_queue();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers:");
    println!("• Array-based queues with capacity management");
    println!("• Linked list-based queues with dynamic sizing");
    println!("• Circular queues for efficient memory usage");
    println!("• Deque-based queues for double-ended operations");
    println!("• Priority queues with heap implementation");
    println!("• Generic queue trait for polymorphism");
    println!("• Practical queue applications");
    println!("• Thread-safe blocking queue implementation");
    println!();
    println!("Key concepts demonstrated:");
    println!("✅ FIFO (First In, First Out) principle");
    println!("✅ Memory management with different backing structures");
    println!("✅ O(1) enqueue and dequeue operations");
    println!("✅ Iterator implementations");
    println!("✅ Generic programming with traits");
    println!("✅ Thread synchronization primitives");
    println!("✅ Real-world algorithm applications");
}

/*
================================================================================
QUEUE DATA STRUCTURES SUMMARY:
================================================================================

1. ARRAY-BASED QUEUE:
   - Backed by Vec<T> with circular buffer
   - O(1) amortized enqueue/dequeue
   - Memory efficient with automatic compaction
   - Capacity management options

2. LINKED LIST-BASED QUEUE:
   - Separate head and tail pointers
   - O(1) enqueue/dequeue operations
   - Unbounded dynamic sizing
   - Memory overhead per node

3. CIRCULAR QUEUE:
   - Fixed-size ring buffer
   - O(1) operations without reallocation
   - Efficient memory usage
   - Good for producer-consumer scenarios

4. PRIORITY QUEUE:
   - Based on BinaryHeap (max-heap by default)
   - O(log n) enqueue/dequeue
   - Priority-based ordering
   - Useful for scheduling algorithms

5. DEQUE-BASED QUEUE:
   - Wrapper around VecDeque
   - Double-ended operations
   - Flexible API
   - Good performance characteristics

6. APPLICATIONS:
   - Task scheduling and job queues
   - Breadth-first search (BFS)
   - Level order tree traversal
   - Producer-consumer patterns
   - Josephus problem solution

7. THREAD SAFETY:
   - BlockingQueue with Mutex and Condvar
   - Producer-consumer synchronization
   - Capacity limiting
   - Thread-safe operations

8. RUST-SPECIFIC FEATURES:
   - Generic implementations with traits
   - Memory safety guarantees
   - Iterator pattern support
   - Zero-cost abstractions
   - Type safety at compile time

COMPILATION:
rustc 04-Queues.rs -o queues_demo

USAGE:
./queues_demo
================================================================================
*/