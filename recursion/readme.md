# recursion 

 A function calling itself until a stopping condition occur
 
---
 f()
 └── f()
      └── f()
           └── f()
           
---
it is to ensure a big problem reduce into smaller version of itslef 

## maths representation

eg 5!

5!=5*4!

again now

4!=4*3!

and so on

---
Every recursive algorithm needs one.

Examples:

Trees
Graphs
Parsing
Dynamic Programming

---

# transcript

# Recursion — Complete Deep-Dive (Topic-by-Topic)

The transcript covers the following topics in order: 

1. Function Prerequisite
2. Recursion
3. Infinite Recursion
4. Function Calls & Execution Flow
5. Stack Space
6. Stack Overflow
7. Base Condition (Base Case)
8. Return Statement in Recursion
9. Recursive Backtracking / Returning
10. Recursion Tree

---

# 🔷 Topic 1: Functions (Prerequisite)

The instructor states that understanding functions is the only prerequisite for recursion. 

## 1. Conceptual Clarity

A function is a reusable block of code.

Think of it as:

* Input
* Processing
* Output

Example:

```text
add(2, 3)
```

returns

```text
5
```

A function call transfers control to another section of code.

### C++ vs Rust

C++:

```cpp
int add(int a, int b)
```

Rust:

```rust
fn add(a: i32, b: i32) -> i32
```

Rust enforces:

* Ownership
* Borrowing
* Lifetime checks

before compilation.

This prevents many runtime bugs.

---

## 2. Rust Implementation

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("{}", result);
}
```

### Beginner Mistakes

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

Works fine.

But:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}
```

fails because semicolon turns expression into `()`.

---

## 3. Real World Applications

### Backend

Request handlers

### Database

Query execution routines

### Blockchain

Transaction validation functions

### AI

Tensor operation functions

### OS

System calls

---

## 4. System-Level Understanding

Function call creates a stack frame.

Contains:

* Parameters
* Return address
* Local variables

CPU jumps to function address.

---

## 5. Engineering Tradeoffs

Functions improve:

* Reusability
* Maintainability

Cost:

* Function call overhead

Modern compilers often inline small functions.

---

## 6. Exercises

### Coding

Implement:

```rust
multiply(a, b)
```

### System Design

How would a web server organize thousands of request handlers?

---

# 🔷 Topic 2: Recursion

Definition from transcript:

> A function calling itself. 

---

## 1. Conceptual Clarity

Recursion means:

```text
Function
    ↓
calls itself
    ↓
calls itself
    ↓
calls itself
```

until some stopping condition occurs.

Example:

```text
f()
 └── f()
      └── f()
           └── f()
```

Key idea:

A big problem is reduced into a smaller version of itself.

---

### Mathematical View

Factorial:

```text
5!
=
5 × 4!
```

Again:

```text
4!
=
4 × 3!
```

Same problem repeatedly.

---

### Complexity

Depends on recursion pattern.

Example:

```text
f(n)
```

calling

```text
f(n-1)
```

Time:

```text
O(n)
```

Space:

```text
O(n)
```

because of call stack.

---

### C++ vs Rust

C++:

```cpp
void f() {
    f();
}
```

Rust:

```rust
fn f() {
    f();
}
```

Behavior is same.

Rust does not automatically optimize recursion.

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

### Ownership Note

Recursive functions often pass:

```rust
&Vec<T>
```

instead of

```rust
Vec<T>
```

to avoid moving ownership repeatedly.

Bad:

```rust
fn solve(v: Vec<i32>)
```

Good:

```rust
fn solve(v: &[i32])
```

---

## 3. Real-World Applications

### Backend

Directory traversal

### Database

Query planning trees

### Blockchain

Merkle tree traversal

### AI

Decision trees

### OS

Filesystem walking

---

## 4. System-Level Understanding

Each recursive call creates a stack frame.

Memory grows downward on most architectures.

More recursion:

```text
More stack memory
```

---

## 5. Engineering Tradeoffs

Choose recursion when:

* Problem naturally recursive

Avoid when:

* Very deep depth

---

## 6. Exercises

### Coding

Print numbers:

```text
10 → 1
```

recursively.

### System Design

How would you recursively crawl directories in Linux?

---

# 🔷 Topic 3: Infinite Recursion

