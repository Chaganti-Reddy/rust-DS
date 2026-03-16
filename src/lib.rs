pub mod bst;
pub mod graphs;
pub mod linked_list;
pub mod stack;

#[cfg(test)]
mod tests {
    use crate::bst::Bst;
    use crate::graphs::Graph;
    use crate::linked_list::List;
    use crate::stack::Stack;

    #[test]
    fn test_stack_push_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);
        assert_eq!(s.peek(), Some(&20));
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.peek(), Some(&10));
    }

    #[test]
    fn test_stack_is_empty() {
        let mut s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        assert!(!s.is_empty());
        s.pop();
        assert!(s.is_empty());
    }

    #[test]
    fn test_linked_list_push_pop() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);
        assert_eq!(l.pop_back(), Some(3));
        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.pop_back(), Some(1));
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn test_linked_list_push_front() {
        let mut l = List::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);
    }

    #[test]
    fn test_linked_list_peek() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        assert_eq!(l.peek_front(), Some(&1));
        assert_eq!(l.peek_back(), Some(&2));
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn test_linked_list_mixed_operations() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        l.push_front(0);
        assert_eq!(l.len(), 3);
        assert_eq!(l.pop_front(), Some(0));
        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.len(), 1);
        assert!(!l.is_empty());
        assert_eq!(l.pop_front(), Some(1));
        assert!(l.is_empty());
    }

    #[test]
    fn test_linked_list_display() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);
        println!("{}", l);
    }

    #[test]
    fn test_bst_insert_search() {
        let mut bst = Bst::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(4);
        assert!(bst.search(5));
        assert!(bst.search(3));
        assert!(bst.search(7));
        assert!(bst.search(1));
        assert!(bst.search(4));
        assert!(!bst.search(0));
        assert!(!bst.search(6));
    }

    #[test]
    fn test_bst_duplicates() {
        let mut bst = Bst::new();
        bst.insert(5);
        bst.insert(5);
        bst.insert(5);
        assert!(bst.search(5));
    }

    #[test]
    fn test_bst_display_sorted() {
        let mut bst = Bst::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(4);
        println!("{}", bst);
    }

    #[test]
    fn test_graph_add_node() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        assert_eq!(n1.borrow().data, 1);
        assert_eq!(n2.borrow().data, 2);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(&n1, &n2);
        g.add_edge(&n1, &n3);

        assert_eq!(n1.borrow().neighbors.len(), 2);
        assert_eq!(n2.borrow().neighbors.len(), 1);
        assert_eq!(n3.borrow().neighbors.len(), 1);
    }

    #[test]
    fn test_graph_shared_ownership() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(&n1, &n2);
        g.add_edge(&n3, &n2);

        assert_eq!(
            n1.borrow().neighbors[0].borrow().data,
            n3.borrow().neighbors[0].borrow().data
        );
    }

    #[test]
    fn test_graph_display() {
        let mut g = Graph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(3);
        g.add_edge(&n1, &n2);
        g.add_edge(&n1, &n3);
        g.add_edge(&n3, &n2);
        g.display();
    }
}
