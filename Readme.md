# Rust DSA — Learning Rust Through Data Structures

A collection of data structures implemented in Rust while learning the language deeply.
The goal is not just to implement DSA but to understand **why** Rust works the way it does —
ownership, borrowing, smart pointers, generics, and trait implementations.

---

## Implemented Data Structures

### Stack `src/stack.rs`

A generic LIFO stack backed by `Vec<T>`.

**Operations:**
| Method | Description | Complexity |
|--------|-------------|------------|
| `push(value)` | adds value to top | O(1) |
| `pop() -> Option<T>` | removes and returns top | O(1) |
| `peek() -> Option<&T>` | returns reference to top | O(1) |
| `is_empty() -> bool` | checks if stack is empty | O(1) |

**Rust concepts practiced:**
- Generics `<T>`
- Returning `Option<T>` vs `Option<&T>` — ownership vs borrowing
- `Vec<T>` as backing store

**Example:**
```rust
let mut s = Stack::new();
s.push(1);
s.push(2);
assert_eq!(s.pop(), Some(2));   // LIFO order
assert_eq!(s.peek(), Some(&1)); // borrows, doesn't take
assert_eq!(s.pop(), Some(1));
assert_eq!(s.pop(), None);      // empty — no panic!
```

---

### Linked List `src/linked_list.rs`

A generic singly linked list using recursive enum and `Box<T>`.

```rust
pub enum List<T> {
    Cons(T, Box<List<T>>),  // value + pointer to next node
    Nil,                     // end of list
}
```

**Operations:**
| Method | Description | Complexity |
|--------|-------------|------------|
| `push_front(value)` | adds to front | O(1) |
| `push_back(value)` | adds to back | O(n) |
| `pop_front() -> Option<T>` | removes from front | O(1) |
| `pop_back() -> Option<T>` | removes from back | O(n) |
| `peek_front() -> Option<&T>` | looks at front | O(1) |
| `peek_back() -> Option<&T>` | looks at back | O(n) |
| `len() -> usize` | counts nodes | O(n) |
| `is_empty() -> bool` | checks if empty | O(1) |

**Traits implemented:**
- `Display` — prints as `1->2->3->Nil`
- `Debug` — derived
- `Default` — returns empty list

**Rust concepts practiced:**
- `Box<T>` for heap allocation and recursive types
- `std::mem::replace` — moving out of borrowed `self`
- Recursive pattern matching
- Match guard patterns `if matches!(**next, List::Nil)`
- `Deref` coercion — calling methods through `Box` automatically
- Implementing `Display` trait manually with `?` operator
- Why enum beats struct for recursive data structures

**Why Box is needed:**
```rust
// without Box — infinite size! compiler error!
enum List<T> {
    Cons(T, List<T>),  // ERROR: recursive type has infinite size
    Nil,
}

// with Box — fixed size (just a pointer!)
enum List<T> {
    Cons(T, Box<List<T>>),  // size = T + pointer = known!
    Nil,
}
```

**Why mem::replace is needed:**
```rust
// can't move out of borrowed self directly!
fn push_front(&mut self, value: T) {
    let old = *self;  // ERROR: can't move out of &mut self!
}

// mem::replace atomically swaps — self is never invalid!
fn push_front(&mut self, value: T) {
    let old = std::mem::replace(self, List::Nil); // swap!
    *self = List::Cons(value, Box::new(old));      // rebuild!
}
```

**Example:**
```rust
let mut l = List::new();
l.push_back(1);
l.push_back(2);
l.push_front(0);
println!("{}", l);              // 0->1->2->Nil
assert_eq!(l.pop_front(), Some(0));
assert_eq!(l.pop_back(), Some(2));
assert_eq!(l.peek_front(), Some(&1));
assert_eq!(l.len(), 1);
```

---

### Binary Search Tree `src/bst.rs`

A generic BST using recursive `Option<Box<Node<T>>>` pattern.

```rust
pub struct Node<T> {
    value: T,
    left:  Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}
```

**BST Rules:**
- smaller than current node → go left
- larger than current node → go right
- equal → ignore (no duplicates)

**Operations:**
| Method | Description | Complexity |
|--------|-------------|------------|
| `insert(value)` | inserts following BST rules | O(log n) avg |
| `search(value) -> bool` | finds value in tree | O(log n) avg |
| `Display` | prints values in sorted order | O(n) |

**Traits implemented:**
- `Display` — inorder traversal (prints sorted!)
- `Default` — returns empty tree

**Rust concepts practiced:**
- `Option<Box<Node<T>>>` — `None` = no child, `Some` = child exists
- Trait bounds `T: Ord` — only on impl, not on struct
- Recursive static helper functions
- Inorder traversal — left → current → right = sorted output
- `Formatter` passed through recursive calls

**Why Ord bound only on impl:**
```rust
// struct just stores T — no comparison needed!
pub struct Bst<T> {       // no bound here!
    root: Option<Box<Node<T>>>,
}

// impl uses < and > — needs Ord here!
impl<T: Ord> Bst<T> {     // bound only where used!
    pub fn insert(...) { }
    pub fn search(...) { }
}
```

**Example:**
```rust
let mut bst = Bst::new();
bst.insert(5);
bst.insert(3);
bst.insert(7);
bst.insert(1);
bst.insert(4);
println!("{}", bst);         // 1 3 4 5 7  ← sorted!
assert!(bst.search(3));      // true
assert!(!bst.search(6));     // false
```

---

## Key Rust Concepts Learned

### Smart Pointers

| Type | Purpose | Use when |
|------|---------|----------|
| `Box<T>` | heap allocation, single owner | recursive types, large data |
| `Rc<T>` | shared ownership, single thread | multiple readers, one thread |
| `Arc<T>` | shared ownership, multi thread | multiple readers, across threads |
| `Cell<T>` | interior mutability, Copy types | simple get/set without refs |
| `RefCell<T>` | interior mutability, any type | need mut ref through shared ref |
| `Rc<RefCell<T>>` | shared ownership + mutation | multiple owners who can mutate |

### Ownership Rules That Came Up

```
1. can't move out of borrowed &mut self
   → solution: std::mem::replace

2. can't have two mutable borrows simultaneously
   → RefCell panics at runtime if violated

3. can't use value after move
   → solution: clone, borrow, or restructure

4. references must not outlive owner
   → solution: Rc<T> when lifetime is unclear
```

### Pattern Matching Insights

```rust
// & and * cancel out in patterns:
let &x = &5;  // x = 5

// match ergonomics — Rust auto-adds & in patterns:
match &some_value {
    SomeVariant(x) => ...  // x is auto &T
}

// guard patterns:
match node {
    Some(n) if matches!(**n, List::Nil) => { }
}
```

### Deref Coercion Chain

```
Box<List<T>>  →  deref  →  List<T>
&mut Box<T>   →  deref  →  &mut T
String        →  deref  →  str
&String       →  deref  →  &str
```

Rust auto-derefs for method calls but NOT for pattern matching.

---

## Running Tests

```bash
cargo test                    # run all tests
cargo test test_stack         # run specific test
cargo test -- --nocapture     # show println! output
```

---

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Jon Gjengset YouTube](https://www.youtube.com/@jonhoo) — deep Rust videos
