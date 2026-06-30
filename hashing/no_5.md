# Part 5 — Hash Functions, Division Method, Collisions, Chaining, and How Hash Tables Actually Work

Topics covered:

31. Internal Hashing Mechanism
32. Hash Functions
33. Division Method of Hashing
34. Modulo-Based Hashing
35. Collision Handling
36. Chaining (Linked-List Style)
37. Collision Analysis
38. Worst Case of Hashing
39. Why Unordered Map Can Become O(N)
40. Keys in Map vs Unordered Map


---

# 🔷 Topic 31: Internal Hashing Mechanism

---

## 1. Conceptual Clarity

So far we've been using:

```rust
HashMap<K, V>
```

without knowing what happens internally.

A hash table has 3 main parts:

```text
Key
Hash Function
Bucket Array
```

Visualization:

```text
Key = 28

      hash()
         ↓

Bucket 8

         ↓

Store Value
```

The hash function converts a huge key space into a smaller bucket space.

---

### Why Is This Needed?

Suppose:

```text
Keys:
28
999999999
123456789
```

We cannot create:

```text
array[999999999]
```

So we compress large values into bucket indexes.

---

### Big Idea

Hash tables trade:

```text
Perfect indexing
```

for

```text
Approximate indexing
```

This is why collisions exist.

---

### C++ vs Rust

C++:

```cpp
unordered_map
```

Rust:

```rust
HashMap
```

Both are hash tables.

The implementation differs.

The concept is identical.

---

## 2. Rust Implementation

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(28, "hello");
}
```

Internally:

```text
28
 ↓
hash()
 ↓
bucket
 ↓
store pair
```

---

### Beginner Mistake

Thinking:

```rust
map.insert(28, value)
```

stores at index 28.

It does not.

Hashing determines location.

---

## 3. Real World Applications

### Backend

```text
UserID → User Data
```

---

### Database

```text
PageID → Page
```

---

### Blockchain

```text
Account → Balance
```

---

### AI

```text
Token → Embedding
```

---

### OS

```text
File Descriptor → Metadata
```

---

## 4. System-Level Understanding

Internally:

```text
CPU
 ↓
Compute Hash
 ↓
Find Bucket
 ↓
Search Bucket
```

Hash computation costs CPU cycles.

Good hashes reduce collisions.

---

## 5. Engineering Tradeoffs

Advantages:

✅ Fast lookups

Disadvantages:

❌ Memory overhead

❌ Collision handling needed

---

## 6. Exercises

Design:

```text
StudentID → Marks
```

using a hash table.

---

# 🔷 Topic 32: Hash Functions

---

## 1. Conceptual Clarity

A hash function transforms:

```text
Large Key
```

into

```text
Small Bucket Index
```

Example:

```text
Key = 123456
```

Hash:

```text
hash(123456)=17
```

Store inside bucket 17.

---

### Properties of Good Hash Functions

1. Fast
2. Deterministic
3. Uniform Distribution
4. Few Collisions

---

### Example

Bad:

```text
hash(x)=0
```

Everything goes to bucket 0.

Disaster.

---

Good:

```text
Keys spread evenly
```

---

## 2. Rust Perspective

Rust HashMap uses:

```text
SipHash
```

by default.

Modern Rust versions optimize around security and collision resistance.

---

### Beginner Mistake

Thinking:

```text
Hash Function
```

means encryption.

It doesn't.

Hashing ≠ Encryption.

---

## 3. Real World Applications

### Backend

Request caches.

---

### Databases

Hash indexes.

---

### Blockchain

State storage.

---

### AI

Vocabulary lookup tables.

---

### OS

Kernel hash tables.

---

## 4. System-Level Understanding

Good hash function:

```text
Uniform Buckets
```

Bad hash function:

```text
Hot Buckets
```

CPU performance collapses.

---

## 5. Engineering Tradeoffs

Stronger hashes:

✅ Fewer attacks

❌ More CPU cost

---

## 6. Exercises

Create a toy hash:

```rust
hash(x)=x%10
```

and test bucket distribution.

---

# 🔷 Topic 33: Division Method of Hashing

---

## 1. Conceptual Clarity

This is the method explained in the lecture. 

Formula:

```text
hash(key)=key % table_size
```

Suppose:

```text
Table Size = 10
```

Buckets:

```text
0
1
2
...
9
```

Now:

```text
2 % 10 = 2
```

Store at bucket 2.

---

Example:

```text
16 % 10 = 6

28 % 10 = 8

