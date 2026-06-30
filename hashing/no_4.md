# Part 4 — Large Number Hashing, Ordered Maps, Unordered Maps, and HashMaps in Rust

Topics covered:

21. Large Number Hashing Problem
22. Map (Ordered Map)
23. Key-Value Storage
24. Frequency Counting with Map
25. Ordered Nature of Map
26. Iterating Through a Map
27. Time Complexity of Map
28. Unordered Map
29. Ordered vs Unordered Map
30. Average / Best / Worst Complexity

 

---

# 🔷 Topic 21: Large Number Hashing Problem

---

## 1. Conceptual Clarity

Earlier we used:

```rust
freq[value]
```

This works when:

```text
value <= 100000
```

or maybe:

```text
value <= 1000000
```

But suppose:

```text
nums = [1, 999999999, 42]
```

Now we need:

```rust
freq[999999999]
```

Impossible.

---

### Why?

Array hashing depends on:

```text
Maximum Value
```

not

```text
Number of Elements
```

Example:

```text
Array Size = 3

Values:
1
999999999
42
```

Only 3 numbers exist.

Yet array hashing requires:

```text
1 billion slots
```

Huge waste.

---

### The Core Problem

Array hashing assumes:

```text
Value = Address
```

But large values create huge addresses.

This is why maps were invented.

---

### C++ vs Rust

C++:

```cpp
unordered_map<int,int>
```

Rust:

```rust
HashMap<i32,i32>
```

These structures store only existing keys.

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn main() {
    let nums = vec![1, 999999999, 42];

    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    println!("{:?}", freq);
}
```

---

### Ownership Notes

Integers implement:

```rust
Copy
```

so insertion is cheap.

---

### Beginner Mistake

Trying:

```rust
vec![0; 1_000_000_000]
```

Consumes enormous memory.

---

## 3. Real World Applications

### Backend

User IDs:

```text
1
542351235
912351235
```

Sparse values.

HashMap required.

---

### Databases

Primary keys are not contiguous.

---

### Blockchain

Wallet balances:

```text
Address -> Balance
```

Addresses are huge.

---

### AI

Vocabulary IDs may be extremely sparse.

---

### Operating Systems

PIDs are sparse.

Maps dominate.

---

## 4. System-Level Understanding

HashMaps store:

```text
Only existing keys
```

instead of

```text
Every possible key
```

Massive memory savings.

---

## 5. Engineering Tradeoffs

Array:

```text
Fastest lookup
Worst memory
```

HashMap:

```text
Slightly slower
Much less memory
```

---

## 6. Exercises

Count frequencies for:

```rust
vec![1, 1_000_000_000, 1, 42]
```

---

# 🔷 Topic 22: Map (Ordered Map)

---

## 1. Conceptual Clarity

The lecture introduces:

```text
Map
```

A map stores:

```text
Key -> Value
```

Example:

```text
1 -> 2
2 -> 2
3 -> 1
```

Meaning:

```text
Number 1 appears twice
Number 2 appears twice
Number 3 appears once
```

Unlike arrays:

```text
Index = Key
```

is not required.

---

### Ordered Map

An ordered map automatically keeps keys sorted.

Example:

Insert:

```text
5
1
9
2
```

Stored internally:

```text
1
2
5
9
```

---

### Data Structure Behind It

Most languages implement ordered maps using:

```text
Balanced Binary Search Trees
```

Usually:

```text
Red-Black Tree
```

---

### Rust Equivalent

Rust does NOT call it Map.

Rust uses:

```rust
BTreeMap
```

for ordered maps.

---

## 2. Rust Implementation

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();

    map.insert(5, 100);
    map.insert(1, 50);
    map.insert(9, 200);

    println!("{:?}", map);
}
```

Output:

```text
{1:50, 5:100, 9:200}
```

Sorted.

---

### Beginner Mistake

Thinking:

```rust
HashMap
```

is ordered.

It is not.

---

## 3. Real World Applications

### Databases

Indexes.

### Storage Engines

Sorted key-value stores.

### Blockchain

State trees.

### Operating Systems

Scheduling structures.

---

## 4. System-Level Understanding

Balanced trees:

```text
O(log N)
```

for:

* insert
* delete
* search

because tree height remains small.

---

## 5. Engineering Tradeoffs

Advantages:

✅ Sorted order

✅ Range queries

Disadvantages:

❌ Slower than HashMap

---

## 6. Exercises

Store:

```text
10,3,7,1
```

inside BTreeMap.

Observe iteration order.

---

# 🔷 Topic 23: Key-Value Storage

---

## 1. Conceptual Clarity

Map stores:

```text
Key -> Value
```

Think of a dictionary.

Example:

```text
Name -> Age

Alice -> 22
Bob -> 19
```

Or:

```text
Number -> Frequency

1 -> 2
2 -> 5
```

---

### Why Important?

Nearly every modern backend service is built on key-value access.

---

### Examples

