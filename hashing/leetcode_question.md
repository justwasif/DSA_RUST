**Mostly yes—but not 100%.**

The transcript you uploaded teaches **only the fundamentals of hashing**:

* ✅ Frequency counting
* ✅ Array hashing
* ✅ Character hashing
* ✅ ASCII mapping
* ✅ `map` / `unordered_map` (`BTreeMap` / `HashMap` in Rust)
* ✅ Hash functions
* ✅ Collisions
* ✅ Chaining
* ✅ Highest/lowest frequency

So the problems that are **100% directly related** to your lecture are these:

| LeetCode # | Problem                            | Directly Covered?             |
| ---------- | ---------------------------------- | ----------------------------- |
| 1          | Two Sum                            | ✅                             |
| 217        | Contains Duplicate                 | ✅                             |
| 242        | Valid Anagram                      | ✅                             |
| 349        | Intersection of Two Arrays         | ✅                             |
| 350        | Intersection of Two Arrays II      | ✅                             |
| 383        | Ransom Note                        | ✅                             |
| 387        | First Unique Character in a String | ✅                             |
| 389        | Find the Difference                | ✅                             |
| 771        | Jewels and Stones                  | ✅                             |
| 1207       | Unique Number of Occurrences       | ✅                             |
| 169        | Majority Element                   | ✅ (frequency approach)        |
| 49         | Group Anagrams                     | ✅                             |
| 451        | Sort Characters by Frequency       | ✅                             |
| 347        | Top K Frequent Elements            | ✅                             |
| 705        | Design HashSet                     | ✅ (hash table implementation) |
| 706        | Design HashMap                     | ✅ (hash table implementation) |

---

## ❌ These are **NOT** directly from this lecture

They require **additional topics** that Striver teaches later.

### Sliding Window

* 3
* 30
* 76
* 424
* 438
* 567

Need:

* Sliding Window
* Two Pointers

---

### Prefix Sum + HashMap

* 560
* 523
* 525
* 930
* 325

Need:

* Prefix Sum
* Prefix XOR
* HashMap

---

### Design

* 146
* 380
* 460

Need:

* Linked List
* Doubly Linked List
* Randomization

---

### Greedy / Sorting

* 1838
* 621
* 846
* 1481
* 1814
* 2225

Need:

* Greedy
* Sorting
* Heap
* Ordered Map

---

# If I were following Striver A2Z exactly, I'd recommend solving **only these after this lecture**:

```text
1
217
242
349
350
383
387
389
771
1207
169
49
451
347
705
706
```

These 16 problems test almost every concept from the transcript:

* Frequency counting
* Character hashing
* Number hashing
* `HashMap`
* `HashSet`
* Collision intuition
* Basic hash table implementation

---

### My recommendation

Since I know you're following **Striver A2Z in Rust**, I recommend staying aligned with the course.

* **After the Hashing lecture:** solve only those **16 problems**.
* **After the Sliding Window lecture:** solve the sliding window hashing problems.
* **After the Prefix Sum lecture:** solve the prefix-sum + HashMap problems.

That way, every problem uses only concepts you've already learned, and you won't have to rely on techniques that haven't been introduced yet.
