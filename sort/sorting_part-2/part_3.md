# Merge Sort in Rust — Part 3

## Pseudocode, Index-Based Division, Base Case, Mid Calculation, Recursion Tree & Dry Run

*(Based on the pseudocode and recursion section of the lecture.)* 

---

# Topics Covered

1. Why We Don't Create New Arrays
2. Using Indices (low, high, mid)
3. Writing the Merge Sort Pseudocode
4. The Base Case
5. Mid Calculation
6. Understanding Recursive Calls
7. Recursion Tree
8. Dry Run of Merge Sort
9. Stack Memory & System-Level Analysis
10. Engineering Tradeoffs

---

# Topic 1 — Why We Don't Create New Arrays

This is one of the biggest ideas in Merge Sort.

Most beginners think Merge Sort literally creates two new arrays every time.

Example:

```text
[3,1,2,4,1]

↓

Left
[3,1,2]

Right
[4,1]
```

The lecture explicitly says this division is **hypothetical**. We do **not** physically create these arrays. Instead, we represent them using indices. 

---

## Why?

Suppose the array has

```text
1,000,000 elements
```

Creating two new arrays at every recursive call would involve enormous copying.

Imagine copying:

```
1,000,000

↓

500,000 + 500,000

↓

250,000 + 250,000 + 250,000 + 250,000

↓

...
```

That would waste both memory and CPU time.

Instead, we say:

```
The original array never moves.
```

We simply remember:

```
Start Index

End Index
```

---

## Example

Instead of

```
Left Array

[3,1,2]
```

we store

```
low = 0

high = 2
```

Instead of

```
Right Array

[4,1]
```

we store

```
low = 3

high = 4
```

The actual array stays exactly where it is.

---

## Memory Layout

Original array

```
Index

0 1 2 3 4

Value

3 1 2 4 1
```

Left half

```
0 → 2
```

Right half

```
3 → 4
```

Nothing gets copied.

---

# C++ vs Rust

### C++

Usually

```cpp
mergeSort(arr, low, high);
```

`arr` is passed by reference.

---

### Rust

Idiomatic version

```rust
fn merge_sort(arr: &mut [i32], low: usize, high: usize)
```

or

```rust
fn merge_sort(arr: &mut Vec<i32>, low: usize, high: usize)
```

The array is borrowed mutably.

No copying.

Ownership never changes.

---

## Beginner Mistakes

Many beginners write

```rust
let left = arr[..mid].to_vec();
```

every recursive call.

This copies memory repeatedly.

Correct implementations operate on indices (or slices), not newly allocated vectors.

---

# Real-World Applications

## Backend

Large JSON arrays are processed by ranges rather than duplicated.

---

## Databases

Indexes point into pages instead of copying records.

---

## Blockchain

Nodes operate on offsets into blocks rather than cloning transactions.

---

## AI

Large tensors are sliced without duplicating memory.

---

## Operating Systems

Kernel schedulers frequently use pointer ranges instead of copying queues.

---

# System-Level Understanding

Passing

```
(low, high)
```

requires only a few machine words (integers).

Copying an array could require megabytes of memory movement.

Modern CPUs spend significant time moving data, so avoiding unnecessary copies dramatically improves performance.

---

# Engineering Tradeoffs

Using indices:

✔ Fast

✔ Cache friendly

✔ Memory efficient

Requires careful boundary handling.

---

# Exercises

### Rust

Represent

```
[9,4,7,1,6]
```

using index ranges after one recursive split.

---

### System Design

Suppose a 100 GB file cannot fit into RAM.

How would you represent partitions without copying data?

---

# Topic 2 — Understanding `low`, `high`, and `mid`

The lecture introduces three variables that completely describe the current subarray. 

## Conceptual Clarity

Every recursive call works on **one contiguous range**.

```
merge_sort(arr, low, high)
```

This means:

```
Sort everything from

low

to

high
```

Example

```
Index

0 1 2 3 4

Value

3 1 2 4 1
```

Initially

```
low = 0

high = 4
```

Entire array.

---

After division

```
Left

low = 0

high = 2
```

```
Right

low = 3

high = 4
```

No new arrays.

Just different index ranges.

---

## Why These Three Variables?

They completely describe the problem.

No extra information is required.

This is a common design pattern in recursive algorithms.

---

# Topic 3 — Writing the Merge Sort Pseudocode

The lecture's pseudocode is conceptually:

```text
merge_sort(arr, low, high)

1. Stop if one element remains.

2. Compute mid.

3. Sort left.

4. Sort right.

5. Merge.
```

Let's translate it into language-independent pseudocode.

```text
merge_sort(arr, low, high)

if low >= high

    return

mid = (low + high) / 2

merge_sort(arr, low, mid)

merge_sort(arr, mid+1, high)

merge(arr, low, mid, high)
```

That's the entire algorithm.

Every recursive call follows these exact five steps.

---

## Why Does It Work?

Assume recursively that:

```
merge_sort(left)
```

always returns a sorted left half.

Likewise,

```
merge_sort(right)
```

returns a sorted right half.

Then `merge()` combines two sorted halves into one sorted array.

This is an example of **inductive reasoning**, which underlies many recursive proofs.

---

# Topic 4 — The Base Case

Every recursive algorithm must eventually stop.

The lecture identifies the base case as:

```
low >= high
```



---

## Why?

If

```
low == high
```

there is exactly one element.

Example

```
Index

2

↓

Value

7
```

One element is already sorted.

Nothing to do.

---

If

```
low > high
```

the range is empty.

This usually doesn't occur in a correct Merge Sort, but checking it makes the function more robust.

---

## Rust

```rust
if low >= high {
    return;
}
```

Simple.

Essential.

---

## Beginner Mistakes

