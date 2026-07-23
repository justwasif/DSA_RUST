Excellent. Part 3 is where Quick Sort becomes a complete algorithm. Up until now, we've only learned **how to partition one array**. In this part, we'll learn **how recursion repeatedly applies partitioning until the entire array becomes sorted**.

> **Based on the transcript**, this part covers:
>
> * Recursive decomposition
> * Base case
> * Recursive calls
> * Recursive tree
> * Correctness proof
> * Complete pseudocode
> * Why Quick Sort terminates
> * Tail recursion discussion
>
> These correspond to the lecture sections after the partition algorithm is explained and before the final implementation. 

---

# Part 3 — Recursion & Complete Quick Sort Algorithm

---

# Topic 1 — Recursive Decomposition

## 1. Conceptual Clarity

Partitioning alone **does not sort** the array.

For example,

```text
4 6 2 5 7 9 1 3
```

After one partition,

```text
2 1 3 |4| 6 5 7 9
```

Notice carefully.

Is the array sorted?

No.

Only **4** is guaranteed to be correct.

Everything else still needs work.

So instead of solving the entire problem again, we solve **two smaller problems**.

```
Original

2 1 3 |4| 6 5 7 9
```

↓

```
Sort

2 1 3
```

↓

```
Sort

6 5 7 9
```

This idea is called **recursive decomposition**.

Large problem

↓

Two smaller independent problems

↓

Each becomes even smaller

↓

Eventually trivial.

---

## Why does this work?

Suppose

```
[Left] Pivot [Right]
```

We already know

```
Every Left Element < Pivot

Every Right Element > Pivot
```

Therefore,

the left side can never affect the right side.

The right side can never affect the left side.

This independence makes recursion possible.

---

## Mathematical View

If

```
QuickSort(A)
```

then

```
Partition(A)

↓

QuickSort(Left)

QuickSort(Right)
```

This recursive definition completely describes Quick Sort.

---

## Complexity Perspective

Each recursive call handles a **smaller subarray**.

Eventually,

```
n

↓

n/2

↓

n/4

↓

...
```

until reaching size 1.

---

## C++ vs Rust

C++

```cpp
quickSort(arr, low, high);
```

Rust

```rust
quick_sort(arr, low, high);
```

Conceptually identical.

Rust's ownership model doesn't interfere because every recursive call receives the same mutable slice, but only for a disjoint range.

---

## Beginner Mistake

Many beginners think recursion copies the array.

It doesn't.

Only

```
low

high
```

change.

The array remains the same.

---

# Real World Applications

Backend

Recursive directory traversal

Database

Recursive B-Tree traversal

Blockchain

Merkle tree traversal

AI

Decision tree inference

Operating Systems

Recursive filesystem walking

---

# System Level Understanding

Every recursive call creates a stack frame.

A stack frame stores

* return address
* parameters
* local variables

Unlike Merge Sort,

Quick Sort does **not** allocate temporary arrays.

Memory usage comes mainly from recursion.

---

# Engineering Tradeoff

Recursion makes code elegant.

Deep recursion can overflow the stack in worst-case inputs.

Production implementations often eliminate one recursive branch (tail recursion optimization) or switch to an iterative version.

---

# Exercise

Draw the recursive decomposition for

```
9 4 7 1 6
```

---

# Topic 2 — Base Case

The transcript states that Quick Sort only continues while `low < high`. 

---

## Why?

Suppose

```
[5]
```

One element.

Already sorted.

No work.

Suppose

```
[]
```

Empty array.

Already sorted.

Therefore,

```
low >= high
```

means

Stop recursion.

---

## Rust

```rust
if low >= high {
    return;
}
```

Simple.

Without this,

recursion never stops.

---

## Beginner Mistake

Writing

```rust
if low > high
```

instead of

```rust
low >= high
```

causes unnecessary recursive calls on single-element ranges.

---

# Real Systems

Recursive parsers

Stop at leaf nodes.

Graph DFS

Stops when there are no more neighbors.

Same principle.

---

# Topic 3 — Recursive Calls

Once partition returns

```
pivot_index
```

we solve

```
Left

low

...

pivot-1
```

and

```
Right

pivot+1

...

high
```

---

Visual

```
Before

4 6 2 5 7 9 1 3
```

↓

Partition

```
2 1 3 |4| 6 5 7 9
```

↓

Recursive calls

```
QuickSort(2 1 3)

QuickSort(6 5 7 9)
```

Each recursive call repeats exactly the same algorithm.

---

## Rust

```rust
quick_sort(arr, low, pivot - 1);

quick_sort(arr, pivot + 1, high);
```

This is why Quick Sort is elegant.

One function solves every subproblem.

---

## Beginner Mistake (Rust)

Suppose

```rust
pivot == 0
```

Then

```rust
pivot - 1
```

underflows because `usize` is unsigned.

Correct

