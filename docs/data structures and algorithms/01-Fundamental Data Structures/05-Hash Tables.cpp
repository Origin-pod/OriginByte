#pragma once
#include <vector>
#include <list>
#include <functional>
#include <stdexcept>
#include <string>

/**
 * @brief Hash Table Implementation using Separate Chaining
 *
 * Hash table (hash map) is a data structure that maps keys to values for
 * highly efficient lookup. It uses a hash function to compute an index into
 * an array of buckets or slots, from which the desired value can be found.
 *
 * Time Complexity (Average Case):
 * - Insert: O(1)
 * - Delete: O(1)
 * - Search: O(1)
 *
 * Time Complexity (Worst Case):
 * - Insert: O(n)
 * - Delete: O(n)
 * - Search: O(n)
 *
 * Applications:
 * - Database indexing
 * - Caches
 * - Symbol tables in compilers
 * - Associative arrays
 */
template <typename K, typename V>
class HashTable {
private:
    /**
     * @brief Individual key-value pair
     */
    struct KeyValuePair {
        K key;
        V value;

        KeyValuePair(const K& k, const V& v) : key(k), value(v) {}
    };

    std::vector<std::list<KeyValuePair>> buckets;  // Array of linked lists
    size_t num_elements;                           // Number of stored elements
    size_t num_buckets;                            // Number of buckets
    const double max_load_factor = 0.75;           // Maximum load factor before rehashing

    /**
     * @brief Hash function for keys
     * @param key Key to hash
     * @return Hash value
     */
    size_t hash(const K& key) const {
        // Use std::hash which works for built-in types and can be specialized
        return std::hash<K>{}(key) % num_buckets;
    }

    /**
     * @brief Resize and rehash all elements when load factor is exceeded
     */
    void rehash() {
        std::vector<std::list<KeyValuePair>> old_buckets = std::move(buckets);
        size_t old_num_buckets = num_buckets;

        // Double the number of buckets
        num_buckets *= 2;
        buckets.clear();
        buckets.resize(num_buckets);
        num_elements = 0;

        // Reinsert all elements into new buckets
        for (size_t i = 0; i < old_num_buckets; ++i) {
            for (const auto& pair : old_buckets[i]) {
                insert(pair.key, pair.value);
            }
        }
    }

    /**
     * @brief Find bucket and iterator for a key
     * @param key Key to find
     * @param bucket_idx Output parameter for bucket index
     * @param it Output parameter for iterator to key-value pair
     * @return true if key found, false otherwise
     */
    bool find_with_iterator(const K& key, size_t& bucket_idx,
                           typename std::list<KeyValuePair>::iterator& it) {
        bucket_idx = hash(key);
        auto& bucket = buckets[bucket_idx];

        for (it = bucket.begin(); it != bucket.end(); ++it) {
            if (it->key == key) {
                return true;
            }
        }

        return false;
    }

public:
    /**
     * @brief Constructor with initial number of buckets
     * @param initial_size Initial number of buckets
     */
    HashTable(size_t initial_size = 16) : num_elements(0), num_buckets(initial_size) {
        buckets.resize(num_buckets);
    }

    /**
     * @brief Insert or update key-value pair
     * @param key Key to insert
     * @param value Value to associate with key
     */
    void insert(const K& key, const V& value) {
        // Check if we need to rehash
        if (static_cast<double>(num_elements) / num_buckets > max_load_factor) {
            rehash();
        }

        size_t bucket_idx = hash(key);
        auto& bucket = buckets[bucket_idx];

        // Check if key already exists
        for (auto& pair : bucket) {
            if (pair.key == key) {
                pair.value = value;  // Update existing value
                return;
            }
        }

        // Insert new key-value pair
        bucket.emplace_front(key, value);
        num_elements++;
    }

    /**
     * @brief Remove key-value pair
     * @param key Key to remove
     * @return true if key was removed, false if key didn't exist
     */
    bool remove(const K& key) {
        size_t bucket_idx = hash(key);
        auto& bucket = buckets[bucket_idx];

        for (auto it = bucket.begin(); it != bucket.end(); ++it) {
            if (it->key == key) {
                bucket.erase(it);
                num_elements--;
                return true;
            }
        }

        return false;  // Key not found
    }

