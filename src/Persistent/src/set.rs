use std::rc::Rc;

pub trait Set {
    type Element;

    fn empty() -> Self;
    fn insert(self: Rc<Self>, element: Self::Element) -> Self;
    fn member(&self, element: Self::Element) -> bool;
}

pub enum Tree<T> {
    Leaf,
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
}