### Forgetting the base case

Leads to infinite recursion and eventually a **stack overflow**.

### Using the wrong condition

For example:

```rust
if low > high
```

This misses the `low == high` case and causes unnecessary recursive calls.

---

# Topic 5 — Mid Calculation

The lecture computes the midpoint as:

```text
mid = (low + high) / 2
```



This works for small examples, but production code often uses a safer formula.

---

## Why?

Suppose

```
low = 2_000_000_000

high = 2_100_000_000
```

Adding them may overflow a 32-bit integer.

---

## Safe Formula

```text
mid = low + (high - low)/2
```

This computes the same value while avoiding overflow.

---

## Rust

With `usize`, overflow is less common for normal arrays, but the safe formula is still good practice:

```rust
let mid = low + (high - low) / 2;
```

---

## Engineering Insight

This is not unique to Merge Sort.

You'll use the same midpoint formula in:

* Binary Search
* Segment Trees
* Divide-and-Conquer algorithms
* KD-Trees

---

# Topic 6 — Understanding Recursive Calls

The lecture repeatedly emphasizes the order:

```
Left

↓

Right

↓

Merge
```



The recursion is **depth-first**.

Suppose:

```
[4,2,7,1]
```

Call sequence:

```
merge_sort(0,3)

↓

merge_sort(0,1)

↓

merge_sort(0,0)

↓

return

↓

merge_sort(1,1)

↓

return

↓

merge

↓

merge_sort(2,3)

↓

...
```

Notice:

The right half isn't processed until the left half is completely finished.

This is how recursion naturally uses the call stack.

---

# Topic 7 — Recursion Tree

Let's build the recursion tree for:

```
[8,3,5,1]
```

```
merge_sort(0,3)

                 (0,3)

               /      \

          (0,1)      (2,3)

         /    \      /    \

     (0,0)(1,1)(2,2)(3,3)
```

Now recursion returns upward:

```
merge(0,0,1)

↓

merge(2,2,3)

↓

merge(0,1,3)
```

The tree has:

* Height: `log₂(n)`
* Number of leaves: `n`
* One merge operation per internal node

---

## Stack Memory

Every recursive call stores:

* `low`
* `high`
* `mid`
* return address
* local variables

The maximum number of simultaneous calls equals the height of the tree.

Thus:

```
Stack Space = O(log n)
```

The additional `O(n)` space of Merge Sort comes from the temporary merge buffer, not the recursion stack.

---

# Topic 8 — Dry Run

Let's trace:

```
[4,1,3,2]
```

### Call 1

```
merge_sort(0,3)
```

mid = 1

```
Left

0→1

Right

2→3
```

---

### Left

```
merge_sort(0,1)

↓

merge_sort(0,0)

return

↓

merge_sort(1,1)

return

↓

merge

↓

[1,4]
```

---

### Right

```
merge_sort(2,3)

↓

merge_sort(2,2)

return

↓

merge_sort(3,3)

return

↓

merge

↓

[2,3]
```

---

### Final Merge

Merge

```
[1,4]

and

[2,3]
```

Result

```
[1,2,3,4]
```

The recursion follows the exact sequence explained in the lecture: recurse fully into the left subtree, then the right subtree, and finally merge on the way back. 

---

# Topic 9 — Stack Memory & System-Level Analysis

## How the CPU Executes This

Each recursive call creates a **stack frame** containing:

* Function arguments (`low`, `high`)
* Local variables (`mid`)
* Return address
* Saved registers

Example:

```
Top of Stack

merge_sort(0,0)

merge_sort(0,1)

merge_sort(0,3)

main()
```

When the base case returns:

```
Pop

↓

Pop

↓

Merge

↓

Continue
```

This push/pop behavior is handled automatically by the CPU's call stack.

---

## Cache Behavior

During recursion:

* Very little data movement.
* Only a few integers (`low`, `high`, `mid`) are manipulated.

During merge:

* The algorithm reads contiguous memory from the two halves.
* Writes contiguously into the temporary buffer.
* Copies the merged data back contiguously.

This sequential access pattern is generally cache-friendly, which is one reason Merge Sort performs well in practice despite using extra memory.

---

# Topic 10 — Engineering Tradeoffs

## Advantages

* Elegant recursive structure.
* Easy to reason about.
* Predictable recursion depth (`O(log n)`).
* Stable sorting.
* Works well with linked lists and external storage.

## Disadvantages

* Function-call overhead compared with iterative algorithms.
* Uses extra memory for merging.
* Very small arrays may be faster with Insertion Sort due to lower constant factors.

---

# Summary of Part 3

| Topic                 | Key Idea                              | Complexity          | Real-World Use               |
| --------------------- | ------------------------------------- | ------------------- | ---------------------------- |
| No New Arrays         | Represent subarrays using indices     | O(1) extra per call | Databases, file systems      |
| `low`, `high`, `mid`  | Describe the active range             | O(1)                | Recursive divide-and-conquer |
| Merge Sort Pseudocode | Divide → Recurse → Merge              | O(n log n) overall  | General-purpose sorting      |
| Base Case             | Stop when range has ≤1 element        | O(1)                | All recursive algorithms     |
| Mid Calculation       | Split safely into two halves          | O(1)                | Binary Search, Segment Trees |
| Recursive Calls       | Left → Right → Merge                  | O(log n) depth      | DFS-style recursion          |
| Recursion Tree        | Visualize recursive decomposition     | O(log n) height     | Complexity analysis          |
| Dry Run               | Follow execution step by step         | —                   | Debugging and learning       |
| Stack Memory          | One frame per recursive call          | O(log n) stack      | Function execution model     |
| Engineering Tradeoffs | Balance simplicity, speed, and memory | —                   | Production system design     |


