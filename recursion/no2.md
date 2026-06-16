# Parameterised and Functional Recursion (Striver Lecture) — Rust-Focused Deep Dive

Based on the transcript, the topics appear in this order:

1. Recursion Fundamentals
2. Sum of First N Numbers
3. Parameterised Recursion
4. Recursion Tree & Call Stack
5. Functional Recursion
6. Returning Values Through Recursion
7. Breaking Problems into Smaller Subproblems
8. Factorial Using Recursion
9. Base Cases in Recursive Problems
10. Time Complexity of Recursion
11. Space Complexity (Auxiliary Stack Space)

Source transcript: 

---

# 🔷 Topic 1: Recursion Fundamentals

---

## 1. Conceptual Clarity

Recursion is a technique where a function calls itself to solve a smaller version of the same problem.

Think of it as:

```text
Big Problem
   ↓
Smaller Problem
   ↓
Even Smaller Problem
   ↓
Base Case
```

Every recursive solution needs:

### Base Case

Condition where recursion stops.

Example:

```rust
if n == 0 {
    return;
}
```

### Recursive Call

Function calls itself with a smaller input.

```rust
f(n - 1);
```

---

### Intuition

Instead of:

```cpp
for(int i=1;i<=n;i++)
```

we repeatedly ask:

```text
Can I solve n by solving n-1?
```

---

### C++ vs Rust

C++:

```cpp
void f(int n)
```

Rust:

```rust
fn f(n: i32)
```

Rust automatically prevents:

* dangling references
* use-after-free
* invalid memory access

Recursion itself behaves similarly in both languages.

---

## 2. Rust Implementation

```rust
fn recurse(n: i32) {
    if n == 0 {
        return;
    }

    println!("{}", n);

    recurse(n - 1);
}
```

---

### Common Rust Mistakes

❌ Forgetting termination condition

```rust
recurse(n);
```

Infinite recursion.

---

❌ Using unsigned integers carelessly

```rust
fn recurse(n: usize)
```

then:

```rust
n - 1
```

can underflow.

Prefer:

```rust
i32
```

for beginner recursion problems.

---

## 3. Real-World Applications

### Backend

* request routing trees
* nested JSON processing

### Databases

* B-Trees
* query execution plans

### Blockchain

* Merkle tree traversal

### AI

* recursive model structures
* tree search

### OS

* directory traversal

---

## 4. System-Level Understanding

Each call creates a stack frame.

Memory:

```text
Stack

f(3)
f(2)
f(1)
f(0)
```

CPU repeatedly pushes/pops frames.

---

## 5. Engineering Tradeoffs

Advantages:

* elegant
* expressive

Disadvantages:

* stack overflow risk
* slower than loops

---

## 6. Exercises

### Coding

1. Print N to 1 recursively.
2. Print 1 to N recursively.

### System Design

How would Linux recursively traverse directories?

### Optimization

Convert recursion to iteration.

---

# 🔷 Topic 2: Sum of First N Numbers

---

## 1. Conceptual Clarity

Problem:

```text
N = 3

1 + 2 + 3 = 6
```

Goal:

```text
sum(3) = 6
```

This becomes the foundation for understanding recursive decomposition.

---

### Complexity

Time:

```text
O(N)
```

Space:

```text
O(N)
```

---

## 2. Rust Implementation

Iterative:

```rust
fn sum_loop(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}
```

Recursive approaches come next.

---

## 3. Real-World Applications

Summation patterns appear in:

### Backend

* aggregations
* analytics

### Databases

* SUM()

### Blockchain

* cumulative balances

### AI

* loss accumulation

### OS

* resource accounting

---

## 4. System-Level Understanding

Sequential accumulation is cache friendly.

---

## 5. Engineering Tradeoffs

For production:

```rust
n * (n + 1) / 2
```

is superior.

---

## 6. Exercises

1. Sum even numbers.
2. Sum odd numbers.

System design:

Design an API returning cumulative user activity.

---

# 🔷 Topic 3: Parameterised Recursion

---

## 1. Conceptual Clarity

This is the first major concept of the lecture.

Instead of returning values, we carry the answer as a parameter.

Striver's idea:

```text
f(i, sum)
```

State travels downward.

Example:

```text
f(3,0)
f(2,3)
f(1,5)
f(0,6)
```

Answer already exists in parameter.

