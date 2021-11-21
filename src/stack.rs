use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Stack<T> {
    Nil,
    Cons(Rc<RefCell<Stack<T>>>, String, Rc<RefCell<T>>),
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stack::Nil => {
                write!(f, "")
            }
            Stack::Cons(t, s, h) => match &*t.borrow() {
                Stack::Nil => {
                    write!(f, "{}", h.borrow())
                }
                _ => {
                    write!(f, "{} {} {}", t.borrow(), s, h.borrow())
                }
            },
        }
    }
}
