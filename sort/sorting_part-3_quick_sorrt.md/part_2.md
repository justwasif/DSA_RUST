# Part 2 — The Partition Algorithm (The Heart of Quick Sort)

> **Based on the transcript:** This part covers the complete partition algorithm discussed by Striver, including:
>
> * What partitioning actually means
> * Smaller on the left, larger on the right
> * Low & High pointers
> * i and j pointers
> * Pointer movement
> * Swapping strategy
> * Crossing pointers
> * Final pivot swap
> * Partition index
> * Complete intuition and dry run
>
> These topics correspond to the central explanation of the lecture before recursion begins. 

---

# Topic 1 — What is Partitioning?

## 1. Conceptual Clarity

Partitioning is the **single most important operation** in Quick Sort.

Notice something interesting.

Quick Sort **never immediately sorts** the entire array.

Instead, it performs one operation:

> Put **one element** into the exact position where it belongs.

Everything else remains partially unsorted.

For example,

```
4 6 2 5 7 9 1 3
```

Suppose our pivot is

```
4
```

The goal is NOT

```
1 2 3 4 5 6 7 9
```

The goal is only

```
? ? ? 4 ? ? ? ?
```

where

* everything left is smaller
* everything right is larger

Notice:

The left half isn't sorted.

The right half isn't sorted.

Only the pivot is guaranteed correct.

That single guarantee is enough.

Recursion finishes the remaining work.

---

## Why is this enough?

Suppose after partition we obtain

```
2 1 3 4 6 5 7 9
```

Observe carefully.

Everything left

```
2 1 3
```

belongs somewhere before 4.

Everything right

```
6 5 7 9
```

belongs somewhere after 4.

No future recursive call will ever move 4 again.

It has reached its permanent home.

This is the key invariant of Quick Sort.

---

## Complexity

Partition scans the array exactly once.

```
Time : O(n)

Space : O(1)
```

---

# C++ vs Rust

In C++

```cpp
swap(arr[i], arr[j]);
```

directly swaps memory.

Rust does exactly the same thing safely.

```rust
arr.swap(i, j);
```

Rust guarantees

* indices remain valid
* ownership isn't violated
* no dangling pointers

without sacrificing speed.

---

# Beginner mistake

Many beginners think

> Partition sorts the left side.

Wrong.

It merely separates values.

Sorting happens later.

---

# Real World Applications

Partitioning appears everywhere.

Backend

* separating urgent and non-urgent jobs

Database

* partition pruning
* range partitioning

Blockchain

* separating valid and invalid transactions

AI

* filtering samples before processing

Operating Systems

* separating runnable vs blocked processes

---

# System Level

Partition performs sequential memory access.

That means

```
CPU Cache Friendly
```

Sequential scans are extremely fast because hardware prefetchers predict the next memory address.

---

# Engineering Tradeoff

Partition itself is cheap.

Choosing a bad pivot makes recursion expensive.

---

# Exercise

Partition

```
9 3 7 2 5
```

using pivot = 9.

---

# Topic 2 — Smaller on Left, Larger on Right

The transcript repeatedly emphasizes

> Smaller on the left

> Larger on the right 

This is the invariant maintained during partition.

---

## Why?

Imagine

```
Pivot = 10
```

Any number

```
<10
```

can never appear after 10 in the final sorted array.

Likewise,

```
>10
```

can never appear before 10.

So we separate them immediately.

---

Example

```
10 6 15 2 9 20 11
```

becomes

```
6 2 9 10 15 20 11
```

Notice

Right half

```
15 20 11
```

is still unsorted.

That's perfectly fine.

---

## Mathematical Invariant

During partition

```
Left Side <= Pivot

Right Side >= Pivot
```

This invariant never breaks.

Maintaining invariants is one of the most important skills in algorithm design.

---

# Real Systems

Databases

```
Customer IDs < 10000

Customer IDs >=10000
```

This is exactly partitioning.

Distributed systems

```
Requests
```

↓

```
Premium Queue

Regular Queue
```

Again partitioning.

---

# Topic 3 — Low and High Pointers

Instead of creating new arrays,

Quick Sort remembers only boundaries.

Instead of

```
Left Array

Right Array
```

we store

```
low

high
```

Suppose

```
4 6 2 5 7 9 1 3
```

Initially

```
low = 0

high = 7
```

Nothing is copied.

Only indices change.

---

## Why?

Imagine 100 million integers.

Copying halves repeatedly would cost enormous memory.

Instead,

Quick Sort simply changes

```
low

high
```

This makes Quick Sort

```
In-place
```

---

# Rust

Instead of allocating vectors

```rust
let left = arr[..mid].to_vec();
```

we use slices

```rust
&mut arr[low..=high]
```

or

```
low

high
```

indices.

No copying.

---

# Engineering

Modern databases almost never copy records.

They move pointers.

Exactly the same philosophy.

---

# Topic 4 — The Two Pointer Algorithm

