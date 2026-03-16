# Rust DSA

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

### Graph `src/graph.rs`

An undirected graph using `Rc<RefCell<Node<T>>>` for shared ownership and interior mutability.

```rust
pub struct Node<T> {
    pub data: T,
    pub neighbors: Vec<Rc<RefCell<Node<T>>>>,
}

pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<Node<T>>>>,
}
```

**Operations:**
| Method | Description | Complexity |
|--------|-------------|------------|
| `add_node(value) -> Rc<RefCell<Node<T>>>` | adds node, returns handle | O(1) |
| `add_edge(from, to)` | connects two nodes both ways | O(1) |
| `display()` | prints each node and its neighbors | O(n + e) |

**Rust concepts practiced:**
- `Rc<RefCell<Node<T>>>` — the classic shared ownership + mutation combo
- `Rc::clone` — multiple owners pointing to same node
- `borrow()` and `borrow_mut()` — reading vs mutating through shared ref
- Why `RefCell` is needed — nodes mutated after creation
- Why `Rc` is needed — multiple nodes share same neighbor
- Struct fields `pub` vs struct `pub` — independent visibility decisions

**Why Rc\<RefCell\<Node\>\> is needed:**
```rust
// problem 1 — multiple nodes point to same neighbor:
// A → B
// C → B    ← B has TWO owners! Box won't work here!
//            Rc = multiple owners ✅

// problem 2 — adding neighbors AFTER node is created:
let a = add_node(1); // created
let b = add_node(2); // created
add_edge(&a, &b);    // mutating a AFTER creation!
//                      Rc alone can't mutate!
//                      RefCell = interior mutability ✅
```

**How shared ownership looks in memory:**
```
add_edge(n1, n2)
add_edge(n3, n2)  ← n2 shared between n1 AND n3!

graph.nodes ──────────────► [Node(1), Node(2), Node(3)]
                                  │        ▲        │
n1.neighbors ─────────────────────┘        │        │
n3.neighbors ───────────────────────────────────────┘
                                       same Node(2)!
                                       ref_count = 4
```

**Example:**
```rust
let mut g = Graph::new();
let n1 = g.add_node(1);
let n2 = g.add_node(2);
let n3 = g.add_node(3);
g.add_edge(&n1, &n2);
g.add_edge(&n1, &n3);
g.add_edge(&n3, &n2);
g.display();
// Node 1 → neighbors: 2 3
// Node 2 → neighbors: 1 3
// Node 3 → neighbors: 1 2
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

### Decision Guide

```
need heap alloc?               → Box<T>
multiple readers, one thread?  → Rc<T>
multiple readers, threads?     → Arc<T>
mutate Copy type via &self?    → Cell<T>
mutate any type via &self?     → RefCell<T>
shared AND mutable?            → Rc<RefCell<T>>
```

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

### Trait Bounds — Where to Put Them

```rust
// WRONG — bound on struct is too restrictive!
pub struct Bst<T: Ord> { ... }

// RIGHT — bound only where the trait is actually used!
pub struct Bst<T> { ... }
impl<T: Ord> Bst<T> { ... }
```

### Visibility Rules

```rust
pub struct Node<T> {     // struct visible outside
    data: T,             // field PRIVATE — only inside module!
    pub data: T,         // field PUBLIC — visible everywhere!
}
// struct pub and field pub are completely independent!
```

---

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Jon Gjengset YouTube](https://www.youtube.com/@jonhoo) — deep Rust videos
