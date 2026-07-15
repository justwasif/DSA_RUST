# Merge Sort in Rust — Part 2

### Merge Procedure, Two-Pointer Technique, Stability, Temporary Arrays, and Complete Dry Run

*(Based on the next section of the lecture transcript.)* 

---

# Topics Covered

1. Why Merge Happens After Division
2. The Merge Operation
3. Why Both Halves Are Already Sorted
4. Two-Pointer Technique
5. Temporary Array
6. Stability of Merge Sort
7. Complete Dry Run of the Merge Process
8. Engineering Perspective of Merging

---

# Topic 1 — Why Merge Happens After Division

## 1. Conceptual Clarity

Many beginners understand the **divide** step but struggle to understand **why we merge**.

Imagine this array:

```text
[3,1,2,4,1]
```

Merge Sort first divides it until every subarray contains **only one element**.

```text
            [3,1,2,4,1]

          /              \

     [3,1,2]           [4,1]

      /    \           /    \

   [3,1]   [2]      [4]    [1]

   /   \

 [3]   [1]
```

At this point every subarray contains one element.

A single element is already sorted.

```text
[7]

↓

Already Sorted
```

There is nothing left to divide.

Now the algorithm starts **coming back**.

Instead of dividing,

it starts **building larger sorted arrays.**

This is the second phase of Divide and Conquer.

---

## Why Not Sort While Dividing?

Suppose we have

```text
[7,2,5,1]
```

Could we somehow sort while splitting?

No.

Because we don't yet know where every number belongs.

Only after every smaller problem has been solved can we combine them correctly.

This is why Merge Sort performs **all the real work during the return phase of recursion.**

---

## Visualization

```text
DOWN

[8,3,5,1]

↓

[8,3]

↓

[8]

----------------

UP

[8]

+

[3]

↓

[3,8]
```

Notice:

Going downward is almost free.

Coming upward performs the comparisons.

---

## Time Complexity

Division itself

```text
O(log n)
```

because each level halves the array.

Merge

```text
O(n)
```

at every level.

Combined

```text
O(n log n)
```

---

## Rust Perspective

Nothing is copied while dividing.

Rust simply keeps passing

```rust
&mut [T]
```

or index ranges.

This makes division extremely cheap.

---

# Real World Applications

### Backend

Recursive task splitting.

Large API responses are partitioned among workers.

---

### Databases

External Merge Sort divides files into chunks.

---

### Blockchain

Blocks are divided among validation threads.

---

### AI

Large datasets are split across GPUs.

---

### Operating Systems

Filesystem indexing often divides directories into subproblems.

---

# System-Level Understanding

Division barely touches memory.

Most CPU work occurs during merging.

That is why Merge Sort spends most of its execution time reading memory sequentially.

---

# Engineering Tradeoffs

Divide phase

Very cheap.

Merge phase

Memory intensive.

---

# Exercises

### Rust

Draw the recursion tree for

```text
[5,2,8,6]
```

---

### System Design

Suppose 32 worker threads each sort their own partition.

How would you merge the 32 sorted outputs efficiently?

(Hint: Multi-way Merge.)

---

# Topic 2 — The Merge Operation

This is the heart of Merge Sort.

Everything before this step simply prepares for merging.

---

## Conceptual Clarity

Suppose recursion gives us

```text
Left

[1,3]

Right

[2]
```

Notice something important.

Neither array needs sorting anymore.

They are **already sorted.**

Our only task is combining them.

---

Imagine two queues.

```text
Left

1 3

Right

2
```

Compare only the front.

```text
1 vs 2
```

Smaller wins.

```text
Result

1
```

Move the left pointer.

Now

```text
3 vs 2
```

Smaller wins.

```text
Result

1 2
```

Move the right pointer.

Right becomes empty.

Everything remaining on the left must already be larger.

Simply copy it.

```text
1 2 3
```

Finished.

---

## Key Observation

We never compare

```text
3

with

1
```

again.

Every comparison happens only once.

This is why merging is linear.

---

## Complexity

Suppose

Left

```text
n₁ elements
```

Right

```text
n₂ elements
```

Total

```text
n₁+n₂
```

Every element enters the output exactly once.

Therefore

```text
O(n₁+n₂)
```

which is

```text
O(n)
```

---

## Rust Implementation

Conceptually

```rust
let mut temp = Vec::new();
```

Compare

```rust
left[i]
right[j]
```

Push smaller.

Advance pointer.

Eventually copy leftovers.

We'll implement this fully in Part 4.

---

## Common Beginner Mistakes

### Mistake 1

Sorting again during merge.

Wrong.

The halves are already sorted.

---

### Mistake 2

Comparing every element against every element.

That becomes

```text
O(n²)
```

instead of

```text
O(n)
```

---

### Mistake 3

Forgetting leftover elements.

Many students stop after one pointer reaches the end.

The remaining elements must still be copied.

---

# Applications

Database join algorithms use this exact merge idea.

Streaming log processors merge sorted event streams.

Git merges sorted commit histories.

Search engines merge posting lists.

---

# System-Level View

Sequential reading

```text
1

2

3

4

5
```

is cache-friendly.

Random access

```text
5

1

8

2

4
```

is much slower.

Merge Sort benefits because it mostly reads memory sequentially.

---

# Topic 3 — Why Both Halves Are Already Sorted

This question confuses many beginners.

