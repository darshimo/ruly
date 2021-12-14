use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
enum StackInner<S, T> {
    Dummy,
    Nil(S),
    Cons(Box<Stack<S, T>>, String, T),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stack<S, T>(StackInner<S, T>);

impl<S: fmt::Display, T: fmt::Display> fmt::Display for Stack<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            StackInner::Nil(h) => {
                write!(f, "{}", h)
            }
            StackInner::Cons(t, s, h) => {
                write!(f, "{} {} {}", t, s, h)
            }
            _ => panic!(),
        }
    }
}

impl<S, T> Stack<S, T> {
    pub fn new(s: S) -> Self {
        Stack(StackInner::Nil(s))
    }

    pub fn pop(self) -> Result<(Self, String, T), S> {
        match self.0 {
            StackInner::Nil(s) => Err(s),
            StackInner::Cons(next, infix, t) => Ok((*next, infix, t)),
            _ => panic!(),
        }
    }

    pub fn pop_ref<'a>(&'a self) -> Result<(&'a Self, String, &'a T), &'a S> {
        match &self.0 {
            StackInner::Nil(s) => Err(s),
            StackInner::Cons(next, infix, t) => Ok((next, infix.clone(), t)),
            _ => panic!(),
        }
    }

    pub fn push(&mut self, infix: String, t: T) {
        *self = Stack(StackInner::Cons(
            Box::new(std::mem::replace(self, Stack(StackInner::Dummy))),
            infix,
            t,
        ))
    }
}