    /**
     * @brief Get value associated with key
     * @param key Key to search for
     * @return Reference to value
     */
    V& get(const K& key) {
        size_t bucket_idx;
        typename std::list<KeyValuePair>::iterator it;

        if (find_with_iterator(key, bucket_idx, it)) {
            return it->value;
        }

        throw std::out_of_range("Key not found in hash table");
    }

    /**
     * @brief Get value associated with key (const version)
     */
    const V& get(const K& key) const {
        size_t bucket_idx = hash(key);
        const auto& bucket = buckets[bucket_idx];

        for (const auto& pair : bucket) {
            if (pair.key == key) {
                return pair.value;
            }
        }

        throw std::out_of_range("Key not found in hash table");
    }

    /**
     * @brief Check if key exists in hash table
     * @param key Key to check
     * @return true if key exists, false otherwise
     */
    bool contains(const K& key) const {
        size_t bucket_idx = hash(key);
        const auto& bucket = buckets[bucket_idx];

        for (const auto& pair : bucket) {
            if (pair.key == key) {
                return true;
            }
        }

        return false;
    }

    /**
     * @brief Get number of stored elements
     */
    size_t size() const {
        return num_elements;
    }

    /**
     * @brief Check if hash table is empty
     */
    bool is_empty() const {
        return num_elements == 0;
    }

    /**
     * @brief Get current load factor
     */
    double load_factor() const {
        return static_cast<double>(num_elements) / num_buckets;
    }

    /**
     * @brief Clear all elements
     */
    void clear() {
        for (auto& bucket : buckets) {
            bucket.clear();
        }
        num_elements = 0;
    }

    /**
     * @brief Get all keys in hash table
     */
    std::vector<K> get_keys() const {
        std::vector<K> keys;
        for (const auto& bucket : buckets) {
            for (const auto& pair : bucket) {
                keys.push_back(pair.key);
            }
        }
        return keys;
    }

    /**
     * @brief Get all values in hash table
     */
    std::vector<V> get_values() const {
        std::vector<V> values;
        for (const auto& bucket : buckets) {
            for (const auto& pair : bucket) {
                values.push_back(pair.value);
            }
        }
        return values;
    }

    /**
     * @brief Print hash table contents (for debugging)
     */
    void print() const {
        std::cout << "Hash Table (size: " << num_elements << ", buckets: " << num_buckets << "):" << std::endl;
        for (size_t i = 0; i < num_buckets; ++i) {
            const auto& bucket = buckets[i];
            if (!bucket.empty()) {
                std::cout << "  Bucket " << i << ": ";
                for (const auto& pair : bucket) {
                    std::cout << "[" << pair.key << ": " << pair.value << "] ";
                }
                std::cout << std::endl;
            }
        }
    }

    /**
     * @brief Subscript operator for easy access
     */
    V& operator[](const K& key) {
        size_t bucket_idx;
        typename std::list<KeyValuePair>::iterator it;

        if (find_with_iterator(key, bucket_idx, it)) {
            return it->value;
        } else {
            // Insert default value and return reference
            insert(key, V{});
            return get(key);
        }
    }

    /**
     * @brief Const subscript operator
     */
    const V& operator[](const K& key) const {
        return get(key);
    }
};

/**
 * @brief Hash Table with Open Addressing (Linear Probing)
 *
 * Alternative implementation using open addressing with linear probing.
 * All entries are stored in the bucket array itself.
 */
template <typename K, typename V>
class OpenAddressingHashTable {
private:
    enum class EntryState {
        EMPTY,
        OCCUPIED,
        DELETED
    };

    struct Entry {
        K key;
        V value;
        EntryState state;

        Entry() : state(EntryState::EMPTY) {}
        Entry(const K& k, const V& v) : key(k), value(v), state(EntryState::OCCUPIED) {}
    };

    std::vector<Entry> entries;
    size_t num_elements;
    size_t num_buckets;
    const double max_load_factor = 0.7;

    size_t hash(const K& key) const {
        return std::hash<K>{}(key) % num_buckets;
    }

    size_t probe(size_t index) const {
        return (index + 1) % num_buckets;
    }