Now comes the real algorithm.

We create

```
i
```

starting from left

and

```
j
```

starting from right.

```
4 6 2 5 7 9 1 3

^

i
```

```
4 6 2 5 7 9 1 3

              ^

              j
```

---

Their jobs differ.

i searches for

```
Wrong element on left

(> Pivot)
```

j searches for

```
Wrong element on right

(< Pivot)
```

When both are found,

swap them.

---

# Why Two Pointers?

Suppose

```
4 6 2 5 7 9 1 3
```

6 belongs on right.

3 belongs on left.

One swap fixes two mistakes simultaneously.

Without two pointers,

many unnecessary swaps happen.

---

Complexity

Still

```
O(n)
```

---

# Real World

Garbage Collectors

```
Live Objects

Dead Objects
```

↓

partition

Memory Compaction

---

# Topic 5 — Pointer Movement

The transcript explains the exact movement rule. 

---

Move i until

```
arr[i] > pivot
```

Move j until

```
arr[j] < pivot
```

Then stop.

---

Example

```
4 6 2 5 7 9 1 3
```

Pivot

```
4
```

i

```
4

skip

6

stop
```

j

```
3

stop
```

Swap

```
6

3
```

Array

```
4 3 2 5 7 9 1 6
```

Continue.

---

Eventually

```
4 3 2 1 7 9 5 6
```

Pointers cross.

Stop.

---

# Beginner Mistake

Many beginners stop when

```
i == j
```

Hoare partition stops after pointers cross.

Not before.

---

# Topic 6 — Swapping Strategy

Only swap when

```
i < j
```

If

```
i > j
```

stop immediately.

Why?

Because

Left side is already clean.

Right side is already clean.

Another swap would destroy the partition.

---

Rust

```rust
arr.swap(i, j);
```

No temporary variable needed.

Internally optimized.

---

# Topic 7 — Crossing Pointers

This is the stopping condition.

Initially

```
i →

← j
```

Eventually

```
← j

→ i
```

Crossing means

Every incorrect element has already been fixed.

No more work remains.

---

Visual

Before

```
2 1 3 4 6 5 7 9
```

After crossing

Everything left satisfies

```
<= pivot
```

Everything right satisfies

```
>= pivot
```

Mission accomplished.

---

# Topic 8 — Final Pivot Swap

Now comes the final move.

Remember,

Pivot was still sitting at

```
low
```

Swap

```
Pivot

↓

Last smaller element
```

Result

```
1 3 2 4 7 9 5 6
```

Now

```
4
```

can never move again.

Its position is permanent.

---

# Topic 9 — Partition Index

The pivot's final position is called the **partition index**. 

```
1 3 2 |4| 7 9 5 6
```

Index

```
3
```

This value is returned.

Recursion now processes

```
low

...

partition-1
```

and

```
partition+1

...

high
```

---

# Idiomatic Rust Implementation (Hoare-style Partition)

```rust
fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[low];
    let mut i = low;
    let mut j = high;

    loop {
        while i < high && arr[i] <= pivot {
            i += 1;
        }

        while j > low && arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
    }

    arr.swap(low, j);
    j
}
```

### Why this design?

* `&mut [i32]` uses a mutable slice instead of owning a `Vec`, making the function reusable and avoiding allocations.
* The `i < high` and `j > low` checks prevent index underflow/overflow with `usize`.
* `arr.swap()` is safe and efficient, with no need for a temporary variable.

> **Note:** There are several valid partition schemes (such as Hoare and Lomuto). The transcript's logic aligns with the two-pointer Hoare-style approach, though production implementations often vary slightly in boundary conditions. 

---

# Engineering Tradeoffs

| Approach         | Advantages                                   | Disadvantages                        |
| ---------------- | -------------------------------------------- | ------------------------------------ |
| Hoare Partition  | Fewer swaps, excellent practical performance | More subtle to implement correctly   |
| Lomuto Partition | Easier to understand and teach               | More swaps, often slower in practice |

---

# Exercises

### Coding Exercise 1

Implement only the `partition()` function in Rust.

Input:

```text
[8, 4, 7, 3, 10, 5]
```

Pivot:

```text
8
```

Print:

* Final array
* Partition index

---

### Coding Exercise 2

Modify the partition logic to place **larger elements on the left** and **smaller elements on the right**, producing the partition needed for **descending Quick Sort**.

---

### System Design Exercise

You are building a backend service that receives one million tasks with different priorities. Design a partitioning step that rearranges tasks so that:

* High-priority tasks come before a chosen priority threshold.
* Low-priority tasks come after it.
* The rearrangement is **in-place** with minimal extra memory.

How does this resemble Quick Sort's partition algorithm?

---

### Optimization Challenge

Research and implement **three-way partitioning (Dutch National Flag)**. Compare it against the standard two-way partition on arrays containing many duplicate values, and explain why it can significantly reduce recursion depth and improve performance.
