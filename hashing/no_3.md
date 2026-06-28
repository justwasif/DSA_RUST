# Part 3 — Character Hashing, ASCII, and Character-to-Index Mapping (Rust + Systems View)

Topics covered:

14. Character Hashing
15. Character Frequency Counting
16. ASCII Representation
17. Character → Index Mapping
18. Lowercase Character Hashing
19. Full ASCII Hashing (256 Characters)
20. Auto Type Conversion / ASCII Casting

Source: Character hashing section of the lecture. 

---

# 🔷 Topic 14: Character Hashing

---

## 1. Conceptual Clarity

Number hashing:

```text
1 -> count
2 -> count
3 -> count
```

Character hashing:

```text
'a' -> count
'b' -> count
'c' -> count
```

The goal is identical:

> Count how many times a character appears.

Example:

```text
"abcdabefc"
```

Queries:

```text
Count('a')
Count('c')
Count('z')
```

Answers:

```text
a -> 2
c -> 2
z -> 0
```

Instead of storing frequencies of numbers, we store frequencies of characters.

---

### Why It Matters

Many interview problems are secretly character hashing:

* Anagrams
* Palindromes
* Frequency Sort
* Most Frequent Character
* Sliding Window Problems

You will use this constantly.

---

### Complexity

Brute Force:

```text
O(Q × N)
```

Hashing:

```text
Build = O(N)
Query = O(1)
```

---

### C++ vs Rust

C++:

```cpp
char c = 'a';
```

Rust:

```rust
let c: char = 'a';
```

Rust's `char` is Unicode-aware.

C++ `char` is typically a single byte.

This difference becomes important later.

---

## 2. Rust Implementation

```rust
fn main() {
    let s = "abcdabefc";

    let mut freq = [0; 26];

    for ch in s.chars() {
        let idx = (ch as u8 - b'a') as usize;
        freq[idx] += 1;
    }

    println!("{}", freq[0]);
}
```

Output:

```text
2
```

---

### Beginner Mistake

Wrong:

```rust
ch - 'a'
```

Rust doesn't allow character arithmetic directly.

Need:

```rust
(ch as u8 - b'a')
```

---

## 3. Real World Applications

### Backend

Search analytics:

```text
Character frequencies
```

---

### Databases

Text indexing engines.

---

### Blockchain

Transaction memo analysis.

---

### AI

Tokenization preprocessing.

---

### Operating Systems

Command-line parsing.

---

## 4. System-Level Understanding

Characters are actually integers.

CPU sees:

```text
'a' = 97
'b' = 98
```

Hashing characters is really hashing numbers.

---

## 5. Engineering Tradeoffs

Array hashing:

✅ Fast

❌ Works best when character set is bounded

---

## 6. Exercises

Count frequencies of:

```text
"mississippi"
```

using character hashing.

---

# 🔷 Topic 15: Character Frequency Counting

---

## 1. Conceptual Clarity

Given:

```text
banana
```

We want:

```text
b -> 1
a -> 3
n -> 2
```

This is exactly the same problem as number frequency counting.

Only the key type changed.

---

### Brute Force

For every query:

```text
Scan entire string
```

Complexity:

```text
O(Q × N)
```

---

### Hashing

Build frequency once:

```text
O(N)
```

Answer instantly:

```text
O(1)
```

---

## 2. Rust Implementation

```rust
fn frequency(s: &str) -> [usize; 26] {
    let mut freq = [0; 26];

    for ch in s.chars() {
        let idx = (ch as u8 - b'a') as usize;
        freq[idx] += 1;
    }

    freq
}
```

---

### Ownership Discussion

Notice:

```rust
&str
```

Borrowed string.

No copy.

Efficient.

---

### Beginner Mistake

Using:

```rust
String
```

when:

```rust
&str
```

is enough.

---

## 3. Real World Applications

### Backend

Spam detection.

### Databases

Text statistics.

### Blockchain

Metadata analysis.

### AI

Bag-of-Words models.

### OS

Shell command parsing.