Suppose recursion reaches

```text
[8]

[3]
```

Single elements are sorted.

Merge gives

```text
[3,8]
```

Now recursion returns.

Later

```text
[3,8]

[5]
```

Again,

both halves are sorted.

Merge gives

```text
[3,5,8]
```

This continues upward.

Every merge always receives **two sorted arrays**.

Never unsorted arrays.

That is the invariant maintained by recursion.

---

## Engineering Insight

Whenever you write recursive algorithms,

always ask

> What does recursion promise me?

Here,

recursion promises

> "Both children are sorted."

Everything else becomes easy.

---

# Topic 4 — Two-Pointer Technique

This technique appears everywhere in DSA.

Merge Sort is one of its earliest applications.

---

## Concept

Suppose

```text
Left

1 4 8

Right

2 3 9
```

Keep

```text
L

↓

1 4 8

R

↓

2 3 9
```

Compare

```text
1

2
```

Take

```text
1
```

Advance

```text
L

↓

4
```

Now compare

```text
4

2
```

Take

```text
2
```

Advance

Continue until one pointer finishes.

---

## Complexity

Each pointer moves only forward.

Never backward.

Maximum movements

```text
n
```

Hence

```text
O(n)
```

---

## Rust View

Typically

```rust
let mut left = low;
let mut right = mid + 1;
```

These are simply indices.

No copying.

No new arrays.

Just integer movement.

---

## Real World Examples

Backend

Merge paginated API responses.

---

Databases

Merge sorted indexes.

---

Blockchain

Merge validator lists.

---

AI

Merge sorted feature vectors.

---

Operating Systems

Merge event queues.

---

# Topic 5 — Temporary Array

The lecture emphasizes creating a temporary container during merge. 

---

## Why Is It Needed?

Suppose

```text
Original

4 1 3 2
```

If we overwrite elements immediately,

we destroy information that hasn't been compared yet.

Instead,

we build

```text
Temp

1 2 3 4
```

After completion,

copy back.

---

## Why Not Modify the Original Directly?

Imagine

```text
4

1
```

If

```text
4
```

gets overwritten,

you lose it forever.

Temporary storage prevents this.

---

## Complexity

Extra memory

```text
O(n)
```

---

## Rust

```rust
let mut temp = Vec::with_capacity(high-low+1);
```

Using

```rust
with_capacity()
```

avoids repeated reallocations.

This is a common production optimization.

---

## Beginner Mistakes

Using

```rust
Vec::new()
```

without capacity.

Repeated allocations slow the algorithm.

---

# Topic 6 — Stability of Merge Sort

One of Merge Sort's greatest strengths.

---

## What Is Stability?

Suppose

```text
(Alice,90)

(Bob,90)

(Charlie,80)
```

Sort by marks.

Stable output

```text
Charlie

Alice

Bob
```

Alice still comes before Bob.

Unstable output

```text
Charlie

Bob

Alice
```

Order changed.

---

## Why Merge Sort Is Stable

During merging,

when values are equal,

we take the **left element first**.

```text
if left <= right
```

instead of

```text
<
```

That tiny

```text
<=
```

makes Merge Sort stable.

---

## Real World Uses

Database sorting

Spreadsheet sorting

Student records

Financial transactions

Medical records

---

# Topic 7 — Complete Dry Run

Consider

```text
[5,2,8,1]
```

Divide

```text
[5,2]

[8,1]
```

Divide

```text
[5]

[2]

[8]

[1]
```

Merge

```text
5

2

↓

2 5
```

Merge

```text
8

1

↓

1 8
```

Final Merge

```text
2 5

1 8
```

Compare

```text
2

1

↓

1
```

Compare

```text
2

8

↓

1 2
```

Compare

```text
5

8

↓

1 2 5
```

Copy remaining

```text
8
```

Final

```text
1 2 5 8
```

---

# Topic 8 — Engineering Perspective of Merging

Production systems rarely merge just two arrays.

Examples:

Google Search

Millions of posting lists.

Databases

Hundreds of sorted runs.

External Merge Sort

Thousands of disk files.

Distributed systems

Thousands of worker outputs.

The exact same merge algorithm scales from

```text
2 arrays
```

to

```text
1000 arrays
```

using structures like **priority queues** (k-way merge).

---

# Summary of Part 2

| Topic                   | Key Idea                                    | Complexity                 | Production Use                             |
| ----------------------- | ------------------------------------------- | -------------------------- | ------------------------------------------ |
| Why Merge Happens       | Real work occurs during recursion unwinding | —                          | Recursive algorithms                       |
| Merge Operation         | Combine two sorted arrays                   | O(n)                       | Database joins, stream processing          |
| Sorted Halves Invariant | Recursion guarantees sorted children        | —                          | Divide-and-conquer algorithms              |
| Two-Pointer Technique   | Compare front elements and advance          | O(n)                       | APIs, indexes, event streams               |
| Temporary Array         | Prevent overwriting original data           | O(n) space                 | External sorting, in-memory sorting        |
| Stability               | Preserve relative order of equal elements   | —                          | Databases, spreadsheets, financial systems |
| Dry Run                 | Divide → Merge → Divide → Merge             | O(n log n) overall         | Algorithm understanding                    |
| Engineering Perspective | Merging scales to multi-way merges          | O(n log k) for k-way merge | Search engines, distributed systems        |

.