---

### Intuition

Think of it as:

```text
Accumulation while going down
```

instead of

```text
Computation while returning up
```

---

### Complexity

Time:

```text
O(N)
```

Space:

```text
O(N)
```

---

### C++ vs Rust

C++ often uses references:

```cpp
void f(int i,int sum)
```

Rust:

```rust
fn f(i: i32, sum: i32)
```

Values are copied safely.

No need for pointers.

---

## 2. Rust Implementation

```rust
fn sum_parameterised(i: i32, sum: i32) {
    if i < 1 {
        println!("{}", sum);
        return;
    }

    sum_parameterised(i - 1, sum + i);
}
```

Usage:

```rust
sum_parameterised(3, 0);
```

Output:

```text
6
```

---

### Common Rust Mistakes

❌ Passing mutable references unnecessarily

```rust
fn f(i: i32, sum: &mut i32)
```

Not needed.

Use value semantics.

---

## 3. Real-World Applications

### Backend

Running request statistics.

### Databases

Aggregate scan operators.

### Blockchain

Running transaction totals.

### AI

Mini-batch metric accumulation.

### OS

CPU usage accumulation.

---

## 4. System-Level Understanding

Each frame stores:

```text
i
sum
return address
```

Memory grows linearly.

---

## 5. Engineering Tradeoffs

Advantages:

* simple state propagation
* fewer return calculations

Disadvantages:

* less natural for many problems

---

## 6. Exercises

1. Parameterised factorial.
2. Parameterised product of array.

System design:

Design recursive directory size calculation.

Optimization:

Turn parameter into mutable accumulator.

---

# 🔷 Topic 4: Recursion Tree & Call Stack

---

## 1. Conceptual Clarity

Lecture trace:

```text
f(3,0)
   ↓
f(2,3)
   ↓
f(1,5)
   ↓
f(0,6)
```

This forms a recursion tree.

---

### Call Stack

```text
Top
f(0,6)
f(1,5)
f(2,3)
f(3,0)
Bottom
```

---

## 2. Rust Implementation

You can visualize:

```rust
fn f(i: i32, sum: i32) {
    println!("Entering {} {}", i, sum);

    if i < 1 {
        println!("{}", sum);
        return;
    }

    f(i - 1, sum + i);

    println!("Leaving {}", i);
}
```

---

## 3. Real-World Applications

* DFS
* AST traversal
* file systems

---

## 4. System-Level Understanding

Stack frames are contiguous memory.

Good cache locality.

But large recursion risks:

```text
Stack Overflow
```

---

## 5. Engineering Tradeoffs

Deep trees → iteration preferred.

---

## 6. Exercises

1. Print entering/exiting order.
2. Draw recursion tree manually.

System design:

Estimate memory for DFS of 1 million nodes.

---

# 🔷 Topic 5: Functional Recursion

---

## 1. Conceptual Clarity

Functional recursion returns answers.

Striver's formula:

```text
sum(n) = n + sum(n-1)
```

Base:

```text
sum(0)=0
```

---

### Intuition

Instead of carrying answer:

```text
answer travels upward
```

---

### Complexity

```text
Time  O(N)
Space O(N)
```

---

## 2. Rust Implementation

```rust
fn sum(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    n + sum(n - 1)
}
```

Usage:

```rust
println!("{}", sum(3));
```

Output:

```text
6
```

---

### Common Rust Mistakes

Missing return type:

```rust
fn sum(n: i32)
```

Should be:

```rust
fn sum(n: i32) -> i32
```

---

## 3–6. Applications, Systems, Tradeoffs, Exercises

Functional recursion dominates:

* expression evaluation
* query planners
* AST traversal
* DP

Exercise:

Implement recursive Fibonacci.

System design:

Recursive evaluation of arithmetic expressions.

Optimization:

Memoization.

---

# 🔷 Topic 6: Returning Values Through Recursion

---

## 1. Conceptual Clarity

Striver explains:

```text
3 + f(2)
```

waits.

Then:

```text
2 + f(1)
```

waits.

Then:

```text
1 + f(0)
```

waits.

Finally:

```text
f(0)=0
```

returns upward.

---

### Return Flow

```text
0
↓
1
↓
3
↓
6
```

---

## System-Level Insight

This is called stack unwinding.

Used heavily in:

* compilers
* parsers
* interpreters