```rust
if pivot > 0 {
    quick_sort(arr, low, pivot - 1);
}
```

This is one of the most common Rust errors.

---

# Topic 4 — Recursive Tree

Quick Sort forms a recursion tree.

Example

```
8 Elements
```

↓

```
4 + 3
```

↓

```
2 1 2 1
```

↓

```
1 1 1 1 ...
```

Tree

```
                8

          /            \

        3               4

      /   \          /    \

     1     1       2       1
```

Every node

=

One partition.

Leaves

=

Already sorted arrays.

---

## Why is this useful?

Height of tree

↓

Determines recursion depth.

Balanced tree

```
Height = log₂ n
```

Worst tree

```
Height = n
```

---

# Real Systems

Recursive syntax trees

Recursive expression evaluation

Binary tree traversal

Compiler AST processing

---

# Topic 5 — Correctness Proof

How do we know Quick Sort always works?

We prove using induction.

---

### Step 1

Partition guarantees

```
Left < Pivot < Right
```

Always true.

---

### Step 2

Assume recursive calls correctly sort smaller arrays.

---

### Step 3

Then

```
Sorted Left

Pivot

Sorted Right
```

must be globally sorted.

Therefore

Quick Sort is correct.

---

This proof style appears everywhere in Computer Science.

---

# Topic 6 — Complete Pseudocode

High-level algorithm:

```
QuickSort(array, low, high)

if low >= high

    return

partition_index = partition(array, low, high)

QuickSort(array,
          low,
          partition_index - 1)

QuickSort(array,
          partition_index + 1,
          high)
```

Notice

The recursion is incredibly small.

Almost all complexity lies inside `partition()`.

---

# Topic 7 — Why Does Quick Sort Terminate?

Every recursive call works on a strictly smaller range.

Suppose

```
8 Elements
```

↓

```
3 Elements

4 Elements
```

↓

```
1

2

1

2
```

Eventually,

every subarray becomes

```
Size = 1
```

Base case reached.

Recursion ends.

---

Mathematically

If

```
Size decreases every call

AND

Minimum size = 1
```

Algorithm must terminate.

---

# Topic 8 — Tail Recursion Discussion

Standard Quick Sort

```
QuickSort()

↓

QuickSort(Left)

↓

QuickSort(Right)
```

Two recursive calls.

Not tail-recursive.

---

Production systems often optimize.

Instead of

```
Left

Right
```

recursively,

they recurse only into the smaller partition and process the larger one in a loop.

Pseudo-idea:

```
while low < high {
    let p = partition(...);

    if left_size < right_size {
        quick_sort(left);
        low = p + 1;      // loop on right
    } else {
        quick_sort(right);
        high = p - 1;     // loop on left
    }
}
```

### Why?

Worst-case recursion depth:

```
O(n)
```

↓

Optimized version:

```
O(log n)
```

This dramatically reduces the chance of stack overflow.

---

# Complete Idiomatic Rust Quick Sort

```rust
fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot = partition(arr, low, high);

    if pivot > 0 {
        quick_sort(arr, low, pivot - 1);
    }

    quick_sort(arr, pivot + 1, high);
}
```

### Design Choices

* `&mut [i32]` allows in-place sorting without owning the data.
* The base case prevents infinite recursion.
* The `pivot > 0` guard avoids `usize` underflow.
* The algorithm allocates no additional arrays.

---

# Engineering Tradeoffs

| Recursive Quick Sort       | Iterative Quick Sort    |
| -------------------------- | ----------------------- |
| Easier to understand       | More complex            |
| Elegant                    | Better stack usage      |
| Natural divide-and-conquer | Explicit stack required |
| Can overflow in worst case | Avoids recursion limits |

---

# Real Production Systems

### Backend Engineering

Recursive request processing trees and hierarchical data traversal.

### Database Systems

Recursive B-tree descent during searches and updates.

### Blockchain

Merkle tree construction and verification.

### AI/ML Infrastructure

Decision-tree inference and recursive spatial partitioning (e.g., KD-trees).

### Operating Systems

Recursive directory traversal, process tree inspection, and hierarchical namespace handling.

---

# Exercises

## Coding Exercise 1

Implement the complete recursive Quick Sort using your `partition()` function.

Test it on:

```text
[9, 4, 7, 1, 6, 2, 8, 5]
```

---

## Coding Exercise 2

Modify Quick Sort to sort in **descending order** by changing only the partition comparisons.

---

## System Design Exercise

You are building a distributed log-processing service. Logs are recursively partitioned by timestamp ranges before being processed by different worker nodes.

* How does this resemble Quick Sort's recursive decomposition?
* What advantages does this provide for parallel execution?

---

## Optimization Challenge

Implement **tail-recursion-optimized Quick Sort**, where you always recurse on the **smaller partition** and iterate over the larger one. Compare its maximum recursion depth with the standard implementation on already sorted input. This optimization is widely used in production sorting libraries to improve robustness.
