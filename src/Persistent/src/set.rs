use std::rc::Rc;
use Tree::*;

pub trait Set {
    type Element;

    fn empty() -> Rc<Self>;
    fn is_empty(&self) -> bool;
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

    fn is_empty(&self) -> bool {
        matches!(self, Leaf)
    }

    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self> {
        match &*self {
            Leaf => Rc::new(Node(Set::empty(), element, Set::empty())),
            Node(left, v, right) => {
                if v == &element {
                    self
                } else if v < &element {
                    Rc::new(Node(
                        Rc::clone(left),
                        v.clone(),
                        Rc::clone(right).insert(element),
                    ))
                } else {
                    Rc::new(Node(
                        Rc::clone(left).insert(element),
                        v.clone(),
                        Rc::clone(right),
                    ))
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
                    Rc::clone(right).member(element)
                } else {
                    Rc::clone(left).member(element)
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
        Rc::new(Node(Rc::clone(&t), x, t))
    }
}