139 % 10 = 9
```

---

### Why It Works

It compresses huge values.

```text
139
```

becomes:

```text
9
```

---

## 2. Rust Example

```rust
fn hash(key: usize) -> usize {
    key % 10
}
```

---

### Beginner Mistake

Using:

```text
table_size = 2
```

creates terrible distribution.

---

## 3. Real World Applications

Many educational hash tables begin with division hashing.

---

## 4. System-Level Understanding

Modulo operation:

```text
CPU division instruction
```

Relatively cheap.

---

## 5. Engineering Tradeoffs

Simple:

✅ Easy

Bad:

❌ Can create patterns

---

## 6. Exercises

Hash:

```text
12
22
32
42
52
```

using:

```text
mod 10
```

Observe the issue.

---

# 🔷 Topic 34: Modulo-Based Hashing

---

## 1. Conceptual Clarity

Division method is a specific form of modulo hashing.

Formula:

```text
bucket = key % m
```

where:

```text
m = bucket count
```

---

Example:

```text
m = 8

17 % 8 = 1
25 % 8 = 1
33 % 8 = 1
```

Problem:

All collide.

---

### Why Modulo?

Keeps result inside valid range:

```text
0..m-1
```

---

## 2. Rust Example

```rust
let bucket = key % bucket_count;
```

---

## 3. Real World Applications

Networking.

Caches.

Databases.

---

## 4. System-Level Understanding

Modulo is effectively:

```text
Range Compression
```

---

## 5. Engineering Tradeoffs

Good:

```text
Fast
```

Bad:

```text
Pattern Sensitive
```

---

## 6. Exercises

Test:

```text
100 random numbers
```

with:

```text
mod 8
mod 97
```

Compare distribution.

---

# 🔷 Topic 35: Collision Handling

---

## 1. Conceptual Clarity

Collision means:

```text
Different Keys
```

produce

```text
Same Bucket
```

Example:

```text
18 % 10 = 8

28 % 10 = 8

38 % 10 = 8
```

All map to bucket 8.

---

Visualization:

```text
Bucket 8

18
28
38
```

This is a collision.

---

### Key Insight

Collisions are unavoidable.

No practical hash function eliminates them completely.

---

## 2. Rust Perspective

HashMap handles collisions automatically.

You never manually resolve them.

---

### Beginner Mistake

Thinking collisions mean:

```text
Wrong answer
```

No.

Collision handling is built in.

---

## 3. Real World Applications

Every production hash table deals with collisions.

Redis.

Rust HashMap.

Java HashMap.

Go Maps.

Everything.

---

## 4. System-Level Understanding

More collisions:

```text
More Memory Reads
More Cache Misses
More CPU Work
```

---

## 5. Engineering Tradeoffs

Good hash:

```text
Few collisions
```

Bad hash:

```text
Many collisions
```

---

## 6. Exercises

Find collisions under:

```text
mod 10
```

for:

```text
18
28
38
48
58
```

---

# 🔷 Topic 36: Chaining (Linked-List Style)

---

## 1. Conceptual Clarity

Lecture explains collision resolution through chaining. 

Instead of:

```text
Bucket 8

18
```

Store:

```text
Bucket 8

18 → 28 → 38 → 48
```

Multiple values occupy same bucket.

---

### Why Called Chaining?

Because entries form a chain.

Historically:

```text
Linked List
```

Modern implementations may use:

```text
Vectors
Trees
Hybrid structures
```

---

## 2. Rust Toy Example

```rust
let mut table: Vec<Vec<i32>> = vec![vec![]; 10];

table[8].push(18);
table[8].push(28);
table[8].push(38);
```

Bucket:

```text
[18,28,38]
```

---

### Beginner Mistake

Thinking one bucket stores one value.

It can store many.

---

## 3. Real World Applications

Almost every classical hash table.

---

## 4. System-Level Understanding

Search:

```text
Find bucket
Scan chain
```

Chain length determines speed.

---

## 5. Engineering Tradeoffs

Short chains:

✅ Fast

Long chains:

❌ Slow

---

## 6. Exercises

Implement a toy chained hash table.

---

# 🔷 Topic 37: Collision Analysis

---

## 1. Conceptual Clarity

Ideal:

```text
10 buckets
10 keys
```

Distribution:

```text
1 key per bucket
```

Bad:

```text
10 buckets
10 keys
```

Distribution:

```text
All 10 keys in one bucket
```

---

### Load Factor

A key concept:

```text
Load Factor

= Elements / Buckets
```

Example:

```text
100 elements
10 buckets

