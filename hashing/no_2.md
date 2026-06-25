# Part 2 — Number Hashing, Frequency Arrays, and Memory Limits in Rust

Topics covered:

7. Number Hashing using Arrays
8. Frequency Arrays / Hash Arrays
9. Hashing Workflow (Store & Fetch)
10. Complexity of Array Hashing
11. Memory Limits of Arrays
12. Local vs Global Arrays
13. Number Hashing Limitations

Based on the lecture transcript. 

---

# 🔷 Topic 7: Number Hashing Using Arrays

---

## 1. Conceptual Clarity

The lecture introduces the simplest form of hashing:

Instead of storing frequencies inside a map, we use an array itself as the hash table.

Example:

```text
nums = [1,2,1,3,2]
```

Maximum value:

```text
max = 3
```

Create:

```text
hash = [0,0,0,0]
```

Indexes:

```text
0 1 2 3
```

Now:

```text
hash[1] += 1
hash[2] += 1
hash[1] += 1
hash[3] += 1
hash[2] += 1
```

Result:

```text
hash = [0,2,2,1]
```

Meaning:

```text
1 appears 2 times
2 appears 2 times
3 appears 1 time
```

This is called **direct-address hashing**.

---

### Intuition

Instead of:

```text
Value -> Search
```

We do:

```text
Value -> Index
```

The value itself becomes the address.

---

### Complexity

Building:

```text
O(N)
```

Query:

```text
O(1)
```

Space:

```text
O(MaxValue)
```

This space complexity is important.

---

### C++ vs Rust

C++:

```cpp
int hash[100];
```

Rust:

```rust
let mut hash = vec![0; 100];
```

Rust avoids:

* buffer overflow
* invalid memory access

through bounds checking.

---

## 2. Rust Implementation

```rust
fn main() {
    let nums = vec![1,2,1,3,2];

    let mut hash = vec![0; 4];

    for num in nums {
        hash[num as usize] += 1;
    }

    println!("{:?}", hash);
}
```

Output:

```text
[0,2,2,1]
```

---

### Why `usize`?

Rust arrays use:

```rust
usize
```

for indexing.

This prevents negative indexes.

---

### Beginner Mistake

Wrong:

```rust
hash[num]
```

Compiler error.

Need:

```rust
hash[num as usize]
```

---

## 3. Real World Applications

### Backend

HTTP status frequency:

```text
200 -> count
404 -> count
500 -> count
```

---

### Database

Page access counts:

```text
PageID -> frequency
```

---

### Blockchain

Gas usage histogram.

---

### AI

Token frequency counting.

---

### Operating Systems

Interrupt frequency tracking.

---

## 4. System-Level Understanding

Arrays are stored contiguously:

```text
|0|1|2|3|4|5|
```

CPU cache loves this.

Access:

```rust
hash[5]
```

is extremely fast.

Usually faster than HashMap.

---

## 5. Engineering Tradeoffs

Advantages:

✅ Fastest lookup possible

✅ Best cache locality

✅ Minimal overhead

Disadvantages:

❌ Wastes memory

❌ Requires bounded values

---

## 6. Exercises

### Rust

Count frequencies for:

```text
[2,5,2,8,5,5]
```

using array hashing.

---

### System Design

Design:

```text
Realtime score counter
```

Scores between:

```text
0..100
```

Would array hashing work?

---

# 🔷 Topic 8: Frequency Arrays / Hash Arrays

---

## 1. Conceptual Clarity

The lecture calls this:

```text
Hash Array
```

or

```text
Frequency Array
```

Both mean the same thing.

Example:

```text
hash[7] = 5
```

means:

```text
Number 7 appears 5 times
```

The array stores frequencies instead of raw data.

---

### Mathematical View

Input:

```text
1,2,1,3,2
```

Transform into:

```text
f(1)=2
f(2)=2
f(3)=1
```

This transformation is the foundation of hashing.

---

## 2. Rust Implementation

```rust
fn build_freq(nums: &[usize], max_val: usize) -> Vec<usize> {
    let mut freq = vec![0; max_val + 1];

    for &num in nums {
        freq[num] += 1;
    }

    freq
}
```

---

