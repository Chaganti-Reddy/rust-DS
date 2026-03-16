use std::fmt::Display;
#[derive(Debug, Default)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    #[default]
    Nil,
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Nil => write!(f, "Nil"),
            List::Cons(val, next) => {
                write!(f, "{}->", val)?;
                next.fmt(f)
            }
        }
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::Nil
    }

    pub fn push_front(&mut self, value: T) {
        let old = std::mem::replace(self, List::Nil);
        *self = List::Cons(value, Box::new(old));
    }

    pub fn push_back(&mut self, value: T) {
        match self {
            List::Nil => *self = List::Cons(value, Box::new(List::Nil)),
            List::Cons(_, rest) => rest.push_back(value),
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old = std::mem::replace(self, List::Nil);

        match old {
            List::Nil => None,
            List::Cons(val, next) => {
                *self = *next;
                Some(val)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self {
            List::Nil => None,
            List::Cons(_, next) if matches!(**next, List::Nil) => {
                let old = std::mem::replace(self, List::Nil);
                match old {
                    List::Cons(val, _) => Some(val),
                    List::Nil => None,
                }
            }
            List::Cons(_, next) => next.pop_back(),
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(val, _) => Some(val),
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(val, next) if matches!(**next, List::Nil) => Some(val),
            List::Cons(_, next) => next.peek_back(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, next) => 1 + next.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }
}