=10
```

High load factor means more collisions.

---

## 2. Rust Perspective

Rust HashMap automatically resizes.

This keeps collision rates low.

---

## 3. Real World Applications

Databases monitor load factors carefully.

---

## 4. System-Level Understanding

Collisions increase:

```text
Branch Mispredictions
Cache Misses
Pointer Traversals
```

---

## 5. Engineering Tradeoffs

More buckets:

✅ Fewer collisions

❌ More memory

---

## 6. Exercises

Calculate load factor:

```text
500 elements
50 buckets
```

---

# 🔷 Topic 38: Worst Case of Hashing

---

## 1. Conceptual Clarity

Worst case:

```text
All keys
```

land in:

```text
One bucket
```

Example:

```text
18
28
38
48
58
68
```

all map to bucket 8.

---

Then search becomes:

```text
Linear Search
```

---

Complexity:

```text
O(N)
```

instead of:

```text
O(1)
```

---

## 2. Rust Perspective

Possible theoretically.

Extremely rare practically.

---

## 3. Real World Applications

Security attacks deliberately create this.

---

## 4. System-Level Understanding

Lookup:

```text
Hash
 ↓
Bucket
 ↓
Scan Entire Chain
```

---

## 5. Engineering Tradeoffs

Worst-case guarantees:

```text
BTreeMap
```

wins.

Average performance:

```text
HashMap
```

wins.

---

## 6. Exercises

Show how:

```text
10 keys
```

can all collide under:

```text
mod 10
```

---

# 🔷 Topic 39: Why Unordered Map Can Become O(N)

---

## 1. Conceptual Clarity

Lecture emphasizes:

Average:

```text
O(1)
```

Worst:

```text
O(N)
```

because of collisions. 

---

Example:

```text
Bucket 8

18
28
38
48
58
68
```

Find:

```text
68
```

Need to scan chain.

---

Cost:

```text
O(N)
```

---

## 2. Rust Perspective

Interview answer:

```text
HashMap Lookup

Average O(1)

Worst O(N)
```

Always say both.

---

## 3. Real World Applications

Web servers defend against collision attacks.

---

## 4. System-Level Understanding

Poor bucket distribution destroys performance.

---

## 5. Engineering Tradeoffs

HashMap:

```text
Fast usually
```

BTreeMap:

```text
Predictable always
```

---

## 6. Exercises

Explain why collisions cause O(N).

---

# 🔷 Topic 40: Keys in Map vs Unordered Map

---

## 1. Conceptual Clarity

Lecture mentions:

Ordered maps support more complex key types than unordered maps in C++. 

Rust is different.

---

### HashMap Requirements

Key must implement:

```rust
Eq
Hash
```

---

### BTreeMap Requirements

Key must implement:

```rust
Ord
```

---

Examples:

```rust
HashMap<i32, i32>
HashMap<String, i32>
HashMap<(i32,i32), i32>
```

All valid.

---

## 2. Rust Example

```rust
use std::collections::HashMap;

let mut map: HashMap<(i32,i32), i32> = HashMap::new();

map.insert((1,2), 100);
```

Works perfectly.

---

### Beginner Mistake

Thinking Rust HashMap has same restrictions as C++ unordered_map.

It doesn't.

Trait system makes it more flexible.

---

## 3. Real World Applications

Graph algorithms:

```text
(NodeA, NodeB)
```

as key.

---

### Blockchain

```text
(Account, Token)
```

as key.

---

### Databases

Composite indexes.

---

## 4. System-Level Understanding

Composite keys are hashed by hashing each component and combining results.

---

## 5. Engineering Tradeoffs

Complex keys:

✅ Expressive

❌ Larger hash computation cost

---

## 6. Exercises

Create:

```rust
HashMap<(String,String), usize>
```

for flight routes.

---

# Part 5 Summary

| Topic                   | Core Idea                      | Complexity      |
| ----------------------- | ------------------------------ | --------------- |
| Internal Hashing        | Key → Hash → Bucket            | O(1) avg        |
| Hash Function           | Compress key space             | O(1)            |
| Division Method         | key % m                        | O(1)            |
| Modulo Hashing          | Bucket calculation             | O(1)            |
| Collisions              | Same bucket for multiple keys  | Depends         |
| Chaining                | Store multiple keys per bucket | O(chain length) |
| Collision Analysis      | Load factor matters            | —               |
| Worst Case              | All keys collide               | O(N)            |
| Why HashMap Can Be O(N) | Long chain scan                | O(N)            |
| Keys & Traits           | `Hash+Eq` vs `Ord`             | Depends         |

## The Most Important Interview Takeaway

When asked:

> What's the complexity of Rust's `HashMap`?

Answer:

```text
Insert  : Average O(1), Worst O(N)
Search  : Average O(1), Worst O(N)
Delete  : Average O(1), Worst O(N)
```

When asked:

> Why can worst case become O(N)?

Answer:

```text
Hash collisions cause many keys to land
in the same bucket, forcing linear scans.
```

That is exactly the insight Striver was teaching in the collision section. 

**Part 6** will cover the final lecture topic:

* Highest Frequency Element Problem
* Lowest Frequency Element Problem
* Complete Comparison Table
* Rust DSA Hashing Cheat Sheet
* Interview Patterns
* Real Production Examples (Redis, Linux, RocksDB, Ethereum, PyTorch).