---

## Exercises

Implement recursive max element.

---

# 🔷 Topic 7: Breaking Problems into Smaller Subproblems

---

## 1. Conceptual Clarity

Core recursion mindset:

```text
Can I solve N
using solution of N-1?
```

For sum:

```text
sum(5)
=
5 + sum(4)
```

For factorial:

```text
fact(5)
=
5 * fact(4)
```

---

## Engineering Importance

This is exactly how:

* dynamic programming
* divide-and-conquer
* tree algorithms

are built.

---

## Exercises

1. Power(x,n)
2. Array sum recursively

System design:

Recursive dependency resolver.

---

# 🔷 Topic 8: Factorial Using Recursion

---

## 1. Conceptual Clarity

Definition:

```text
5! = 5×4×3×2×1
```

Recursive relation:

```text
fact(n)=n*fact(n-1)
```

---

### Correct Base Case

```text
fact(0)=1
```

or

```text
fact(1)=1
```

---

### Complexity

```text
Time O(N)
Space O(N)
```

---

## 2. Rust Implementation

```rust
fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    n * factorial(n - 1)
}
```

---

### Common Rust Mistakes

Overflow:

```rust
13!
```

already exceeds `i32`.

Prefer:

```rust
u64
```

or

```rust
u128
```

---

## 3. Real-World Applications

### Backend

Combinatorics services.

### Databases

Query optimization formulas.

### Blockchain

Cryptographic math.

### AI

Probability calculations.

### OS

Rare directly, but appears in combinatorial scheduling analysis.

---

## 4. System-Level Understanding

Multiplication chain:

```text
5×4×3×2×1
```

creates dependency chain.

No parallelism.

---

## 5. Engineering Tradeoffs

Large factorials require:

* BigInteger
* arbitrary precision arithmetic

---

## 6. Exercises

1. Parameterised factorial.
2. Tail-recursive factorial.

System design:

Service computing combinations nCr at scale.

Optimization:

Memoize repeated factorial calls.

---

# 🔷 Topic 9: Base Cases in Recursive Problems

---

## 1. Conceptual Clarity

Most important lesson from factorial.

Wrong:

```rust
if n == 0 {
    return 0;
}
```

Correct:

```rust
if n == 0 {
    return 1;
}
```

---

### Why?

Addition identity:

```text
0
```

Multiplication identity:

```text
1
```

---

## Engineering Insight

Choosing the wrong base case is the #1 recursion bug.

---

## Exercises

Find correct base cases for:

1. Fibonacci
2. Power
3. Binary search

---

# 🔷 Topic 10: Time Complexity of Recursion

---

## 1. Conceptual Clarity

Striver notes:

```text
1 call
2 call
3 call
...
N call
```

Each call performs constant work.

Therefore:

```text
O(N)
```

---

### Formula

```text
T(N)=T(N−1)+O(1)
```

Solution:

```text
O(N)
```

---

## System-Level Understanding

CPU executes:

```text
call
call
call
return
return
return
```

Function-call overhead matters.

---

## Applications

Used in analyzing:

* DFS
* recursion trees
* DP

---

## Exercises

Analyze complexity of:

1. Recursive sum
2. Recursive factorial

System design:

Estimate throughput loss from recursive APIs.

---

# 🔷 Topic 11: Space Complexity (Auxiliary Stack Space)

---

## 1. Conceptual Clarity

Every recursive call stays on stack.

For:

```text
sum(5)
```

Stack:

```text
sum(5)
sum(4)
sum(3)
sum(2)
sum(1)
sum(0)
```

Total:

```text
O(N)
```

space.

---

## 2. Rust Example

```rust
fn sum(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    n + sum(n - 1)
}
```

Uses:

```text
O(N)
```

stack memory.

---

## 3. Real-World Applications

### Backend

Recursive parsers.

### Databases

B-tree traversals.

### Blockchain

Merkle proofs.

### AI

Computation graph traversal.

### OS

Recursive directory walking.

---

## 4. System-Level Understanding

Stack frame contains:

```text
Arguments
Local variables
Return address
Registers
```

Large recursion:

```text
Stack Overflow
```

---

## 5. Engineering Tradeoffs

Use iteration when:

* depth is huge
* production reliability matters

---

## 6. Exercises

1. Measure recursion depth.
2. Rewrite recursively solved problems iteratively.

