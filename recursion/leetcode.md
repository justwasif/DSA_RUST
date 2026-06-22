Since you've just learned:

* Void recursion
* Functional recursion
* Reverse array/string recursively
* Palindrome checking recursively

here are some LeetCode problems in increasing difficulty.

---

# 🟢 Level 1 — Easy

### 1. Reverse String

🔗 LeetCode #344

**Problem**

Given a character array:

```rust
['h','e','l','l','o']
```

convert it to:

```rust
['o','l','l','e','h']
```

**Constraint**

Do it in-place.

**What you'll practice**

* Two pointers
* Single-pointer recursion
* Mutable borrowing

**Rust Function**

```rust
fn reverse_string(s: &mut Vec<char>)
```

---

### 2. Valid Palindrome

🔗 LeetCode #125

Example:

```text
"A man, a plan, a canal: Panama"
```

Output:

```text
true
```

Ignore:

* spaces
* punctuation
* case

---

### 3. Palindrome Number

🔗 LeetCode #9

```text
121 -> true
123 -> false
```

Can be solved recursively.

---

# 🟡 Level 2 — Build Recursion Intuition

### 4. Fibonacci Number

🔗 LeetCode #509

```rust
fn fib(n: i32) -> i32
```

Example:

```text
fib(5) = 5
```

Learn:

```text
f(n)=f(n-1)+f(n-2)
```

---

### 5. Pow(x,n)

🔗 LeetCode #50

Example:

```text
2^10 = 1024
```

Recursive solution:

```text
x^n = (x^(n/2))²
```

Learn:

* Divide & Conquer
* Logarithmic recursion

---

### 6. Climbing Stairs

🔗 LeetCode #70

```text
n=3

1+1+1
1+2
2+1

Answer = 3
```

Great bridge to Dynamic Programming.

---

---
my process
recusion to karna hai 
constrain bhi n= 45 hai to overflow nahi hona chiyea
1 ya 2 clime karega
1= 1
2=1+1,2
3=1+1+1,1+2,2+1
4=1+1+1+1,2+1+1,1+2+1,1+1+2,2+2
so basicall f n steps is= f(n-1) +f(n-2)

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn output(typ:i32)->i32{
            if typ==1{
                return 1;
            }
            if typ==2{
                return 2;
            }

            output(typ-1)+output(typ-2)
            
        }
        output(n)

        
    }
}

a good sol but isse is its giving time limit error




# 🟠 Level 3 — Functional Recursion

### 7. Maximum Depth of Binary Tree

🔗 LeetCode #104

```rust
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32
```

Learn:

```text
depth =
1 + max(left,right)
```

This is one of the most important recursion problems ever.

---

### 8. Same Tree

🔗 LeetCode #100

Check if two trees are identical.

Learn:

```text
compare current nodes
compare left
compare right
```

---

### 9. Symmetric Tree

🔗 LeetCode #101

Tree version of palindrome.

Very important interview problem.

---

# 🔴 Level 4 — Backtracking

After mastering today's lecture:

### 10. Subsets

🔗 LeetCode #78

Input:

```text
[1,2,3]
```

Output:

```text
[]
[1]
[2]
[3]
[1,2]
[1,3]
[2,3]
[1,2,3]
```

Learn:

```text
Take
Not Take
```

recursion.

---

### 11. Permutations

🔗 LeetCode #46

Input:

```text
[1,2,3]
```

Output:

```text
All possible orderings
```

Learn:

```text
swap
recurse
backtrack
```

---

### 12. Generate Parentheses

🔗 LeetCode #22

One of the most famous recursion questions.

---

# Your Next Challenge

Try this without looking at solutions:

### Reverse String Using Recursion

```rust
fn reverse_string_rec(chars: &mut [char], left: usize) {
    // write this yourself
}
```

Input:

```rust
let mut chars = vec!['r','u','s','t'];
```

Expected:

```rust
['t','s','u','r']
```

Hints:

```text
Base Case:
left >= n/2

Work:
swap(left, n-left-1)

Recursive Call:
left+1
```

This is the closest LeetCode problem to the Striver lecture you just finished. After you solve it, move to LeetCode #125 (Valid Palindrome).