Explained in transcript through repeatedly calling the same function without a stopping condition. 

---

## 1. Conceptual Clarity

Bad recursion:

```rust
fn f() {
    println!("1");
    f();
}
```

No stop.

Never ends.

---

### Result

```text
f
 └── f
      └── f
           └── f
                ...
```

Eventually memory runs out.

---

## 2. Rust Implementation

```rust
fn f() {
    println!("1");
    f();
}
```

Do NOT run.

Will crash.

---

## 3. Real World Examples

Infinite recursion bugs occur in:

### Backend

Recursive API serializers

### Database

Faulty tree traversal

### Blockchain

Cycle detection bugs

---

## 4. System-Level Understanding

Stack grows until:

```text
Stack Limit Reached
```

OS kills process.

---

## 5. Engineering Tradeoffs

No valid production use.

Always require stopping condition.

---

## 6. Exercises

Find bug:

```rust
fn countdown(n: i32) {
    countdown(n - 1);
}
```

---

# 🔷 Topic 4: Function Calls & Execution Flow

The transcript repeatedly traces how execution moves from `main()` into recursive calls. 

---

## 1. Conceptual Clarity

When function is called:

```text
main
 ↓
f()
 ↓
f()
 ↓
f()
```

Execution jumps into newest call.

Earlier calls pause.

---

## 2. Rust Example

```rust
fn f(n: i32) {
    println!("Start {}", n);

    if n > 0 {
        f(n - 1);
    }

    println!("End {}", n);
}
```

Output:

```text
Start 3
Start 2
Start 1
Start 0
End 0
End 1
End 2
End 3
```

---

## 3. Applications

Important for:

* DFS
* Tree traversal
* Expression evaluation

---

## 4. System-Level

CPU stores:

* Return address
* Registers
* Local variables

inside stack frame.

---

# 🔷 Topic 5: Stack Space

Discussed extensively in transcript. 

---

## 1. Conceptual Clarity

Stack stores active function calls.

Example:

```text
main
 f
  f
   f
```

Topmost call executes.

---

### Memory Layout

```text
Heap
↑

Free Space

↓

Stack
```

---

## 2. Rust Example

```rust
fn f(n: i32) {
    if n == 0 {
        return;
    }

    f(n - 1);
}
```

Depth = stack usage.

---

## 3. Real World Applications

### Backend

Request processing chain

### Databases

Query execution stack

### Blockchain

Transaction validation

### OS

Kernel call stack

---

## 4. System-Level

Stack access is extremely fast.

Reason:

```text
Contiguous memory
```

Excellent cache locality.

---

## 5. Tradeoffs

Pros:

* Fast allocation

Cons:

* Limited size

---

## 6. Exercise

Estimate stack depth for:

```rust
f(10000)
```

---

# 🔷 Topic 6: Stack Overflow

The transcript explains stack overflow as too many waiting recursive calls. 

---

## 1. Conceptual Clarity

If stack becomes full:

```text
Stack Overflow
```

occurs.

---

### Example

```rust
fn f() {
    f();
}
```

creates infinite frames.

---

## 2. Rust Example

```rust
fn recurse() {
    recurse();
}
```

Crash.

---

## 3. Real-World Examples

### Linux

Kernel prevents uncontrolled stack growth.

### Databases

Avoid deep recursion in query planners.

### Ethereum

Smart contracts impose gas limits partly for this reason.

---

## 4. System-Level

OS allocates finite stack:

Common sizes:

```text
1 MB
8 MB
16 MB
```

When exceeded:

```text
SIGSEGV
```

or panic.

---

## 5. Tradeoffs

Recursion simplicity vs stack risk.

---

## 6. Exercise

Convert recursive DFS to iterative DFS using stack.

---

# 🔷 Topic 7: Base Condition (Base Case)

The most important recursion concept. 

---

## 1. Conceptual Clarity

Base Case:

```text
Condition where recursion stops
```

Example:

```rust
if n == 0 {
    return;
}
```

Without base case:

Infinite recursion.

---

### Factorial

```text
0! = 1
```

This is the base case.

---

## 2. Rust Implementation

```rust
fn print_numbers(n: i32) {
    if n == 0 {
        return;
    }

    println!("{}", n);

    print_numbers(n - 1);
}
```

---

