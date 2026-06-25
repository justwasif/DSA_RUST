# part-1

### Introduction to Hashing
### Frequency Counting Problem
### Brute Force Frequency Counting
### Time Complexity Analysis (Q × N)
### Motivation for Hashing
### Precomputation & Query Answering
---
# Part 1 — Foundations of Hashing (Rust + Systems Perspective)

Topics covered from the lecture:

1. Introduction to Hashing
2. Frequency Counting Problem
3. Brute Force Frequency Counting
4. Time Complexity Analysis (Q × N)
5. Motivation for Hashing
6. Precomputation & Query Answering

Source: Lecture transcript on Hashing, Maps, Time Complexity, and Collisions. 

---

# 🔷 Topic 1: Introduction to Hashing

---

## 1. Conceptual Clarity

Hashing is a technique used to transform data into a form that can be accessed extremely quickly.

Think of a library:

Without hashing:

```
Find book:
Search shelf 1
Search shelf 2
Search shelf 3
...
```

With hashing:

```
Book ID -> Shelf Number
Go directly to shelf
```

The central idea:

```
Key -> Hash Function -> Location
```

Example:

```text
Value = 42

hash(42) = 5

Store at position 5
```

Instead of searching through every element, we jump directly to where information is stored.

---

### Why Hashing Exists

Suppose:

```text
Array = [1,2,1,3,2]
```

Question:

```text
How many times does 2 appear?
```

Searching every time is slow.

Hashing stores answers beforehand.

---

### Time Complexity

| Operation        | Without Hashing | With Hashing |
| ---------------- | --------------- | ------------ |
| Search           | O(N)            | O(1) average |
| Frequency Query  | O(N)            | O(1)         |
| Repeated Queries | O(QN)           | O(N+Q)       |

---

### C++ vs Rust

In C++:

```cpp
unordered_map<int,int>
```

In Rust:

```rust
HashMap<i32, i32>
```

Rust provides:

* ownership safety
* no dangling references
* thread-safe abstractions

C++ lets you shoot yourself in the foot.

Rust tries to stop you.

---

## 2. Rust Implementation

Simple hash map:

```rust
use std::collections::HashMap;

fn main() {
    let mut freq = HashMap::new();

    freq.insert(1, 2);

    println!("{:?}", freq);
}
```

---

### Ownership Discussion

HashMap owns its keys and values.

```rust
let mut map = HashMap::new();

map.insert(String::from("hello"), 1);
```

Map owns `"hello"`.

You cannot use the moved String afterward.

---

### Beginner Mistake

Wrong:

```rust
map[&key] += 1;
```

May panic if key doesn't exist.

Correct:

```rust
*map.entry(key).or_insert(0) += 1;
```

---

## 3. Real World Applications

### Backend

Rate limiting:

```text
IP -> Request Count
```

Stored using hashing.

---

### Databases

Database page cache:

```text
Page ID -> Memory Location
```

Uses hash tables.

---

### Blockchain

Ethereum nodes:

```text
Account -> State
```

Fast lookup via hashing structures.

---

### AI Systems

Token frequency:

```text
word -> count
```

Hash maps everywhere.

---

### Operating Systems

Kernel caches:

```text
File ID -> Metadata
```

Implemented using hashing.

---

## 4. System-Level Understanding

Hash tables reduce:

```text
CPU Instructions
Memory Traversal
Cache Misses
```

Compared to linear search.

CPU prefers:

```text
Direct lookup
```

over

```text
Scan 1 million entries
```

---

### Production Examples

* Linux Kernel hash tables
* Redis dictionaries
* RocksDB caches
* PostgreSQL buffer manager

---

## 5. Engineering Tradeoffs

Advantages:

✅ Fast lookup

✅ Fast insert

✅ Fast delete

Problems:

❌ Memory overhead

❌ Collisions

❌ Poor cache locality compared to arrays

---

## 6. Exercises

### Rust Exercise

Build:

```rust
HashMap<String, usize>
```

that counts words in a sentence.

---

### System Design

Design:

```text
Username -> User Profile
```