---

## 4. System-Level Understanding

Frequency counting is a streaming algorithm.

Characters arrive:

```text
a
b
c
a
```

Update count immediately.

No need to store entire history.

---

## 5. Engineering Tradeoffs

Memory:

```text
26 integers
```

Tiny.

Performance:

```text
Excellent
```

---

## 6. Exercises

Count:

```text
"programming"
```

---

# 🔷 Topic 16: ASCII Representation

---

## 1. Conceptual Clarity

The lecture introduces ASCII because hashing characters requires converting them to numbers. 

ASCII:

```text
American Standard Code for Information Interchange
```

Every character has a numeric value.

Examples:

```text
'A' = 65
'B' = 66

'a' = 97
'b' = 98

'0' = 48
'1' = 49
```

---

### Why ASCII Exists

Computers only understand binary:

```text
01000001
```

Human:

```text
A
```

ASCII bridges that gap.

---

## 2. Rust Implementation

```rust
fn main() {
    let c = 'a';

    println!("{}", c as u8);
}
```

Output:

```text
97
```

---

### Beginner Mistake

Confusing:

```rust
'a'
```

with

```rust
"a"
```

Rust:

```rust
'a'  // char
"a"  // string slice
```

Different types.

---

## 3. Real World Applications

Every text-processing system depends on character encoding.

---

## 4. System-Level Understanding

Memory stores:

```text
97
```

not

```text
'a'
```

The character is merely a human representation.

---

## 5. Engineering Tradeoffs

ASCII:

```text
128 characters
```

Modern systems use Unicode.

Rust chars are Unicode.

---

## 6. Exercises

Print ASCII values of:

```text
A
Z
a
z
0
9
```

---

# 🔷 Topic 17: Character → Index Mapping

---

## 1. Conceptual Clarity

Lecture's crucial formula:

```text
character - 'a'
```

Example:

```text
'f' - 'a'
```

ASCII:

```text
102 - 97
```

Result:

```text
5
```

Meaning:

```text
f maps to index 5
```

---

### Mapping Table

```text
a -> 0
b -> 1
c -> 2
...
z -> 25
```

This allows us to store frequencies in a 26-size array.

---

## 2. Rust Implementation

```rust
fn idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}
```

Examples:

```rust
idx('a') // 0
idx('c') // 2
idx('z') // 25
```

---

### Beginner Mistake

Forgetting:

```rust
as usize
```

Arrays require usize indexing.

---

## 3. Real World Applications

Compression systems.

Spell checkers.

Search engines.

Autocomplete.

---

## 4. System-Level Understanding

Mapping converts:

```text
Sparse characters
```

into

```text
Dense indexes
```

Dense indexes improve cache performance.

---

## 5. Engineering Tradeoffs

Array:

```text
26 slots
```

instead of:

```text
HashMap<char, usize>
```

Much faster.

---

## 6. Exercises

Implement:

```rust
char_to_index()
index_to_char()
```

---

# 🔷 Topic 18: Lowercase Character Hashing

---

## 1. Conceptual Clarity

Lecture assumes:

```text
Only lowercase letters.
```

Then:

```text
26 slots
```

are enough. 

---

Example:

```text
abcdabefc
```

Hash array:

```text
Index: 0 1 2 3 4 5 ...
Value: 2 2 2 1 1 1 ...
```

---

### Complexity

Build:

```text
O(N)
```

Query:

```text
O(1)
```

Space:

```text
O(26)
```

Effectively constant.

---

## 2. Rust Implementation

```rust
let mut freq = [0usize; 26];

for ch in s.chars() {
    freq[(ch as u8 - b'a') as usize] += 1;
}
```

---

### Beginner Mistake

Not validating input.

If:

```text
'A'
```

appears,

formula breaks.

---

## 3. Real World Applications

Word games.

Autocomplete.

Dictionary lookups.

---

## 4. System-Level Understanding

26 integers:

```text
26 × 8 bytes
```

Only:

```text
208 bytes
```

