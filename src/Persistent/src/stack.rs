use std::rc::Rc;
use List::*;

pub trait Stack<T> {
    fn empty() -> Rc<Self>;
    fn is_empty(&self) -> bool;
    fn cons(x: T, stack: Rc<Self>) -> Rc<Self>;
    fn head(&self) -> T;
    fn tail(self: Rc<Self>) -> Rc<Self>;
}

#[derive(Debug, Clone)]
pub enum List<T: Copy> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> Stack<T> for List<T>
where
    T: Copy,
{
    fn empty() -> Rc<Self> {
        Rc::new(Nil)
    }
    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }
    fn cons(x: T, stack: Rc<Self>) -> Rc<Self> {
        Rc::new(Cons(x, stack.clone()))
    }
    fn head(&self) -> T {
        match self {
            Nil => panic!(),
            Cons(v, _) => *v,
        }
    }
    fn tail(self: Rc<Self>) -> Rc<Self> {
        match *self {
            Nil => panic!(),
            Cons(_, _) => self.clone(),
        }
    }
}
