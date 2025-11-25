/**
 * @file 05-Hash Tables.rs
 * @brief Comprehensive Hash Table Data Structures Implementation in Rust
 *
 * This file covers various hash table implementations including separate chaining,
 * open addressing, custom hash functions, collision resolution strategies, and
 * practical applications with proper memory management and performance analysis.
 */

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

// ================================================================================
// 1. HASH TABLE WITH SEPARATE CHAINING
// ================================================================================

#[derive(Debug)]
struct HashNode<K, V> {
    key: K,
    value: V,
    next: Option<Box<HashNode<K, V>>>,
}

impl<K, V> HashNode<K, V> {
    fn new(key: K, value: V) -> Self {
        HashNode {
            key,
            value,
            next: None,
        }
    }
}

#[derive(Debug)]
struct SeparateChainingHashTable<K, V> {
    buckets: Vec<Option<Box<HashNode<K, V>>>>,
    size: usize,
    capacity: usize,
    load_factor_threshold: f64,
}

impl<K: Hash + Eq + Clone, V: Clone> SeparateChainingHashTable<K, V> {
    fn new() -> Self {
        SeparateChainingHashTable::with_capacity(16)
    }

    fn with_capacity(capacity: usize) -> Self {
        SeparateChainingHashTable {
            buckets: vec![None; capacity.next_power_of_two()],
            size: 0,
            capacity,
            load_factor_threshold: 0.75,
        }
    }

    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.buckets.len() - 1)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.should_resize() {
            self.resize();
        }

        let index = self.hash_key(&key);
        let bucket = &mut self.buckets[index];

        if let Some(node) = bucket {
            let mut current = node;
            loop {
                if current.key == key {
                    let old_value = current.value.clone();
                    current.value = value;
                    return Some(old_value);
                }

                if current.next.is_none() {
                    break;
                }
                current = current.next.as_mut().unwrap();
            }

            // Add new node at the end
            current.next = Some(Box::new(HashNode::new(key, value)));
        } else {
            *bucket = Some(Box::new(HashNode::new(key, value)));
        }

        self.size += 1;
        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        let mut current = &self.buckets[index];

        while let Some(node) = current {
            if node.key == *key {
                return Some(&node.value);
            }
            current = &node.next;
        }

        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.hash_key(key);
        let mut current = &mut self.buckets[index];

        while let Some(node) = current {
            if node.key == *key {
                return Some(&mut node.value);
            }
            current = &mut node.next;
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_key(key);
        let bucket = &mut self.buckets[index];

        if bucket.is_none() {
            return None;
        }

        let mut current = bucket.take().unwrap();

        if current.key == *key {
            self.size -= 1;
            return Some(current.value);
        }

        let mut prev = current;
        while let Some(mut node) = prev.next.take() {
            if node.key == *key {
                prev.next = node.next;
                self.size -= 1;
                return Some(node.value);
            }
            prev.next = Some(node);
            prev = prev.next.as_mut().unwrap();
        }

        // Put back the bucket if key not found
        *bucket = Some(current);
        None
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.buckets.len() as f64
    }

    fn should_resize(&self) -> bool {
        self.load_factor() > self.load_factor_threshold
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_table = SeparateChainingHashTable {
            buckets: vec![None; new_capacity],
            size: 0,
            capacity: new_capacity,
            load_factor_threshold: self.load_factor_threshold,
        };

        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                new_table.insert(node.key.clone(), node.value.clone());
                current = &node.next;
            }
        }

        *self = new_table;
    }

    fn keys(&self) -> Vec<&K> {
        let mut keys = Vec::new();
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                keys.push(&node.key);
                current = &node.next;
            }
        }
        keys
    }

    fn values(&self) -> Vec<&V> {
        let mut values = Vec::new();
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(node) = current {
                values.push(&node.value);
                current = &node.next;
            }
        }
        values
    }

    fn iter(&self) -> HashTableIterator<K, V> {
        HashTableIterator {
            table: self,
            bucket_index: 0,
            current_node: None,
        }
    }
}

