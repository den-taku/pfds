use crate::heap::Heap;
use num_traits::identities::{One, Zero};
use std::rc::Rc;
use LeftistHeap::*;

#[derive(Debug, Clone)]
pub enum LeftistHeap<T, R>
where
    T: PartialOrd + Clone,
    R: PartialOrd + Copy + One + Zero,
{
    Leaf,
    Node(R, T, Rc<LeftistHeap<T, R>>, Rc<LeftistHeap<T, R>>),
}

impl<T, R> Heap for LeftistHeap<T, R>
where
    T: PartialOrd + Clone,
    R: PartialOrd + Copy + One + Zero,
{
    type Element = T;

    fn empty() -> Rc<Self> {
        Rc::new(Leaf)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Leaf)
    }

    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self> {
        let ept = Self::empty();
        Rc::new(Node(R::one(), element, Rc::clone(&ept), ept)).merge(self)
    }

    fn merge(self: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        match &*self {
            Leaf => rhs,
            Node(_, e1, left1, right1) => match &*rhs {
                Leaf => self,
                Node(_, e2, left2, right2) => {
                    if e1 < e2 {
                        Self::make_tree(e1.clone(), Rc::clone(left1), Rc::clone(right1).merge(rhs))
                    } else {
                        Self::make_tree(
                            e2.clone(),
                            Rc::clone(left2),
                            Rc::clone(&self).merge(Rc::clone(right2)),
                        )
                    }
                }
            },
        }
    }

    fn find_min(&self) -> Self::Element {
        match self {
            Leaf => panic!(),
            Node(_, v, _, _) => v.clone(),
        }
    }

    fn delete_min(self: Rc<Self>) -> Rc<Self> {
        match &*self {
            Leaf => panic!(),
            Node(_, _, left, right) => Rc::clone(left).merge(Rc::clone(right)),
        }
    }
}

impl<T, R> LeftistHeap<T, R>
where
    T: PartialOrd + Clone,
    R: PartialOrd + Copy + One + Zero,
{
    fn rank(&self) -> R {
        match self {
            Leaf => R::zero(),
            Node(r, _, _, _) => *r,
        }
    }

    fn make_tree(e: T, tree1: Rc<Self>, tree2: Rc<Self>) -> Rc<Self> {
        if tree1.rank() >= tree2.rank() {
            Rc::new(Node(tree2.rank() + R::one(), e, tree1, tree2))
        } else {
            Rc::new(Node(tree1.rank() + R::one(), e, tree2, tree1))
        }
    }
}