System design:

Design DFS for billion-node graph without recursion.

---

# Final Summary Table

| Topic                              | Time Complexity    | Space Complexity | Real-World Use           |
| ---------------------------------- | ------------------ | ---------------- | ------------------------ |
| Recursion Fundamentals             | Depends            | Depends          | Tree traversal           |
| Sum of First N Numbers             | O(N)               | O(N)             | Aggregations             |
| Parameterised Recursion            | O(N)               | O(N)             | Running metrics          |
| Recursion Tree                     | O(N) visualization | O(N)             | DFS analysis             |
| Functional Recursion               | O(N)               | O(N)             | Expression evaluation    |
| Returning Values Through Recursion | O(N)               | O(N)             | Interpreters & parsers   |
| Problem Decomposition              | Depends            | Depends          | Dynamic Programming      |
| Factorial Recursion                | O(N)               | O(N)             | Combinatorics            |
| Base Cases                         | O(1) logic         | O(1)             | Correct recursive design |
| Time Complexity Analysis           | O(N) here          | —                | Algorithm analysis       |
| Stack Space Analysis               | —                  | O(N)             | Production reliability   |

The central lesson of this lecture is the distinction between **Parameterised Recursion (state flows downward through parameters)** and **Functional Recursion (answers flow upward through return values)**. Almost every advanced recursive algorithm in trees, dynamic programming, parsers, databases, blockchains, and operating systems is built on one of these two patterns. 

---

# addition-1

---

Q how is a for loop is different from recursion like at the end both are in stack and if we did somthing large we should get stack full so stack overflow right ?what does actuall time and space complexity does here like ok o(n) but what ?and what is a binary search and what is power here in exercise

---

Great questions. Most beginners get confused here because people say both recursion and loops are "O(N)", but the memory behavior is very different.

---

# 1. Why doesn't a for loop cause stack overflow?

Consider:

```rust
for i in 1..=1_000_000 {
    println!("{}", i);
}
```

### What happens?

There is only **one function call**:

```text
main()
```

Stack:

```text
┌─────────┐
│ main()  │
└─────────┘
```

The variable `i` is reused every iteration.

Memory doesn't keep growing.

---

Now recursion:

```rust
fn f(n: i32) {
    if n == 0 {
        return;
    }

    f(n - 1);
}
```

Calling:

```rust
f(1_000_000);
```

creates:

```text
f(1000000)
f(999999)
f(999998)
...
f(1)
f(0)
```

Stack:

```text
┌───────────┐
│ f(0)      │
├───────────┤
│ f(1)      │
├───────────┤
│ f(2)      │
├───────────┤
│ ...       │
├───────────┤
│ f(1000000)│
└───────────┘
```

Every call gets its own stack frame.

Eventually stack memory fills.

Boom:

```text
Stack Overflow
```

---

# 2. "But both are O(N), so what's the difference?"

You're mixing time complexity and space complexity.

---

## For Loop

```rust
for i in 1..=n
```

### Time

Runs N times.

```text
O(N)
```

### Space

Only stores:

```text
i
n
```

Constant amount.

```text
O(1)
```

---

## Recursion

```rust
f(n)
```

### Time

Also runs N times.

```text
O(N)
```

### Space

Stores:

```text
f(n)
f(n-1)
f(n-2)
...
```

N stack frames.

```text
O(N)
```

---

So:

| Method    | Time | Space |
| --------- | ---- | ----- |
| For Loop  | O(N) | O(1)  |
| Recursion | O(N) | O(N)  |

This is the huge difference.

---

# 3. What does O(N) actually mean here?

When we say:

```text
Time Complexity = O(N)
```

we mean:

> If N doubles, the amount of work roughly doubles.

Example:

```rust
sum(100)
```

100 additions.

```rust
sum(1000)
```

1000 additions.

10× more work.

---

When we say:

```text
Space Complexity = O(N)
```

we mean:

> Memory grows proportional to N.

Example:

```rust
sum(10)
```

10 stack frames.

```rust
sum(1000)
```

1000 stack frames.

100× more memory.

---

# 4. Then why use recursion at all?

Because some problems are naturally recursive.

Example:

### Directory Structure

```text
root
 ├─ folderA
 │   ├─ file1
 │   └─ file2
 └─ folderB
     └─ file3
```

