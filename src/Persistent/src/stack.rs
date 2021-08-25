use List::*;

pub trait Stack<T> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(x: T, stack: Self) -> Self;
    fn head(&self) -> T;
    fn tail(stack: Self) -> Self;
}

#[derive(Debug, Clone)]
pub enum List<T: Copy> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> Stack<T> for List<T>
where
    T: Copy,
{
    fn empty() -> Self {
        Nil
    }
    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }
    fn cons(x: T, stack: Self) -> Self {
        Cons(x, Box::new(stack))
    }
    fn head(&self) -> T {
        match self {
            Nil => panic!(),
            Cons(v, _) => *v,
        }
    }
    fn tail(stack: Self) -> Self {
        match stack {
            Nil => panic!(),
            Cons(_, tail) => *tail,
        }
    }
}
