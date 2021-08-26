use std::rc::Rc;

pub trait Heap {
    type Element;

    fn empty() -> Rc<Self>;
    fn is_empty(&self) -> bool;
    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self>;
    fn merge(self: Rc<Self>, rhs: Rc<Self>) -> Rc<Self>;
    fn find_min(&self) -> Self::Element;
    fn delete_min(self: Rc<Self>) -> Rc<Self>;
}