for 100 million users.

Would hashing work?

---

### Optimization Idea

What happens when:

```text
1 billion keys
```

must fit in RAM?

Research:

```text
Distributed Hash Tables (DHT)
```

---

# 🔷 Topic 2: Frequency Counting Problem

---

## 1. Conceptual Clarity

Problem:

```text
Array = [1,2,1,3,2]

Queries:

Count(1)
Count(2)
Count(3)
```

Expected:

```text
1 -> 2
2 -> 2
3 -> 1
```

This is one of the most common interview patterns.

---

### Core Observation

You repeatedly ask:

```text
How many times does X appear?
```

The naive approach recalculates every time.

Hashing stores answers once.

---

### Complexity

Without preprocessing:

```text
Every query = O(N)
```

With preprocessing:

```text
Build frequency = O(N)

Answer query = O(1)
```

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn frequencies(nums: &[i32]) -> HashMap<i32, usize> {
    let mut freq = HashMap::new();

    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    freq
}
```

---

### Why `&num`?

Loop variable:

```rust
for &num in nums
```

copies integer value.

Integers implement `Copy`.

No ownership issues.

---

### Beginner Mistake

```rust
freq.insert(num, 1);
```

This overwrites previous counts.

Need:

```rust
+= 1
```

---

## 3. Real World Applications

Backend:

```text
Endpoint hit count
```

Database:

```text
Query frequency tracking
```

Blockchain:

```text
Transaction frequency analysis
```

AI:

```text
Word frequencies
```

OS:

```text
Process event counts
```

---

## 4. System-Level Understanding

Frequency counting is:

```text
Streaming Friendly
```

Data arrives:

```text
increment count
increment count
increment count
```

No need to store entire history.

---

## 5. Engineering Tradeoffs

Store all data:

```text
Huge memory
```

Store counts:

```text
Tiny memory
```

---

## 6. Exercises

Count:

```rust
['a','b','a','c']
```

using HashMap.

---

System Design:

Design a service counting:

```text
API requests per minute.
```

---

# 🔷 Topic 3: Brute Force Frequency Counting

---

## 1. Conceptual Clarity

Lecture's original approach:

For every query:

```text
Scan entire array.
```

Example:

```text
Count(1)

1 ✓
2 ✗
1 ✓
3 ✗
2 ✗
```

Answer:

```text
2
```

---

### Complexity

One query:

```text
O(N)
```

Q queries:

```text
O(QN)
```

---

## 2. Rust Implementation

```rust
fn count_occurrences(nums: &[i32], target: i32) -> usize {
    let mut count = 0;

    for &num in nums {
        if num == target {
            count += 1;
        }
    }

    count
}
```

---

### Ownership

Notice:

```rust
nums: &[i32]
```

Borrowing.

No copy of array.

Efficient.

---

### Beginner Mistake

Passing:

```rust
Vec<i32>
```

instead of

```rust
&[i32]
```

causes unnecessary moves.

---

## 3. Real World Applications

Rarely used directly.

Useful when:

* small datasets
* one query only

---

## 4. System-Level Understanding

Excellent cache locality.

Sequential scan:

```text
1
2
3
4
5
```

CPU loves this.

Sometimes faster than a hash table for tiny inputs.

---

## 5. Engineering Tradeoffs

For:

```text
N = 10
```

Brute force wins.

For:

```text
N = 1,000,000
```

Hashing wins.

---

## 6. Exercises

Implement brute force count.

Compare against HashMap.

Measure runtime.

---

# 🔷 Topic 4: Time Complexity Analysis (Q × N)

---

## 1. Conceptual Clarity

If:

```text
Array Size = N
Queries = Q
```

Then:

```text
For every query:
    scan N elements
