# Part 4 — Complete Rust Implementation (Production Quality)

> **Based on the transcript**, this part covers the coding section of the lecture, but expands it into production-quality Rust. We'll not only translate the C++ implementation but also explain *why* Rust code looks different, discuss ownership, borrowing, slices, unsigned indexing pitfalls, generic implementations, and production engineering practices. 

---

# Topics Covered

1. Mapping the C++ implementation to Rust
2. Rust function signatures
3. Mutable slices vs `Vec`
4. Writing the `partition()` function
5. Writing the recursive `quick_sort()`
6. Understanding ownership & borrowing
7. The `usize` problem
8. Generic Quick Sort (`T: Ord`)
9. Common beginner mistakes
10. Testing the implementation
11. Debugging strategies
12. Production optimizations

---

# Topic 1 — Translating the Algorithm into Rust

## Conceptual Clarity

The C++ implementation from the lecture is essentially:

```cpp
quickSort(arr, low, high)
{
    if(low < high)
    {
        int p = partition(arr, low, high);

        quickSort(arr, low, p-1);

        quickSort(arr, p+1, high);
    }
}
```

Rust follows **exactly the same algorithm**.

Only the syntax changes.

The recursive structure remains identical.

---

## Rust Version

```rust
fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition(arr, low, high);

    if p > 0 {
        quick_sort(arr, low, p - 1);
    }

    quick_sort(arr, p + 1, high);
}
```

---

## Why isn't it `Vec<i32>`?

Many beginners immediately write

```rust
fn quick_sort(arr: &mut Vec<i32>)
```

This works.

But it is **less flexible**.

Using

```rust
&mut [i32]
```

means the function works for

* vectors
* arrays
* slices
* borrowed portions of arrays

without copying.

This is the idiomatic Rust approach.

---

# C++ vs Rust

C++

```cpp
vector<int>& arr
```

Rust

```rust
&mut [i32]
```

Both avoid copying.

Rust simply makes the borrowing explicit.

---

# Real World

Production Rust libraries almost always accept slices instead of vectors.

For example

```rust
slice.sort();
```

works because slices are the universal abstraction.

---

# Topic 2 — Understanding Mutable Borrowing

Rust introduces ownership.

Quick Sort needs

* swapping
* mutation
* recursion

So we borrow mutably.

```rust
fn quick_sort(arr: &mut [i32], ...)
```

means

> "I temporarily have exclusive permission to modify this slice."

After the function returns,

ownership goes back.

---

## Why only one mutable borrow?

Suppose two functions modified

```text
arr
```

simultaneously.

One swaps

```text
4

6
```

Another swaps

```text
6

9
```

Data races become possible.

Rust forbids this entirely.

---

# Backend Example

Imagine

```text
Bank Balance
```

Two threads modify it simultaneously.

Rust prevents this unless synchronization is explicit.

Quick Sort benefits from the same guarantees.

---

# Topic 3 — Writing the Partition Function

Partition is the core.

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

---

## Line-by-Line

### Pivot

```rust
let pivot = arr[low];
```

The lecture chooses the first element.

No copying of the array.

Only copying one integer.

---

### Two pointers

```rust
let mut i = low;
let mut j = high;
```

Exactly like the lecture.

---

### Left scan

```rust
while i < high && arr[i] <= pivot
```

Move until

```text
First element > Pivot
```

---

### Right scan

```rust
while j > low && arr[j] > pivot
```

Move until

```text
First element <= Pivot
```

---

### Swap

```rust
arr.swap(i, j);
```

Rust provides this.

No temporary variable needed.

---

### Final swap

```rust
arr.swap(low, j);
```

Pivot reaches its permanent position.

Return

```text
j
```

which becomes

```text
partition_index
```

---

# Topic 4 — Why Rust Uses `usize`

This surprises many programmers.

Indices are

```rust
usize
```

not

```rust
i32
```

Why?

Memory addresses cannot be negative.

Example

```rust
arr[5]
```

Internally

```text
base_address

+

5 × element_size
```

A negative address makes no sense.

Hence

```rust
usize
```

---

# The Biggest Beginner Bug

Suppose

```rust
pivot = 0
```

Then

```rust
pivot - 1
```

becomes

```text
0 - 1
```

Unsigned integers wrap.

Rust panics in debug builds.

Wrong

```rust
quick_sort(arr, low, pivot - 1);
```

Correct

```rust
if pivot > 0 {
    quick_sort(arr, low, pivot - 1);
}
```

---

# Topic 5 — Generic Quick Sort

Production libraries rarely sort only integers.

Rust allows

```rust
fn quick_sort<T: Ord>(...)
```

Example

```rust
fn partition<T: Ord + Copy>(
    arr: &mut [T],
    low: usize,
    high: usize,
) -> usize {
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

Now Quick Sort works for

```rust
Vec<String>

Vec<char>

Vec<f64>

