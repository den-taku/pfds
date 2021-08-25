use std::rc::Rc;
use Tree::*;

pub trait Set {
    type Element;

    fn empty() -> Rc<Self>;
    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self>;
    fn member(&self, element: Self::Element) -> bool;
}

#[derive(Debug, Clone)]
pub enum Tree<T: PartialOrd + PartialEq + Clone> {
    Leaf,
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
}

impl<T> Set for Tree<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    type Element = T;
    fn empty() -> Rc<Self> {
        Rc::new(Leaf)
    }
    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self> {
        match &*self {
            Leaf => Rc::new(Node(Set::empty(), element, Set::empty())),
            Node(left, v, right) => {
                if v == &element {
                    self
                } else if v < &element {
                    Rc::new(Node(left.clone(), v.clone(), right.clone().insert(element)))
                } else {
                    Rc::new(Node(left.clone().insert(element), v.clone(), right.clone()))
                }
            }
        }
    }
    fn member(&self, element: Self::Element) -> bool {
        match &*self {
            Leaf => false,
            Node(left, v, right) => {
                if v == &element {
                    true
                } else if v < &element {
                    right.clone().member(element)
                } else {
                    left.clone().member(element)
                }
            }
        }
    }
}

pub fn complete<T: PartialEq + PartialOrd + Clone>(x: T, d: usize) -> Rc<Tree<T>> {
    if d == 0 {
        Tree::empty()
    } else {
        let t = complete(x.clone(), d - 1);
        Rc::new(Node(t.clone(), x.clone(), t.clone()))
    }
}