### Ownership Notes

Borrowing:

```rust
&[usize]
```

avoids copying.

Efficient.

---

## 3. Real World Applications

### Backend

API hit frequency.

---

### Databases

Query frequency statistics.

---

### Blockchain

Transaction type frequencies.

---

### AI

Word counts.

---

### OS

System call frequencies.

---

## 4. System-Level Understanding

Frequency arrays behave like compressed statistics.

Instead of storing:

```text
1
1
1
1
1
```

store:

```text
1 -> 5
```

Huge reduction in storage.

---

## 5. Engineering Tradeoffs

Good:

```text
Small bounded universe
```

Bad:

```text
Large sparse universe
```

Example:

```text
Value = 1,000,000,000
```

Array becomes impractical.

---

## 6. Exercises

Build a frequency array for:

```text
0..50
```

Track occurrences.

---

# 🔷 Topic 9: Hashing Workflow (Store & Fetch)

---

## 1. Conceptual Clarity

The lecture defines hashing as:

```text
Store
Fetch
```

or

```text
Precompute
Query
```

Workflow:

### Step 1

Store frequencies.

### Step 2

Answer queries instantly.

---

Example:

```text
nums=[1,2,1,3,2]
```

Store:

```text
hash[1]=2
hash[2]=2
hash[3]=1
```

Query:

```text
hash[2]
```

Answer:

```text
2
```

No searching.

---

## 2. Rust Implementation

Precompute:

```rust
let freq = build_freq(&nums, 10);
```

Fetch:

```rust
println!("{}", freq[2]);
```

---

### Common Mistake

Rebuilding hash table every query:

Wrong:

```rust
for q in queries {
    build_freq(...);
}
```

Build once.

Reuse forever.

---

## 3. Real World Applications

Redis:

```text
Store once
Query thousands of times
```

Same idea.

---

## 4. System-Level Understanding

Classic systems optimization:

```text
Trade memory
for speed
```

---

## 5. Engineering Tradeoffs

Preprocessing cost:

```text
O(N)
```

Saved query cost:

```text
O(QN)
```

---

## 6. Exercises

Implement:

```rust
build()
query()
```

as separate functions.

---

# 🔷 Topic 10: Complexity of Array Hashing

---

## 1. Conceptual Clarity

Build:

```text
Traverse array once
```

Cost:

```text
O(N)
```

Query:

```text
hash[x]
```

Cost:

```text
O(1)
```

Total:

```text
O(N+Q)
```

instead of:

```text
O(QN)
```

---

### Example

```text
N=100000
Q=100000
```

Brute force:

```text
10^10 operations
```

Hashing:

```text
200000 operations
```

Massive improvement. 

---

## 2. Rust Perspective

Array indexing:

```rust
freq[x]
```

is constant time.

Because:

```text
address = base + x * size
```

computed directly.

---

## 3. Real World Applications

Every cache system depends on this idea.

---

## 4. System-Level Understanding

CPU performs:

```text
One arithmetic operation
One memory read
```

for lookup.

Extremely efficient.

---

## 5. Engineering Tradeoffs

Time:

✅ Excellent

Memory:

⚠ Depends on maximum value.

---

## 6. Exercises

Compute complexity:

```text
N=10^6
Q=10^6
```

for both brute force and hashing.

---

# 🔷 Topic 11: Memory Limits of Arrays

---

## 1. Conceptual Clarity

Lecture discusses:

```text
Maximum practical array sizes.
```

Why?

Arrays require continuous memory.

Example:

```text
int array[10^9]
```

Memory needed:

```text
10^9 * 4 bytes
=
4 GB
```

Impossible in most competitive programming environments.

---

### Why This Matters

Array hashing depends on:

```text
Maximum Value
```

not

```text
Number of Elements
```

Huge difference.

---

## 2. Rust Example

Dangerous:

```rust
let huge = vec![0u32; 1_000_000_000];
```

Needs ~4 GB.

Likely crashes.

---

## 3. Real World Applications

Databases avoid giant arrays.

They use:

* B-Trees
* HashMaps
* Sparse structures

instead.

---

## 4. System-Level Understanding

Memory comes from:

* Stack
* Heap

