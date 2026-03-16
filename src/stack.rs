pub struct Stack<T> {
    v: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.v.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.v.last()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
