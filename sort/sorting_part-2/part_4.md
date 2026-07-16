Excellent. This is where we move beyond the lecture. Striver implements Merge Sort in C++, but since you're learning **DSA in Rust**, we'll build a **production-quality, idiomatic Rust implementation** while explaining every ownership and borrowing decision. This part corresponds to the implementation section of the lecture. 

---

# Merge Sort in Rust — Part 4

# Complete Rust Implementation & Rust-Specific Engineering

## Topics Covered

1. Rust Function Design
2. Ownership and Borrowing
3. Why `&mut [T]` is Better than `&mut Vec<T>`
4. Complete `merge()` Implementation
5. Complete `merge_sort()` Implementation
6. Generic Merge Sort (`T: Ord`)
7. Common Rust Mistakes
8. Performance Optimizations
9. Production Considerations

---

# Topic 1 — Designing Merge Sort in Rust

In C++, Striver writes something similar to

```cpp
void mergeSort(vector<int>& arr, int low, int high)
```

In Rust we have two possible signatures.

## Option 1

```rust
fn merge_sort(arr: &mut Vec<i32>, low: usize, high: usize)
```

Works.

But isn't ideal.

---

## Option 2 (Idiomatic)

```rust
fn merge_sort(arr: &mut [i32], low: usize, high: usize)
```

This is the preferred Rust approach.

Why?

Because slices are more general.

A slice can represent

* an entire vector
* part of a vector
* an array
* part of an array

without copying memory.

---

## Example

```rust
let mut numbers = vec![5, 3, 1, 4];

merge_sort(&mut numbers[..], 0, numbers.len() - 1);
```

Notice

```rust
&mut numbers[..]
```

This creates a mutable slice.

No allocation.

No copy.

Only a borrow.

---

# Why Rust Loves Slices

Suppose

```rust
let arr = vec![4,2,7,1];
```

Internally

```
Vec

Pointer
↓

4 2 7 1

Length = 4

Capacity = 4
```

A slice simply stores

```
Pointer

Length
```

No ownership.

No allocation.

This makes recursion very efficient.

---

# Topic 2 — Ownership and Borrowing

This is where Rust differs completely from C++.

---

## C++

```cpp
vector<int>& arr
```

Reference.

Compiler trusts you.

---

## Rust

```rust
&mut [i32]
```

Mutable borrow.

Compiler guarantees

* only one mutable borrow exists
* no data races
* no dangling references

---

Suppose

```rust
merge_sort(arr, low, high);
```

During recursion

Ownership never moves.

Instead

```
Main

↓

Mutable Borrow

↓

Recursive Borrow

↓

Recursive Borrow
```

Only one mutable reference exists at every point.

This is what makes Rust memory-safe.

---

## Beginner Mistake

Trying to write

```rust
let left = &mut arr[..mid];
let right = &mut arr[mid..];
```

and then modifying both.

Rust may reject overlapping mutable borrows.

Instead,

we either

* use indices

or

* use

```rust
split_at_mut()
```

when appropriate.

---

# Topic 3 — Building the merge() Function

Let's implement it exactly like the lecture.

---

## Function Header

```rust
fn merge(arr: &mut [i32], low: usize, mid: usize, high: usize)
```

---

## Step 1

Temporary vector

```rust
let mut temp = Vec::new();
```

Better version

```rust
let mut temp = Vec::with_capacity(high - low + 1);
```

Why?

Without capacity

```
push()

↓

allocate

↓

allocate again

↓

allocate again
```

Repeated reallocations.

With capacity

```
allocate once

↓

fill
```

Much faster.

---

## Step 2

Pointers

```rust
let mut left = low;
let mut right = mid + 1;
```

Exactly like Striver's lecture.

---

## Step 3

Compare

```rust
while left <= mid && right <= high {
```

This means

```
Left still has elements

AND

Right still has elements
```

---

Now compare

```rust
if arr[left] <= arr[right] {
```