struct HashTableIterator<'a, K, V> {
    table: &'a SeparateChainingHashTable<K, V>,
    bucket_index: usize,
    current_node: Option<&'a Box<HashNode<K, V>>>,
}

impl<'a, K, V> Iterator for HashTableIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // Try to get next node in current bucket
        if let Some(node) = self.current_node {
            self.current_node = node.next.as_ref();
            return Some((&node.key, &node.value));
        }

        // Find next non-empty bucket
        while self.bucket_index < self.table.buckets.len() {
            if let Some(node) = &self.table.buckets[self.bucket_index] {
                self.current_node = Some(node);
                self.bucket_index += 1;
                return Some((&node.key, &node.value));
            }
            self.bucket_index += 1;
        }

        None
    }
}

fn demonstrate_separate_chaining() {
    println!("=== HASH TABLE WITH SEPARATE CHAINING ===");

    let mut table = SeparateChainingHashTable::new();
    println!("Created empty hash table with capacity: {}", table.capacity);

    // Insert key-value pairs
    let test_data = [
        ("apple", 5),
        ("banana", 3),
        ("orange", 7),
        ("grape", 2),
        ("pear", 4),
    ];

    println!("Inserting key-value pairs:");
    for (key, value) in &test_data {
        let old_value = table.insert(key.to_string(), *value);
        println!("  Inserted {}: {} (old: {:?})", key, value, old_value);
    }

    println!("Table size: {}", table.len());
    println!("Load factor: {:.2}", table.load_factor());

    // Get values
    println!("\nGetting values:");
    for (key, _) in &test_data {
        if let Some(value) = table.get(&key.to_string()) {
            println!("  {} = {}", key, value);
        }
    }

    // Update existing key
    println!("\nUpdating 'apple':");
    let old_value = table.insert("apple".to_string(), 10);
    println!("  Old value: {:?}", old_value);
    println!("  New value: {:?}", table.get(&"apple".to_string()));

    // Remove key
    println!("\nRemoving 'banana':");
    let removed_value = table.remove(&"banana".to_string());
    println!("  Removed value: {:?}", removed_value);
    println!("  Contains 'banana': {}", table.contains_key(&"banana".to_string()));

    // Iterator demonstration
    println!("\nIterating over all key-value pairs:");
    for (key, value) in table.iter() {
        println!("  {} = {}", key, value);
    }

    // Keys and values
    println!("\nKeys: {:?}", table.keys());
    println!("Values: {:?}", table.values());
    println!();
}

// ================================================================================
// 2. HASH TABLE WITH OPEN ADDRESSING (LINEAR PROBING)
// ================================================================================

#[derive(Debug, Clone)]
enum OpenAddressingEntry<K, V> {
    Occupied(K, V),
    Deleted,
    Empty,
}

#[derive(Debug)]
struct OpenAddressingHashTable<K, V> {
    table: Vec<OpenAddressingEntry<K, V>>,
    size: usize,
    capacity: usize,
    load_factor_threshold: f64,
}

impl<K: Hash + Eq + Clone, V: Clone> OpenAddressingHashTable<K, V> {
    fn new() -> Self {
        OpenAddressingHashTable::with_capacity(16)
    }

    fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        OpenAddressingHashTable {
            table: vec![OpenAddressingEntry::Empty; capacity],
            size: 0,
            capacity,
            load_factor_threshold: 0.7,
        }
    }

    fn hash_key(&self, key: &K, probe: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        (hash + probe) & (self.capacity - 1)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.should_resize() {
            self.resize();
        }

        for probe in 0..self.capacity {
            let index = self.hash_key(&key, probe);

            match &mut self.table[index] {
                OpenAddressingEntry::Empty => {
                    self.table[index] = OpenAddressingEntry::Occupied(key, value);
                    self.size += 1;
                    return None;
                }
                OpenAddressingEntry::Occupied(existing_key, existing_value) => {
                    if existing_key == &key {
                        let old_value = existing_value.clone();
                        *existing_value = value;
                        return Some(old_value);
                    }
                }
                OpenAddressingEntry::Deleted => {
                    self.table[index] = OpenAddressingEntry::Occupied(key, value);
                    self.size += 1;
                    return None;
                }
            }
        }

        panic!("Hash table is full even after resizing");
    }

    fn get(&self, key: &K) -> Option<&V> {
        for probe in 0..self.capacity {
            let index = self.hash_key(key, probe);

            match &self.table[index] {
                OpenAddressingEntry::Occupied(existing_key, existing_value) => {
                    if existing_key == key {
                        return Some(existing_value);
                    }
                }
                OpenAddressingEntry::Empty => {
                    return None;
                }
                OpenAddressingEntry::Deleted => {
                    continue;
                }
            }
        }

        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for probe in 0..self.capacity {
            let index = self.hash_key(key, probe);

            match &mut self.table[index] {
                OpenAddressingEntry::Occupied(existing_key, existing_value) => {
                    if existing_key == key {
                        return Some(existing_value);
                    }
                }
                OpenAddressingEntry::Empty => {
                    return None;
                }
                OpenAddressingEntry::Deleted => {
                    continue;
                }
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        for probe in 0..self.capacity {
            let index = self.hash_key(key, probe);

            match &mut self.table[index] {
                OpenAddressingEntry::Occupied(existing_key, _) => {
                    if existing_key == key {
                        let old_entry = mem::replace(&mut self.table[index], OpenAddressingEntry::Deleted);
                        self.size -= 1;
                        if let OpenAddressingEntry::Occupied(_, value) = old_entry {
                            return Some(value);
                        }
                    }
                }
                OpenAddressingEntry::Empty => {
                    return None;
                }
                OpenAddressingEntry::Deleted => {
                    continue;
                }
            }
        }

        None
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn should_resize(&self) -> bool {
        self.load_factor() > self.load_factor_threshold
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_table = OpenAddressingHashTable {
            table: vec![OpenAddressingEntry::Empty; new_capacity],
            size: 0,
            capacity: new_capacity,
            load_factor_threshold: self.load_factor_threshold,
        };

        for entry in &self.table {
            if let OpenAddressingEntry::Occupied(key, value) = entry {
                new_table.insert(key.clone(), value.clone());
            }
        }

        *self = new_table;
    }

    fn keys(&self) -> Vec<&K> {
        let mut keys = Vec::new();
        for entry in &self.table {
            if let OpenAddressingEntry::Occupied(key, _) = entry {
                keys.push(key);
            }
        }
        keys
    }
}

fn demonstrate_open_addressing() {
    println!("=== HASH TABLE WITH OPEN ADDRESSING ===");

    let mut table = OpenAddressingHashTable::new();
    println!("Created empty hash table with capacity: {}", table.capacity);

    // Insert key-value pairs
    let test_data = [
        ("cat", 1),
        ("dog", 2),
        ("bird", 3),
        ("fish", 4),
        ("lion", 5),
    ];

    println!("Inserting key-value pairs:");
    for (key, value) in &test_data {
        let old_value = table.insert(key.to_string(), *value);
        println!("  Inserted {}: {} (old: {:?})", key, value, old_value);
    }

    println!("Table size: {}", table.len());
    println!("Load factor: {:.2}", table.load_factor());

    // Get values
    println!("\nGetting values:");
    for (key, _) in &test_data {
        if let Some(value) = table.get(&key.to_string()) {
            println!("  {} = {}", key, value);
        }
    }

    // Remove key
    println!("\nRemoving 'dog':");
    let removed_value = table.remove(&"dog".to_string());
    println!("  Removed value: {:?}", removed_value);
    println!("  Contains 'dog': {}", table.contains_key(&"dog".to_string()));

    // Try to get removed key
    println!("  Get 'dog': {:?}", table.get(&"dog".to_string()));

    println!();
}

// ================================================================================
// 3. CUSTOM HASH FUNCTION
// ================================================================================

struct CustomHasher {
    state: u64,
}

impl CustomHasher {
    fn new() -> Self {
        CustomHasher { state: 0x123456789ABCDEF0 }
    }
}

impl Hasher for CustomHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

fn custom_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = CustomHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

fn demonstrate_custom_hash() {
    println!("=== CUSTOM HASH FUNCTION ===");

    let strings = ["hello", "world", "rust", "hash", "table"];

    println!("Custom hash values:");
    for s in &strings {
        let hash = custom_hash(s);
        println!("  {} -> {}", s, hash);
    }

    // Compare with default hasher
    println!("\nDefault hash values:");
    for s in &strings {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        println!("  {} -> {}", s, hasher.finish());
    }

    println!();
}

// ================================================================================
// 4. HASH TABLE WITH GENERIC HASH FUNCTION
// ================================================================================

struct GenericHashTable<K, V, H>
where
    H: Fn(&K) -> u64,
{
    table: Vec<Vec<(K, V)>>,
    hasher: H,
    size: usize,
}

impl<K, V, H> GenericHashTable<K, V, H>
where
    K: Eq + Clone,
    V: Clone,
    H: Fn(&K) -> u64,
{
    fn new(capacity: usize, hasher: H) -> Self {
        GenericHashTable {
            table: vec![Vec::new(); capacity.next_power_of_two()],
            hasher,
            size: 0,
        }
    }

    fn hash_key(&self, key: &K) -> usize {
        let hash = (self.hasher)(key);
        (hash as usize) & (self.table.len() - 1)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_key(&key);
        let bucket = &mut self.table[index];

        for (existing_key, existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                let old_value = existing_value.clone();
                *existing_value = value;
                return Some(old_value);
            }
        }

        bucket.push((key, value));
        self.size += 1;
        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        let bucket = &self.table[index];

        for (existing_key, existing_value) in bucket {
            if existing_key == key {
                return Some(existing_value);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_key(key);
        let bucket = &mut self.table[index];

        for (i, (existing_key, _)) in bucket.iter().enumerate() {
            if existing_key == key {
                self.size -= 1;
                return Some(bucket.remove(i).1);
            }
        }

        None
    }

    fn len(&self) -> usize {
        self.size
    }
}

fn demonstrate_generic_hash_table() {
    println!("=== GENERIC HASH TABLE ===");

    // Create hash table with simple hash function
    let mut table = GenericHashTable::new(16, |s: &String| {
        s.chars().map(|c| c as u64).sum()
    });

    // Insert some strings
    let words = ["apple", "banana", "orange", "grape", "pear"];

    println!("Inserting words:");
    for word in &words {
        table.insert(word.to_string(), word.len());
        println!("  {} -> {}", word, word.len());
    }

    // Look up words
    println!("\nLooking up words:");
    for word in &words {
        if let Some(length) = table.get(&word.to_string()) {
            println!("  {} length: {}", word, length);
        }
    }

    println!();
}

// ================================================================================
// 5. CUCKOO HASHING
// ================================================================================

struct CuckooHashTable<K, V> {
    tables: [Vec<Option<(K, V)>>; 2],
    hash_functions: [fn(&K) -> usize; 2],
    size: usize,
    capacity: usize,
}

impl<K: Eq + Clone, V: Clone> CuckooHashTable<K, V> {
    fn new(capacity: usize) -> Self {
        CuckooHashTable {
            tables: [vec![None; capacity], vec![None; capacity]],
            hash_functions: [
                |key| {
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    (hasher.finish() as usize) % capacity
                },
                |key| {
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    ((hasher.finish() >> 16) as usize) % capacity
                },
            ],
            size: 0,
            capacity,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), (K, V)> {
        let mut current_key = key;
        let mut current_value = value;
        let mut table_index = 0;

        for _ in 0..1000 { // Prevent infinite loops
            let index = self.hash_functions[table_index](&current_key) % self.capacity;

            if let Some(entry) = &mut self.tables[table_index][index] {
                if entry.0 == current_key {
                    entry.1 = current_value;
                    return Ok(());
                }
            } else {
                self.tables[table_index][index] = Some((current_key, current_value));
                self.size += 1;
                return Ok(());
            }

            // Evict current entry and continue
            let mut evicted = self.tables[table_index][index].take().unwrap();
            std::mem::swap(&mut evicted.0, &mut current_key);
            std::mem::swap(&mut evicted.1, &mut current_value);
            table_index = 1 - table_index; // Switch table
        }

        Err((current_key, current_value))
    }

    fn get(&self, key: &K) -> Option<&V> {
        for (table, hash_fn) in self.tables.iter().zip(self.hash_functions.iter()) {
            let index = hash_fn(key) % self.capacity;
            if let Some((existing_key, existing_value)) = &table[index] {
                if existing_key == key {
                    return Some(existing_value);
                }
            }
        }
        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        for (table, hash_fn) in self.tables.iter_mut().zip(self.hash_functions.iter()) {
            let index = hash_fn(key) % self.capacity;
            if let Some((existing_key, _)) = &table[index] {
                if existing_key == key {
                    self.size -= 1;
                    return table[index].take().map(|(_, value)| value);
                }
            }
        }
        None
    }

    fn len(&self) -> usize {
        self.size
    }
}

fn demonstrate_cuckoo_hashing() {
    println!("=== CUCKOO HASHING ===");

    let mut table = CuckooHashTable::new(8);
    println!("Created cuckoo hash table with capacity: {}", table.capacity);

    // Insert key-value pairs
    let test_data = [
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
    ];

    println!("Inserting key-value pairs:");
    for (key, value) in &test_data {
        match table.insert(key.to_string(), *value) {
            Ok(()) => println!("  Inserted {}: {}", key, value),
            Err((k, v)) => println!("  Failed to insert {}: {}", k, v),
        }
    }

    println!("Table size: {}", table.len());

    // Get values
    println!("\nGetting values:");
    for (key, _) in &test_data {
        if let Some(value) = table.get(&key.to_string()) {
            println!("  {} = {}", key, value);
        }
    }

    println!();
}

// ================================================================================
// 6. HASH TABLE APPLICATIONS
// ================================================================================

// Word frequency counter
fn word_frequency_counter(text: &str) -> std::collections::HashMap<String, usize> {
    let mut freq_map = std::collections::HashMap::new();

    for word in text.split_whitespace() {
        let clean_word = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        if !clean_word.is_empty() {
            *freq_map.entry(clean_word).or_insert(0) += 1;
        }
    }

    freq_map
}

// Two-sum problem solver
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = std::collections::HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }

    None
}

// LRU Cache implementation
struct LRUCache<K, V> {
    capacity: usize,
    map: std::collections::HashMap<K, (V, std::collections::LinkedList<K>)>,
    order: std::collections::LinkedList<K>,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: std::collections::HashMap::new(),
            order: std::collections::LinkedList::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, _)) = self.map.get_mut(key) {
            // Move to front
            self.order.push_back(key.clone());
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity && !self.map.contains_key(&key) {
            // Remove least recently used
            if let Some(lru_key) = self.order.pop_front() {
                self.map.remove(&lru_key);
            }
        }

        self.order.push_back(key.clone());
        self.map.insert(key, (value, self.order.clone()));
    }
}