Large allocations stress:

* RAM
* Virtual memory
* Cache hierarchy

---

## 5. Engineering Tradeoffs

Arrays:

```text
Fast
```

Memory hungry.

Maps:

```text
Slightly slower
```

Memory efficient for sparse data.

---

## 6. Exercises

Calculate memory usage for:

```text
10^7 integers
```

---

# 🔷 Topic 12: Local vs Global Arrays

---

## 1. Conceptual Clarity

Lecture mentions:

Local:

```cpp
inside main()
```

Global:

```cpp
outside main()
```

because memory allocation differs. 

---

### Stack

Local variables:

```text
Stored on stack
```

Small.

Fast.

Limited.

---

### Global

Stored in:

```text
Data segment
```

Much larger.

---

### Rust Equivalent

Rust doesn't encourage giant stack arrays.

Instead:

```rust
let big = vec![0; 10_000_000];
```

allocates on heap.

---

## 2. Rust Implementation

Stack:

```rust
let arr = [0; 100];
```

Heap:

```rust
let arr = vec![0; 10_000_000];
```

---

## 3. Real World Applications

Servers rarely keep huge arrays on stack.

Heap allocations dominate.

---

## 4. System-Level Understanding

Stack:

```text
Thread-local
```

Heap:

```text
Shared process memory
```

---

## 5. Engineering Tradeoffs

Stack:

✅ Fast

❌ Small

Heap:

✅ Large

❌ Allocation cost

---

## 6. Exercises

Compare memory behavior of:

```rust
[0;100]
```

and

```rust
vec![0;100]
```

---

# 🔷 Topic 13: Number Hashing Limitations

---

## 1. Conceptual Clarity

This is where array hashing breaks.

Suppose:

```text
Value = 10^9
```

Need:

```text
freq[1_000_000_000]
```

Array size:

```text
1 billion
```

Impossible.

This is why maps exist.

---

### Sparse Data Example

Data:

```text
1
999999999
```

Only 2 values.

Array would need:

```text
1 billion slots
```

Wasteful.

---

## 2. Rust Solution

Use:

```rust
use std::collections::HashMap;
```

instead.

```rust
let mut freq = HashMap::new();
```

Stores only used keys.

---

## 3. Real World Applications

### Backend

User IDs:

```text
1
1000000000
```

Use HashMap.

---

### Database

Primary keys are sparse.

---

### Blockchain

Wallet addresses huge.

Array impossible.

---

### AI

Vocabulary IDs can be enormous.

---

### Operating Systems

Process IDs sparse.

Maps preferred.

---

## 4. System-Level Understanding

Sparse structures save memory.

HashMap stores:

```text
Only existing keys
```

instead of:

```text
Every possible key
```

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
Huge memory savings
```

---

## 6. Exercises

### Rust

Count frequencies of:

```text
[1, 1_000_000_000, 42]
```

using HashMap.

---

### System Design

Design:

```text
UserID -> Login Count
```

for 500 million users.

Would you choose:

```text
Vec
```

or

```text
HashMap
```

and why?

---

# Part 2 Summary

| Topic                      | Time Complexity | Space Complexity     | Key Insight                   |
| -------------------------- | --------------- | -------------------- | ----------------------------- |
| Number Hashing             | O(N+Q)          | O(MaxValue)          | Value becomes index           |
| Frequency Arrays           | O(N) build      | O(MaxValue)          | Store counts                  |
| Store & Fetch              | O(1) query      | O(MaxValue)          | Precompute once               |
| Array Hashing Complexity   | O(N+Q)          | O(MaxValue)          | Faster than O(QN)             |
| Memory Limits              | —               | Large                | Arrays need contiguous memory |
| Local vs Global Arrays     | —               | Depends              | Stack vs Heap/Data segment    |
| Number Hashing Limitations | O(1) lookup     | Huge for sparse keys | Use HashMap for large values  |

**Part 3** will cover:

* Character Hashing
* Character Frequency Counting
* ASCII
* Character → Index Mapping
* Lowercase Hashing
* Full ASCII Hashing (256)
* Automatic Type Casting of Characters
* Why Character Hashing is Easier than Number Hashing.