### Beginner Mistake

Wrong:

```rust
if n == 1 {
    return;
}

print_numbers(n);
```

No progress.

Infinite recursion.

---

## 3. Applications

Every recursive algorithm needs one.

Examples:

* Trees
* Graphs
* Parsing
* Dynamic Programming

---

## 4. System-Level

Base case enables stack unwinding.

Without it:

Stack overflow.

---

## 5. Tradeoffs

Too early:

Wrong result.

Too late:

Overflow.

---

## 6. Exercise

Write recursive sum:

```text
1 + 2 + ... + n
```

---

# 🔷 Topic 8: Return Statement in Recursion

Explained when recursion hits base condition and returns. 

---

## 1. Conceptual Clarity

`return` terminates current function call.

Example:

```rust
if n == 0 {
    return;
}
```

Control goes back to caller.

---

## 2. Rust Example

```rust
fn f(n: i32) {
    if n == 0 {
        return;
    }

    f(n - 1);
}
```

---

## 3. Applications

Used in:

* Search algorithms
* Tree traversals
* Parsing engines

---

## 4. System-Level

Return pops stack frame.

CPU jumps to stored return address.

---

# 🔷 Topic 9: Recursive Backtracking / Returning

Transcript shows calls returning one-by-one after base case. 

---

## 1. Conceptual Clarity

Going down:

```text
f(3)
f(2)
f(1)
f(0)
```

Coming back:

```text
f(0)
→ f(1)
→ f(2)
→ f(3)
```

This is stack unwinding.

---

## 2. Rust Example

```rust
fn f(n: i32) {
    if n == 0 {
        return;
    }

    f(n - 1);

    println!("{}", n);
}
```

Output:

```text
1
2
3
```

Printed during return phase.

---

## 3. Applications

### Databases

B-Tree traversal

### AI

Decision tree evaluation

### OS

Directory traversal

---

# 🔷 Topic 10: Recursion Tree

Introduced near the end. 

---

## 1. Conceptual Clarity

Visual representation of recursive calls.

Example:

```text
f(3)
|
f(2)
|
f(1)
|
f(0)
```

For Fibonacci:

```text
        f(5)
       /   \
    f(4)   f(3)
    / \     / \
```

This helps analyze complexity.

---

## 2. Rust Example

```rust
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}
```

---

## 3. Applications

### Backend

Task dependency graphs

### Databases

Query execution plans

### Blockchain

Merkle proofs

### AI

Search trees

### OS

Process trees

---

## 4. System-Level

Recursion tree often predicts:

* CPU work
* Memory usage
* Parallelization opportunities

---

## 5. Engineering Tradeoffs

Helpful for analysis.

Can reveal:

```text
Repeated computation
```

which motivates Dynamic Programming.

---

## 6. Exercise

Draw recursion tree for:

```rust
fib(4)
```

and count function calls.

---

# Summary Table

| Topic                  | Time Complexity    | Space Complexity     | Real-World Use              |
| ---------------------- | ------------------ | -------------------- | --------------------------- |
| Function               | O(1) call overhead | O(1) frame           | API handlers                |
| Recursion              | Problem dependent  | O(depth)             | Tree traversal              |
| Infinite Recursion     | Infinite           | Infinite until crash | Bug pattern                 |
| Function Call Flow     | O(depth)           | O(depth)             | DFS execution               |
| Stack Space            | O(depth) memory    | O(depth)             | Query execution             |
| Stack Overflow         | Crash state        | Stack exhausted      | Production failure analysis |
| Base Condition         | O(1) check         | O(1)                 | Every recursive algorithm   |
| Return Statement       | O(1)               | O(1)                 | Stack unwinding             |
| Recursive Backtracking | O(depth)           | O(depth)             | Tree & graph algorithms     |
| Recursion Tree         | Analysis tool      | Depends on tree      | Complexity estimation       |

The key takeaway from this lecture is:

> **Every recursive algorithm has three parts:**
>
> 1. **Base Case** (when to stop)
> 2. **Recursive Call** (smaller version of problem)
> 3. **Return/Unwinding Phase** (how answers propagate back)

These three ideas form the foundation for everything that follows in recursion: factorial, Fibonacci, tree traversals, backtracking, DFS, dynamic programming, and many advanced algorithms. 