    void rehash() {
        std::vector<Entry> old_entries = std::move(entries);
        size_t old_num_buckets = num_buckets;

        num_buckets *= 2;
        entries.clear();
        entries.resize(num_buckets);
        num_elements = 0;

        for (size_t i = 0; i < old_num_buckets; ++i) {
            if (old_entries[i].state == EntryState::OCCUPIED) {
                insert(old_entries[i].key, old_entries[i].value);
            }
        }
    }

public:
    OpenAddressingHashTable(size_t initial_size = 16)
        : num_elements(0), num_buckets(initial_size) {
        entries.resize(num_buckets);
    }

    bool insert(const K& key, const V& value) {
        if (static_cast<double>(num_elements) / num_buckets > max_load_factor) {
            rehash();
        }

        size_t index = hash(key);
        size_t start_index = index;

        do {
            if (entries[index].state == EntryState::EMPTY) {
                entries[index] = Entry(key, value);
                num_elements++;
                return true;
            } else if (entries[index].state == EntryState::OCCUPIED &&
                      entries[index].key == key) {
                entries[index].value = value;  // Update existing
                return true;
            }
            index = probe(index);
        } while (index != start_index);

        // Table is full (shouldn't happen with rehashing)
        return false;
    }

    bool remove(const K& key) {
        size_t index = hash(key);
        size_t start_index = index;

        do {
            if (entries[index].state == EntryState::EMPTY) {
                return false;  // Key not found
            } else if (entries[index].state == EntryState::OCCUPIED &&
                      entries[index].key == key) {
                entries[index].state = EntryState::DELETED;
                num_elements--;
                return true;
            }
            index = probe(index);
        } while (index != start_index);

        return false;
    }

    bool contains(const K& key) const {
        size_t index = hash(key);
        size_t start_index = index;

        do {
            if (entries[index].state == EntryState::EMPTY) {
                return false;
            } else if (entries[index].state == EntryState::OCCUPIED &&
                      entries[index].key == key) {
                return true;
            }
            index = probe(index);
        } while (index != start_index);

        return false;
    }

    V& get(const K& key) {
        size_t index = hash(key);
        size_t start_index = index;

        do {
            if (entries[index].state == EntryState::OCCUPIED &&
                entries[index].key == key) {
                return entries[index].value;
            } else if (entries[index].state == EntryState::EMPTY) {
                break;
            }
            index = probe(index);
        } while (index != start_index);

        throw std::out_of_range("Key not found");
    }

    size_t size() const { return num_elements; }
    bool is_empty() const { return num_elements == 0; }
    double load_factor() const { return static_cast<double>(num_elements) / num_buckets; }
};

/**
 * @brief Hash Table Applications and Examples
 */

/**
 * @brief Count frequency of elements in array
 * @param arr Input array
 * @return Hash table with element frequencies
 */
template <typename T>
HashTable<T, int> count_frequencies(const std::vector<T>& arr) {
    HashTable<T, int> freq_table;

    for (const T& element : arr) {
        if (freq_table.contains(element)) {
            freq_table.get(element)++;
        } else {
            freq_table.insert(element, 1);
        }
    }

    return freq_table;
}

/**
 * @brief Find first non-repeating character in string
 * @param str Input string
 * @return First non-repeating character, or '\0' if none
 */
char first_non_repeating_char(const std::string& str) {
    HashTable<char, int> count_table;

    // Count occurrences
    for (char ch : str) {
        if (count_table.contains(ch)) {
            count_table.get(ch)++;
        } else {
            count_table.insert(ch, 1);
        }
    }

    // Find first character with count 1
    for (char ch : str) {
        if (count_table.get(ch) == 1) {
            return ch;
        }
    }

    return '\0';  // No non-repeating character
}

/**
 * @brief Check if two strings are anagrams
 * @param str1 First string
 * @param str2 Second string
 * @return true if strings are anagrams
 */
bool are_anagrams(const std::string& str1, const std::string& str2) {
    if (str1.length() != str2.length()) {
        return false;
    }

    HashTable<char, int> count_table;

    // Count characters in first string
    for (char ch : str1) {
        if (count_table.contains(ch)) {
            count_table.get(ch)++;
        } else {
            count_table.insert(ch, 1);
        }
    }

    // Subtract characters from second string
    for (char ch : str2) {
        if (count_table.contains(ch)) {
            count_table.get(ch)--;
            if (count_table.get(ch) == 0) {
                count_table.remove(ch);
            }
        } else {
            return false;  // Character not found
        }
    }

    return count_table.is_empty();
}