Notice

```
<=
```

This is important.

It makes Merge Sort

**stable**.

---

Take left

```rust
temp.push(arr[left]);

left += 1;
```

Otherwise

```rust
temp.push(arr[right]);

right += 1;
```

---

# Remaining Elements

Exactly as explained in the lecture.

Left remaining

```rust
while left <= mid {

    temp.push(arr[left]);

    left += 1;
}
```

Right remaining

```rust
while right <= high {

    temp.push(arr[right]);

    right += 1;
}
```

---

## Copy Back

This corresponds to

```
arr[i] = temp[i-low]
```

from the lecture.

Rust

```rust
for i in low..=high {

    arr[i] = temp[i - low];
}
```

Notice

```
i-low
```

because

```
temp

starts

at

0

not

low
```

Exactly as Striver explained. 

---

# Complete merge()

```rust
fn merge(arr: &mut [i32], low: usize, mid: usize, high: usize) {
    let mut temp = Vec::with_capacity(high - low + 1);

    let mut left = low;
    let mut right = mid + 1;

    while left <= mid && right <= high {
        if arr[left] <= arr[right] {
            temp.push(arr[left]);
            left += 1;
        } else {
            temp.push(arr[right]);
            right += 1;
        }
    }

    while left <= mid {
        temp.push(arr[left]);
        left += 1;
    }

    while right <= high {
        temp.push(arr[right]);
        right += 1;
    }

    for i in low..=high {
        arr[i] = temp[i - low];
    }
}
```

---

# Topic 4 — Building merge_sort()

Now recursion.

---

Base case

```rust
if low >= high {
    return;
}
```

---

Middle

Safe version

```rust
let mid = low + (high - low) / 2;
```

---

Recursive calls

```rust
merge_sort(arr, low, mid);

merge_sort(arr, mid + 1, high);
```

---

Merge

```rust
merge(arr, low, mid, high);
```

---

# Complete merge_sort()

```rust
fn merge_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mid = low + (high - low) / 2;

    merge_sort(arr, low, mid);

    merge_sort(arr, mid + 1, high);

    merge(arr, low, mid, high);
}
```

---

# Full Program

```rust
fn merge(arr: &mut [i32], low: usize, mid: usize, high: usize) {
    let mut temp = Vec::with_capacity(high - low + 1);

    let mut left = low;
    let mut right = mid + 1;

    while left <= mid && right <= high {
        if arr[left] <= arr[right] {
            temp.push(arr[left]);
            left += 1;
        } else {
            temp.push(arr[right]);
            right += 1;
        }
    }

    while left <= mid {
        temp.push(arr[left]);
        left += 1;
    }

    while right <= high {
        temp.push(arr[right]);
        right += 1;
    }

    for i in low..=high {
        arr[i] = temp[i - low];
    }
}

fn merge_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let mid = low + (high - low) / 2;

    merge_sort(arr, low, mid);
    merge_sort(arr, mid + 1, high);

    merge(arr, low, mid, high);
}

fn main() {
    let mut arr = vec![3, 1, 2, 4, 1, 5, 2, 6, 4];

    let n = arr.len();

    merge_sort(&mut arr, 0, n - 1);

    println!("{:?}", arr);
}
```

Output

```text
[1,1,2,2,3,4,4,5,6]
```

---

# Topic 5 — Making Merge Sort Generic

Production Rust rarely hardcodes `i32`.

Instead

```rust
fn merge_sort<T: Ord>(...)
```

or

```rust
T: Ord + Copy
```

---

Why?

Now it sorts

```rust
Vec<String>
```

```rust
Vec<char>
```

```rust
Vec<Person>
```

provided the type implements `Ord`.

---

Example

```rust
let mut words = vec![
    "banana",
    "apple",
    "cat"
];
```

Exactly the same algorithm.

---

# Topic 6 — Common Rust Mistakes

## Mistake 1