fn demonstrate_hash_table_applications() {
    println!("=== HASH TABLE APPLICATIONS ===");

    // Word frequency counter
    let text = "Hello world! This is a test. Hello again, world! Rust is great.";
    let frequencies = word_frequency_counter(text);
    println!("Word frequencies:");
    for (word, count) in &frequencies {
        println!("  {}: {}", word, count);
    }

    // Two-sum problem
    let nums = [2, 7, 11, 15];
    let target = 9;
    if let Some((i, j)) = two_sum(&nums, target) {
        println!("\nTwo sum solution for {} with array {:?}: {} + {} = {}",
                 target, nums, nums[i], nums[j], target);
    }

    // LRU Cache demonstration
    let mut cache = LRUCache::new(3);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    println!("\nLRU Cache capacity: 3");
    println!("Inserted a:1, b:2, c:3");

    cache.put("d".to_string(), 4); // Should evict 'a'
    println!("Inserted d:4 (evicts a)");

    println!("\nCache contents:");
    for (key, (value, _)) in &cache.map {
        println!("  {}: {}", key, value);
    }

    println!();
}

// ================================================================================
// 7. HASH TABLE PERFORMANCE ANALYSIS
// ================================================================================

fn analyze_hash_table_performance() {
    println!("=== HASH TABLE PERFORMANCE ANALYSIS ===");

    // Test different load factors
    let load_factors = [0.25, 0.5, 0.75, 0.9];
    let test_size = 1000;

    for &load_factor in &load_factors {
        let capacity = (test_size as f64 / load_factor) as usize;
        let mut table = SeparateChainingHashTable::with_capacity(capacity);

        // Insert elements
        let start = std::time::Instant::now();
        for i in 0..test_size {
            table.insert(i, i * 2);
        }
        let insert_time = start.elapsed();

        // Lookup elements
        let start = std::time::Instant::now();
        for i in 0..test_size {
            table.get(&i);
        }
        let lookup_time = start.elapsed();

        println!("Load factor: {:.2}", load_factor);
        println!("  Insert time: {:?}", insert_time);
        println!("  Lookup time: {:?}", lookup_time);
        println!("  Bucket count: {}", table.buckets.len());
        println!("  Actual load factor: {:.2}", table.load_factor());
        println!();
    }
}

