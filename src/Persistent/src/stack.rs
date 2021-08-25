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
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> Stack<T> for List<T>
where
    T: Clone,
{
    fn empty() -> Rc<Self> {
        Rc::new(Nil)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Nil)
    }

    fn cons(x: T, stack: Rc<Self>) -> Rc<Self> {
        Rc::new(Cons(x, stack))
    }

    fn head(&self) -> T {
        match self {
            Nil => panic!(),
            Cons(v, _) => v.clone(),
        }
    }

    fn tail(self: Rc<Self>) -> Rc<Self> {
        match &*self {
            Nil => panic!(),
            Cons(_, tail) => tail.clone(),
        }
    }
}

pub fn suffixes<T: Clone>(mut list: Rc<List<T>>) -> Rc<List<Rc<List<T>>>> {
    let mut suf = List::empty();
    while !list.is_empty() {
        suf = Stack::cons(list.clone().tail(), suf.clone());
        list = list.tail();
    }
    suf
}
