pub mod bst;
pub mod linked_list;
pub mod stack;

#[cfg(test)]
mod tests {
    use crate::bst::Bst;
    use crate::linked_list::List;
    use crate::stack::Stack;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert!(!s.is_empty());
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn test_linked_list() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        assert_eq!(l.pop_back(), Some(2));
        assert!(!l.is_empty());
        l.push_front(4);
        assert_eq!(l.peek_back(), Some(&1));
        assert_eq!(l.peek_front(), Some(&4));
        l.push_front(6);
        l.push_front(10);
        println!("{}", l);
        assert_eq!(l.pop_front(), Some(10));
        assert_eq!(l.pop_front(), Some(6));
        assert_eq!(l.pop_front(), Some(4));
        assert_eq!(l.len(), 1);
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);
        assert!(l.is_empty());
    }

    #[test]
    fn test_bst() {
        let mut bst = Bst::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        println!("{}", bst);
        assert!(!bst.search(4));
        assert!(bst.search(2));
    }
}