Redis:

```text
SessionID -> Session
```

DNS:

```text
Domain -> IP
```

Cache:

```text
Key -> Cached Result
```

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();

    ages.insert("Alice", 22);
    ages.insert("Bob", 19);
}
```

---

### Ownership Notes

String keys:

```rust
String
```

are moved into map.

Need borrowing or cloning carefully.

---

## 3. Real World Applications

Everywhere.

Literally everywhere.

---

## 4. System-Level Understanding

Key-value storage is the foundation of:

* Redis
* Memcached
* DynamoDB
* Cassandra

---

## 5. Engineering Tradeoffs

Key-value systems:

✅ Simple

✅ Fast

❌ Weak for complex relationships

---

## 6. Exercises

Create:

```text
Country -> Capital
```

map.

---

# 🔷 Topic 24: Frequency Counting with Map

---

## 1. Conceptual Clarity

Lecture's frequency counting pattern:

```cpp
map[arr[i]]++
```

becomes in Rust:

```rust
*map.entry(arr[i]).or_insert(0) += 1;
```

This is probably the most important HashMap pattern in DSA.

---

### What Happens?

Suppose:

```text
1
2
1
```

Process:

```text
1 -> count=1
2 -> count=1
1 -> count=2
```

Final:

```text
1 -> 2
2 -> 1
```

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn frequency(nums: &[i32]) -> HashMap<i32, usize> {
    let mut map = HashMap::new();

    for &num in nums {
        *map.entry(num).or_insert(0) += 1;
    }

    map
}
```

---

### Why `entry()`?

Avoids:

```rust
contains_key()
get()
insert()
```

Multiple lookups.

Only one lookup.

---

### Beginner Mistake

```rust
map.insert(num, 1);
```

overwrites frequency.

---

## 3. Real World Applications

Analytics.

Monitoring.

Logging.

Metrics systems.

---

## 4. System-Level Understanding

Frequency counting is essentially:

```text
Aggregation
```

Databases do this constantly.

---

## 5. Engineering Tradeoffs

HashMap:

```text
Fast
```

Array:

```text
Faster
```

when possible.

---

## 6. Exercises

Count:

```text
[5,5,7,5,7]
```

using HashMap.

---

# 🔷 Topic 25: Ordered Nature of Map

---

## 1. Conceptual Clarity

Lecture mentions:

Map stores elements in sorted order. 

Example:

Insert:

```text
10
3
7
1
```

Stored:

```text
1
3
7
10
```

---

### Why Useful?

Allows:

```text
Smallest Key
Largest Key
Range Queries
```

efficiently.

---

## 2. Rust Example

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();

map.insert(10, 1);
map.insert(3, 1);
map.insert(7, 1);
```

Iterating:

```text
3
7
10
```

Sorted automatically.

---

## 3. Real World Applications

Database indexes.

Filesystem indexes.

Blockchain state trees.

---

## 4. System-Level Understanding

Sorted trees support:

```text
lower_bound
upper_bound
range
```

operations efficiently.

HashMap cannot.

---

## 5. Engineering Tradeoffs

Ordered map:

```text
O(log N)
```

HashMap:

```text
O(1)
```

average.

---

## 6. Exercises

Print keys in ascending order.

---

# 🔷 Topic 26: Iterating Through a Map

---

## 1. Conceptual Clarity

Lecture shows:

```cpp
for(auto it : map)
```

Iterating key-value pairs. 

---

## 2. Rust Implementation

HashMap:

```rust
for (key, value) in &map {
    println!("{} {}", key, value);
}
```

BTreeMap:

```rust
for (key, value) in &btree {
    println!("{} {}", key, value);
}
```

---

### Ownership Notes

Use:

```rust
&map
```

to borrow.

Avoid moving.

---

### Beginner Mistake

```rust
for (k,v) in map
```

Consumes map.

Cannot use afterward.

---

## 3. Real World Applications

Metrics collection.

Report generation.

Analytics.

---

## 4. System-Level Understanding

Iteration complexity:

```text
O(N)
```

Need to visit every entry.

---

## 5. Engineering Tradeoffs

HashMap:

```text
Random order
```

BTreeMap:

```text
Sorted order
```

---

## 6. Exercises

Print all frequencies from a HashMap.

---

# 🔷 Topic 27: Time Complexity of Map

---

## 1. Conceptual Clarity

Ordered Map (BTreeMap):

Search:

```text
O(log N)
```

Insert:

```text
O(log N)
```

Delete:

```text
O(log N)
```

Lecture emphasizes this. 

---

### Why?

Balanced tree height:

```text
log N
```

---

## 2. Rust Example

```rust
btree.insert(key, value);
```

Internally:

```text
Tree traversal
```

---

## 3. Real World Applications

Storage engines.

Database indexes.

---

## 4. System-Level Understanding

Tree nodes:

```text
Pointer chasing
```

causes cache misses.

This is why HashMaps are often faster.

---

## 5. Engineering Tradeoffs

Need ordering?

Use BTreeMap.

Need speed?

Use HashMap.

---

## 6. Exercises

Compare:

```rust
HashMap
BTreeMap
```

on 100,000 inserts.

---

# 🔷 Topic 28: Unordered Map

---

## 1. Conceptual Clarity

Lecture introduces:

```text
unordered_map
```

which corresponds to Rust's:

```rust
HashMap
```

This is hashing-based storage. 

---

### Key Property

No ordering.

Insert:

```text
5
1
9
2
```

May print:

```text
9
2
5
1
```

or something else.

---

## 2. Rust Example

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(5, 1);
map.insert(1, 1);
map.insert(9, 1);
```