```

Total:

```text
O(Q × N)
```

Lecture example:

```text
N = 100000
Q = 100000
```

Operations:

```text
10^10
```

Too large. 

---

## 2. Rust Illustration

```rust
for query in queries {
    count_occurrences(&nums, query);
}
```

Nested work:

```text
Q × N
```

---

## 3. Real World Applications

Bad examples:

* querying logs repeatedly
* searching transactions repeatedly

---

## 4. System-Level Understanding

10¹⁰ operations:

```text
CPU becomes bottleneck
```

Even modern servers struggle.

---

## 5. Engineering Tradeoffs

When:

```text
Q is small
```

Brute force okay.

When:

```text
Q is huge
```

Need preprocessing.

---

## 6. Exercises

Compute:

```text
N = 1e6
Q = 1e6
```

Estimate runtime.

---

# 🔷 Topic 5: Motivation for Hashing

---

## 1. Conceptual Clarity

Hashing converts:

```text
Repeated expensive work
```

into

```text
One-time preprocessing
```

Idea:

```text
Work hard once
Answer quickly forever
```

---

## 2. Rust Example

```rust
let freq = build_frequency(&nums);

println!("{}", freq[&1]);
println!("{}", freq[&2]);
println!("{}", freq[&3]);
```

No scanning.

---

## 3. Real World Applications

Backend:

```text
Session Cache
```

Database:

```text
Buffer Cache
```

Blockchain:

```text
Account Lookup
```

AI:

```text
Vocabulary Index
```

---

## 4. System-Level Understanding

Trading:

```text
More Memory
```

for

```text
Less CPU
```

A classic systems tradeoff.

---

## 5. Engineering Tradeoffs

Hashing is preferred when:

```text
Many lookups
Few updates
```

---

## 6. Exercises

Identify 5 places in software where caching is used.

Most are actually hashing.

---

# 🔷 Topic 6: Precomputation & Query Answering

---

## 1. Conceptual Clarity

This is the most important idea from the lecture.

Two phases:

### Phase 1

Precompute

```text
Build frequencies
```

### Phase 2

Answer queries

```text
Lookup count instantly
```

---

Example:

```text
Array:

[1,2,1,3,2]
```

Precompute:

```text
1 -> 2
2 -> 2
3 -> 1
```

Queries:

```text
Count(1)
Count(2)
Count(3)
```

Answered instantly.

---

### Complexity

Build:

```text
O(N)
```

Query:

```text
O(1)
```

Total:

```text
O(N + Q)
```

instead of

```text
O(QN)
```

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn build_frequency(nums: &[i32]) -> HashMap<i32, usize> {
    let mut freq = HashMap::new();

    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    freq
}
```

Query:

```rust
println!("{}", freq.get(&1).unwrap_or(&0));
```

---

## 3. Real World Applications

Backend:

```text
Redis cache
```

Database:

```text
Query plans
```

Blockchain:

```text
State snapshots
```

AI:

```text
Embedding caches
```

OS:

```text
Page cache
```

---

## 4. System-Level Understanding

Precomputation is one of the most powerful optimization techniques in computer science.

You'll see it again in:

* Prefix Sums
* Sparse Tables
* Dynamic Programming
* Segment Trees
* Database Indexes
* Caching Systems

---

## 5. Engineering Tradeoffs

Benefit:

```text
Fast queries
```

Cost:

```text
Extra memory
Extra preprocessing time
```

---

## 6. Exercises

### Rust

Build frequency table and answer:

```text
Count(5)
Count(7)
Count(10)
```

---

### System Design

Design:

```text
Trending Hashtags Service
```

Millions of tweets.

How would you precompute frequencies?

---

## Part 1 Summary

| Topic                      | Complexity        | Key Idea                     |
| -------------------------- | ----------------- | ---------------------------- |
| Introduction to Hashing    | O(1) avg lookup   | Direct access                |
| Frequency Counting Problem | O(N) → O(1) query | Count occurrences            |
| Brute Force Counting       | O(N) per query    | Linear scan                  |
| Q × N Complexity           | O(QN)             | Repeated scans are expensive |
| Motivation for Hashing     | O(N+Q)            | Precompute once              |
| Precomputation             | O(N) build        | Store answers before queries |

Next, Part 2 will cover **Number Hashing, Frequency Arrays, Hash Arrays, Memory Limits, Local vs Global Arrays, and why array hashing breaks for very large numbers**.