Fits entirely in CPU cache.

Extremely fast.

---

## 5. Engineering Tradeoffs

Perfect if:

```text
a-z only
```

Not suitable for Unicode.

---

## 6. Exercises

Count frequencies of:

```text
leetcode
```

---

# 🔷 Topic 19: Full ASCII Hashing (256 Characters)

---

## 1. Conceptual Clarity

Lecture says:

If input isn't restricted to lowercase,

use:

```text
256
```

size array. 

Then:

```text
freq[ascii_value]
```

directly.

---

Example:

```text
'A' -> 65
```

Store:

```text
freq[65]
```

---

### Why 256?

ASCII:

```text
0..255
```

Possible values.

---

## 2. Rust Implementation

```rust
let mut freq = [0usize; 256];

for b in s.bytes() {
    freq[b as usize] += 1;
}
```

---

### Why `.bytes()`?

More efficient than:

```rust
chars()
```

when ASCII only.

---

## 3. Real World Applications

Parsers.

Compilers.

Network protocols.

Log processing.

---

## 4. System-Level Understanding

256 entries:

```text
256 × 8 bytes
≈ 2 KB
```

Still tiny.

Still cache-friendly.

---

## 5. Engineering Tradeoffs

ASCII hashing:

✅ Fast

❌ Doesn't support arbitrary Unicode properly.

---

## 6. Exercises

Count all ASCII frequencies in:

```text
"Hello World!"
```

---

# 🔷 Topic 20: Auto Type Conversion / ASCII Casting

---

## 1. Conceptual Clarity

Lecture explains:

Characters automatically convert to integers when needed. 

Conceptually:

```text
'a'
```

becomes:

```text
97
```

---

### Example

```text
'f' - 'a'

102 - 97

= 5
```

---

## 2. Rust Implementation

Rust is stricter than C++.

C++:

```cpp
'a' - 'a'
```

works automatically.

Rust:

```rust
('a' as u8 - b'a')
```

must be explicit.

---

### Why?

Rust avoids hidden conversions.

Safer.

---

### Beginner Mistake

```rust
let idx = ch - 'a';
```

Compiler error.

Correct:

```rust
let idx = (ch as u8 - b'a') as usize;
```

---

## 3. Real World Applications

Every parser:

```text
Character → Numeric code
```

Conversion happens constantly.

---

## 4. System-Level Understanding

CPU never sees:

```text
'a'
```

CPU sees:

```text
97
```

Casting merely exposes that numeric representation.

---

## 5. Engineering Tradeoffs

Explicit casts:

✅ Safe

❌ Slightly more verbose

This is a common Rust design philosophy.

---

## 6. Exercises

Implement:

```rust
fn ascii_value(ch: char) -> u8
```

and print:

```text
ASCII('A')
ASCII('z')
```

---

# Part 3 Summary

| Topic                        | Time Complexity | Space          | Key Idea                           |
| ---------------------------- | --------------- | -------------- | ---------------------------------- |
| Character Hashing            | O(N+Q)          | O(26) / O(256) | Hash characters instead of numbers |
| Character Frequency Counting | O(N) build      | O(26)          | Count occurrences                  |
| ASCII Representation         | O(1)            | O(1)           | Characters are numbers             |
| Character→Index Mapping      | O(1)            | O(1)           | `ch - 'a'`                         |
| Lowercase Hashing            | O(N)            | O(26)          | Most common interview pattern      |
| Full ASCII Hashing           | O(N)            | O(256)         | Supports all ASCII chars           |
| ASCII Casting                | O(1)            | O(1)           | Convert char → integer index       |

### Rust DSA Interview Pattern to Memorize

```rust
let mut freq = [0usize; 26];

for ch in s.chars() {
    freq[(ch as u8 - b'a') as usize] += 1;
}
```

This exact pattern appears repeatedly in:

* Valid Anagram
* Ransom Note
* Permutation in String
* Find All Anagrams
* Longest Palindrome
* Frequency Sort Characters
* Sliding Window Problems