Iteration order is unpredictable.

---

## 3. Real World Applications

Caching.

Analytics.

Backend APIs.

---

## 4. System-Level Understanding

Uses:

```text
Hash Function
Buckets
```

instead of trees.

---

## 5. Engineering Tradeoffs

Advantages:

✅ Faster average lookup

Disadvantages:

❌ No ordering

---

## 6. Exercises

Insert keys.

Observe order after printing.

---

# 🔷 Topic 29: Ordered vs Unordered Map

---

## 1. Conceptual Clarity

| Feature       | Ordered Map | HashMap    |
| ------------- | ----------- | ---------- |
| Structure     | Tree        | Hash Table |
| Ordering      | Sorted      | Unordered  |
| Lookup        | O(logN)     | O(1) avg   |
| Range Queries | Yes         | No         |

---

### Rule of Thumb

Need:

```text
Sorted keys?
```

Use:

```rust
BTreeMap
```

Need:

```text
Fast lookup?
```

Use:

```rust
HashMap
```

---

## 2. Rust Comparison

```rust
use std::collections::{HashMap, BTreeMap};
```

Both solve frequency counting.

Different tradeoffs.

---

## 3. Real World Applications

BTreeMap:

* databases
* filesystems

HashMap:

* caches
* APIs
* analytics

---

## 4. System-Level Understanding

HashMap:

```text
CPU friendly
```

BTreeMap:

```text
Ordering friendly
```

---

## 5. Engineering Tradeoffs

90% of DSA problems:

```rust
HashMap
```

wins.

---

## 6. Exercises

Solve frequency counting with both.

Compare performance.

---

# 🔷 Topic 30: Average / Best / Worst Complexity

---

## 1. Conceptual Clarity

Lecture highlights:

HashMap is not always O(1). 

---

### Best Case

```text
O(1)
```

Perfect hash distribution.

---

### Average Case

```text
O(1)
```

What happens in practice.

---

### Worst Case

```text
O(N)
```

All keys collide.

---

Example:

```text
1
11
21
31
41
```

all land in same bucket.

Search becomes:

```text
Linear Search
```

---

## 2. Rust Perspective

Rust HashMap uses strong hashing algorithms.

Worst case is extremely rare.

---

### Beginner Mistake

Assuming:

```text
HashMap always O(1)
```

Interviewers expect:

```text
Average O(1)
Worst O(N)
```

---

## 3. Real World Applications

Security-sensitive systems care about collision attacks.

---

## 4. System-Level Understanding

Good hash function:

```text
Even bucket distribution
```

Bad hash function:

```text
Many collisions
```

Performance collapses.

---

## 5. Engineering Tradeoffs

HashMap:

```text
Fast on average
```

BTreeMap:

```text
Predictable O(logN)
```

---

## 6. Exercises

Research:

```text
Hash Collision Attack
```

and why web servers defend against it.

---

# Part 4 Summary

| Topic                | Rust Structure   | Complexity     |
| -------------------- | ---------------- | -------------- |
| Large Number Hashing | HashMap          | O(1) avg       |
| Ordered Map          | BTreeMap         | O(logN)        |
| Key-Value Storage    | HashMap/BTreeMap | depends        |
| Frequency Counting   | HashMap          | O(N) build     |
| Ordered Nature       | BTreeMap         | Sorted         |
| Map Iteration        | Both             | O(N)           |
| Map Complexity       | BTreeMap         | O(logN)        |
| Unordered Map        | HashMap          | O(1) avg       |
| Ordered vs Unordered | Tree vs Hash     | Tradeoff       |
| Best/Average/Worst   | HashMap          | O(1)/O(1)/O(N) |

### Interview Shortcut

For Rust DSA:

```rust
use std::collections::HashMap;
```

Use this by default.

Switch to:

```rust
use std::collections::BTreeMap;
```

only when the problem requires:

* sorted order
* smallest/largest key repeatedly
* range queries

**Part 5** is where the real hashing internals start:

* Internal Hashing Mechanism
* Hash Functions
* Division Method
* Modulo-Based Hashing
* Collisions
* Chaining
* Why Worst Case Becomes O(N)
* How Hash Tables Actually Work Under the Hood.
