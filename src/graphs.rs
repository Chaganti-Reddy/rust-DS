use std::{cell::RefCell, fmt::Display, rc::Rc};

pub struct Node<T> {
    pub data: T,
    pub neighbors: Vec<Rc<RefCell<Node<T>>>>,
}

pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Display> Default for Graph<T> {
    fn default() -> Self {
        Graph::new()
    }
}

impl<T: Display> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }

    pub fn add_node(&mut self, data: T) -> Rc<RefCell<Node<T>>> {
        let node = Rc::new(RefCell::new(Node {
            data,
            neighbors: vec![],
        }));
        self.nodes.push(Rc::clone(&node));
        node
    }

    pub fn add_edge(&mut self, node1: &Rc<RefCell<Node<T>>>, node2: &Rc<RefCell<Node<T>>>) {
        node1.borrow_mut().neighbors.push(Rc::clone(node2));
        node2.borrow_mut().neighbors.push(Rc::clone(node1));
    }

    pub fn display(&self) {
        for node in &self.nodes {
            let n = node.borrow();
            print!("Node {} → neighbors: ", n.data);
            for neighbor in &n.neighbors {
                print!("{} ", neighbor.borrow().data);
            }
            println!();
        }
    }
}