Vec<MyStruct>
```

provided ordering exists.

---

# Topic 6 — Common Beginner Mistakes

## Mistake 1

Using

```rust
Vec
```

everywhere.

Prefer

```rust
&mut [T]
```

---

## Mistake 2

Forgetting

```rust
pivot > 0
```

before

```rust
pivot - 1
```

---

## Mistake 3

Using

```rust
i32
```

for indices.

Rust expects

```rust
usize
```

---

## Mistake 4

Copying subarrays

Wrong

```rust
let left = arr[..mid].to_vec();
```

Quick Sort should not allocate.

---

## Mistake 5

Stopping partition when

```text
i == j
```

Hoare partition stops after

```text
i >= j
```

depending on the implementation.

---

# Topic 7 — Testing

Rust has excellent testing.

```rust
#[test]
fn quick_sort_test() {
    let mut arr = [4, 6, 2, 5, 7, 9, 1, 3];

    let len = arr.len();

    quick_sort(&mut arr, 0, len - 1);

    assert_eq!(
        arr,
        [1,2,3,4,5,6,7,9]
    );
}
```

Production code should always include tests.

---

# Topic 8 — Debugging Quick Sort

The easiest debugging method is printing pointer movement.

```rust
println!(
    "i={} j={} {:?}",
    i,
    j,
    arr
);
```

Watch

* i movement
* j movement
* swaps
* final pivot

This immediately reveals incorrect logic.

---

# Topic 9 — Performance Optimizations

## 1. Median-of-Three Pivot

Instead of

```text
First element
```

choose

Median of

```text
first

middle

last
```

Much better for nearly sorted arrays.

---

## 2. Random Pivot

Randomization prevents attackers from forcing worst-case performance.

Used in many libraries and systems.

---

## 3. Switch to Insertion Sort

Very small arrays (≈16 elements)

↓

Insertion Sort

because recursion overhead dominates.

This hybrid approach is faster in practice.

---

## 4. Introsort

The C++ Standard Library (`std::sort`) does **not** use plain Quick Sort.

It uses **Introsort**, which combines:

* Quick Sort (fast average case)
* Heap Sort (guaranteed worst-case)
* Insertion Sort (small partitions)

This avoids Quick Sort's worst-case `O(n²)` behavior while keeping excellent average performance.

---

# System-Level Understanding

## CPU Cache

Quick Sort is cache-friendly because partition scans memory sequentially.

Sequential access allows hardware prefetchers to load upcoming cache lines efficiently.

---

## Branch Prediction

The inner comparisons

```rust
arr[i] <= pivot
```

and

```rust
arr[j] > pivot
```

create conditional branches.

Balanced partitions generally produce more predictable execution than highly skewed partitions.

---

## Memory Layout

Quick Sort works **in place**.

No temporary arrays.

Excellent spatial locality.

This often makes it faster than Merge Sort despite having the same average asymptotic complexity.

---

# Real-World Applications

### Backend Engineering

* Sorting API results
* Ordering leaderboards
* Ranking search results

### Database Engines

* Internal sorting before index creation
* External sorting pipelines (often hybrid algorithms)

### Blockchain

* Ordering transactions before block construction
* Sorting validator metadata

### AI / ML

* Sorting distances in nearest-neighbor searches
* Data preprocessing
* Feature ranking

### Operating Systems

* Sorting scheduling statistics
* File metadata organization
* Performance analysis tools

---

# Engineering Tradeoffs

| Feature             | Quick Sort                   |
| ------------------- | ---------------------------- |
| In-place            | ✅ Yes                        |
| Stable              | ❌ No                         |
| Average Time        | O(n log n)                   |
| Worst Time          | O(n²)                        |
| Extra Memory        | O(log n) recursion (average) |
| Cache Friendly      | ✅ Excellent                  |
| Easy to Parallelize | Moderate                     |

---

# Exercises

## Exercise 1 — Generic Rust

Modify the implementation so it works for any type implementing `Ord` **without requiring `Copy`**. (Hint: you'll need a different strategy for handling the pivot value.)

---

## Exercise 2 — Benchmark

Implement:

* Quick Sort
* Merge Sort
* Insertion Sort

Benchmark them on:

* Random arrays
* Already sorted arrays
* Reverse sorted arrays
* Arrays with many duplicate values

Record execution time and explain the differences.

---

## System Design Exercise

You're building a high-performance analytics service that must sort millions of user events every minute.

* Would you choose plain Quick Sort?
* If not, which hybrid approach (such as Introsort or parallel sorting) would you use?
* How would cache locality, worst-case guarantees, and memory usage influence your decision?

---

In **Part 5**, we'll move beyond implementation and study **Complexity Analysis & Engineering**, including:

* Best, Average, and Worst-case analysis
* Why Quick Sort is usually faster than Merge Sort in practice
* Randomized Quick Sort
* Median-of-three pivot selection
* Three-way partitioning (Dutch National Flag)
* Cache locality and branch prediction
* Parallel Quick Sort
* Why `std::sort()` in C++ and Rust's standard sorting routines don't use plain Quick Sort alone
* Production tradeoffs in real systems