Passing ownership

```rust
fn merge_sort(arr: Vec<i32>)
```

Every recursive call copies ownership.

Wrong.

---

Correct

```rust
&mut [i32]
```

---

## Mistake 2

Using

```rust
to_vec()
```

inside recursion.

Allocates memory every call.

---

## Mistake 3

Wrong midpoint

```rust
(low+high)/2
```

Can overflow.

---

## Mistake 4

Forgetting

```rust
if low >= high
```

Infinite recursion.

---

## Mistake 5

Using

```rust
<
```

instead of

```rust
<=
```

Makes Merge Sort unstable.

---

# Topic 7 — Performance Optimizations

## Reserve Capacity

Instead of

```rust
Vec::new()
```

use

```rust
Vec::with_capacity(...)
```

Avoids reallocations.

---

## Reuse Temporary Buffer

Production libraries often allocate one temporary buffer and reuse it across recursive calls instead of creating a new `Vec` each time.

Benefits:

* fewer allocations
* lower memory fragmentation
* better cache locality

---

## Hybrid Sorting

Production implementations usually switch to Insertion Sort for very small subarrays (commonly around 16–32 elements) because its lower constant overhead outweighs Merge Sort's recursive costs on tiny inputs.

---

# Topic 8 — System-Level Understanding

## CPU Cache

Merge Sort reads:

```
1

2

3

4
```

Sequentially.

Excellent cache locality during merge.

---

## Memory

Extra buffer

```
Temporary Array

↓

Copied Back

↓

Freed
```

This is why Merge Sort requires additional memory.

---

## Threading

Merge Sort parallelizes naturally.

```
Left Half

↓

Thread 1

Right Half

↓

Thread 2

↓

Merge
```

This structure is widely used in multicore systems.

---

# Real-World Applications

### Backend Engineering

* Sorting large API datasets before pagination.
* Parallel request processing where worker results are merged.

### Database Systems

* External Merge Sort for `ORDER BY` operations that exceed RAM.
* Background compaction in storage engines uses merge-like operations.

### Blockchain

* Merging sorted transaction batches.
* Deterministic ordering in distributed systems.

### AI / ML

* Sorting indices during preprocessing.
* Merging intermediate results from distributed training.

### Operating Systems

* External file sorting.
* Parallel log processing.
* Large-scale event aggregation.

---

# Exercises

### Exercise 1

Modify the implementation so it sorts

```rust
Vec<String>
```

instead of

```rust
Vec<i32>
```

---

### Exercise 2

Rewrite the implementation using

```rust
split_at_mut()
```

instead of index ranges.

---

### Exercise 3 (System Design)

Design a Merge Sort capable of sorting **100 GB** of data on a machine with **8 GB RAM**.

Think about:

* How would you split the data?
* Where would intermediate runs be stored?
* How would you perform the final merge efficiently?

---

# Summary of Part 4

| Topic           | Key Idea                                          | Rust Focus           | Production Relevance                 |
| --------------- | ------------------------------------------------- | -------------------- | ------------------------------------ |
| Function Design | Use slices instead of vectors                     | `&mut [T]`           | Flexible APIs                        |
| Ownership       | Borrow mutably, don't move ownership              | `&mut`               | Memory safety                        |
| `merge()`       | Two pointers + temporary buffer                   | `Vec::with_capacity` | Core merge logic                     |
| `merge_sort()`  | Divide, recurse, merge                            | Safe recursion       | Standard implementation              |
| Generic Version | Sort any ordered type                             | `T: Ord`             | Reusable libraries                   |
| Common Mistakes | Avoid unnecessary allocations and ownership moves | Borrowing, indexing  | Prevent bugs and inefficiency        |
| Optimizations   | Reuse buffers, hybrid sorting                     | Allocation control   | High-performance systems             |
| System View     | Cache-friendly merges, parallel recursion         | Memory layout        | Multicore and large-scale processing |


