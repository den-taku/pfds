use std::rc::Rc;
use Tree::*;

pub trait Set {
    type Element;

    fn empty() -> Rc<Self>;
    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self>;
    fn member(self: Rc<Self>, element: Self::Element) -> bool;
}

pub enum Tree<T: PartialOrd + PartialEq> {
    Leaf,
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
}

impl<T> Set for Tree<T>
where
    T: PartialOrd + PartialEq,
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
                    self.clone()
                } else if v < &element {
                    right.clone().insert(element)
                } else {
                    left.clone().insert(element)
                }
            }
        }
    }
    fn member(self: Rc<Self>, element: Self::Element) -> bool {
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