// ================================================================================
// MAIN FUNCTION - DEMO RUNNER
// ================================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           COMPREHENSIVE HASH TABLE DATA STRUCTURES         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    demonstrate_separate_chaining();
    demonstrate_open_addressing();
    demonstrate_custom_hash();
    demonstrate_generic_hash_table();
    demonstrate_cuckoo_hashing();
    demonstrate_hash_table_applications();
    analyze_hash_table_performance();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CONCLUSION                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("This implementation covers:");
    println!("• Separate chaining with linked lists");
    println!("• Open addressing with linear probing");
    println!("• Custom hash functions");
    println!("• Generic hash table with pluggable hashers");
    println!("• Cuckoo hashing for guaranteed lookup");
    println!("• Practical applications (frequency counter, two-sum, LRU cache)");
    println!("• Performance analysis at different load factors");
    println!();
    println!("Key concepts demonstrated:");
    println!("✅ Hash function design and collision handling");
    println!("✅ Different collision resolution strategies");
    println!("✅ Load factor management and resizing");
    println!("✅ Time complexity analysis (average O(1))");
    println!("✅ Space efficiency considerations");
    println!("✅ Iterator implementations");
    println!("✅ Real-world problem solving");
}

/*
================================================================================
HASH TABLE DATA STRUCTURES SUMMARY:
================================================================================

1. SEPARATE CHAINING:
   - Uses linked lists for collision resolution
   - Simple implementation and deletion
   - Memory overhead for linked list nodes
   - Good performance at high load factors

2. OPEN ADDRESSING:
   - Uses probing sequences for collision resolution
   - Better cache locality
   - More complex deletion logic
   - Sensitive to clustering

3. COLLISION RESOLUTION STRATEGIES:
   - Linear probing: Simple but prone to clustering
   - Quadratic probing: Better distribution
   - Double hashing: Excellent performance
   - Cuckoo hashing: O(1) worst-case lookup

4. HASH FUNCTIONS:
   - Cryptographic vs non-cryptographic
   - Uniform distribution requirements
   - Custom hash function implementation
   - Performance vs quality trade-offs

5. PERFORMANCE CHARACTERISTICS:
   - Average case: O(1) for insert, lookup, delete
   - Worst case: O(n) with poor hash function
   - Load factor impact on performance
   - Resizing overhead and amortization

6. APPLICATIONS:
   - Symbol tables in compilers
   - Database indexing
   - Caching systems
   - Frequency counting
   - Algorithm optimization (two-sum, etc.)

7. RUST-SPECIFIC FEATURES:
   - Generic implementations with trait bounds
   - Memory safety without runtime overhead
   - Built-in HashMap for production use
   - Custom hasher trait
   - Zero-cost abstractions

8. THREAD SAFETY:
   - Not inherently thread-safe
   - RwLock for read-heavy workloads
   - Concurrent hash map implementations
   - Lock-free designs with atomic operations

COMPILATION:
rustc 05-Hash_Tables.rs -o hash_tables_demo

USAGE:
./hash_tables_demo
================================================================================
*/