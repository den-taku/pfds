use std::rc::Rc;

pub trait Set {
    type Element;

    fn empty() -> Rc<Self>;
    fn is_empty(&self) -> bool;
    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self>;
    fn member(&self, element: Self::Element) -> bool;
}