To visit everything:

```text
visit(root)
  visit(folderA)
     visit(file1)
     visit(file2)
  visit(folderB)
     visit(file3)
```

Recursion is very natural.

---

Other examples:

* Binary Trees
* DFS Graph Traversal
* Backtracking
* Dynamic Programming
* Merge Sort
* Quick Sort

---

# 5. What is Binary Search?

Suppose array is sorted:

```text
[1,3,5,7,9,11,13]
```

Find:

```text
9
```

Instead of checking one by one:

```text
1
3
5
7
9
```

(Binary search doesn't do this.)

---

It checks middle:

```text
7
```

Is 9 bigger?

Yes.

Throw away left half.

```text
[9,11,13]
```

Middle:

```text
11
```

Is 9 smaller?

Yes.

Throw away right half.

```text
[9]
```

Found.

---

### Complexity

Normal Search:

```text
O(N)
```

Binary Search:

```text
O(log N)
```

Because every step removes half the data.

---

Example:

```text
1,000,000 elements

Step 1 -> 500,000
Step 2 -> 250,000
Step 3 -> 125,000
...
```

Only about:

```text
20 comparisons
```

Needed.

That's insanely fast.

---

# 6. What is "Power" in the exercise?

I meant:

```text
x^n
```

Examples:

```text
2^3 = 8

2 × 2 × 2
```

Recursive idea:

```text
power(2,3)

=
2 * power(2,2)

=
2 * 2 * power(2,1)

=
2 * 2 * 2 * power(2,0)

=
2 * 2 * 2 * 1

=
8
```

Base case:

```rust
if n == 0 {
    return 1;
}
```

because:

```text
x^0 = 1
```

---

# One more important thing

When you asked earlier:

> "each thread has its own stack?"

Yes.

Each thread usually has:

```text
Thread
 ├─ Stack
 └─ Heap (shared process heap)
```

If Thread A recursively calls 1 million functions:

```text
Thread A stack overflow
```

Thread B's stack is unaffected.

That's why recursion depth matters per thread.

This stack-vs-recursion memory model is one of the most important concepts to understand before learning trees, DFS, backtracking, and dynamic programming. Once this clicks, recursion becomes much less mysterious.

---

# Addition-2

---

Q so then arry is in order in binary or else loop would have been better

---

Let's visualize it first before code.

Suppose:

```text
Index: 0 1 2 3 4 5 6
Array: 1 3 5 7 9 11 13
```

Find:

```text
9
```

---

### Step 1

```text
[1 3 5 7 9 11 13]
       ↑
      mid=3
      value=7
```

Compare:

```text
9 > 7
```

So we know:

```text
1 3 5
```

can never contain 9.

Throw them away.

---

### Step 2

```text
[9 11 13]
 ↑  ↑   ↑
 l mid  r
```

Middle is:

```text
11
```

Compare:

```text
9 < 11
```

Throw away:

```text
11 13
```

---

### Step 3

```text
[9]
 ↑
mid
```

Found.

---

The algorithm keeps track of:

```text
left
right
mid
```

Initially:

```text
left = 0
right = 6
```

Then:

```text
mid = (left + right)/2
```

---

For our example:

### Iteration 1

```text
left=0
right=6

mid=(0+6)/2
mid=3
```

```text
arr[3]=7
```

Since:

```text
9 > 7
```

move left:

```text
left = mid + 1

left = 4
```

---

### Iteration 2

```text
left=4
right=6

mid=(4+6)/2
mid=5
```

```text
arr[5]=11
```

Since:

```text
9 < 11
```

move right:

```text
right = mid - 1

right = 4
```

---

### Iteration 3

```text
left=4
right=4

mid=4
```

```text
arr[4]=9
```

Found.

---

In Rust it looks like:

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}
```

But don't memorize it yet.

The important thing to understand is:

```text
Linear Search:
1 3 5 7 9 11 13
↑ ↑ ↑ ↑ ↑
checks one by one

O(N)
```

vs

```text
Binary Search:
1 3 5 7 9 11 13
      ↑

Throw away HALF

9 11 13
  ↑

Throw away HALF

9

O(log N)
```

Every step removes **50% of the remaining search space**. That's the entire magic of binary search. Once you're comfortable with recursion, you'll see that binary search can also be written recursively, which is a very common DSA interview question.


