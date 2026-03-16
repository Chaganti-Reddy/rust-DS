use std::fmt::Display;

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Display> Display for Bst<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::display_node(&self.root, f)
    }
}

impl<T: Ord + Display> Default for Bst<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Display> Bst<T> {
    pub fn new() -> Bst<T> {
        Bst { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(value)));
            }
            Some(n) if value < n.value => {
                Self::insert_node(&mut n.left, value);
            }
            Some(n) if value > n.value => {
                Self::insert_node(&mut n.right, value);
            }
            _ => {}
        }
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(node: &Option<Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(n) if value < n.value => Self::search_node(&n.left, value),
            Some(n) if value > n.value => Self::search_node(&n.right, value),
            _ => true,
        }
    }

    fn display_node(
        node: &Option<Box<Node<T>>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match node {
            None => Ok(()),
            Some(n) => {
                Self::display_node(&n.left, f)?;
                write!(f, "{} ", n.value)?;
                Self::display_node(&n.right, f)
            }
        }
    }
}
