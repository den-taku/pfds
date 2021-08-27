use crate::{heap::Heap, stack::*};
use num_traits::identities::{One, Zero};
use std::rc::Rc;
use Tree::*;

#[derive(Debug, Clone)]
pub struct BinomialHeap<T: PartialOrd + Clone, R: One + Zero + Copy + PartialOrd>(
    Rc<List<Rc<Tree<T, R>>>>,
);

#[derive(Debug, Clone)]
pub enum Tree<T, R>
where
    T: PartialOrd + Clone,
    R: One + Zero + Copy + PartialOrd,
{
    Node(R, T, Rc<List<Rc<Self>>>),
}

impl<T, R> Heap for BinomialHeap<T, R>
where
    T: PartialOrd + Clone,
    R: One + Zero + Copy + PartialOrd,
{
    type Element = T;

    fn empty() -> Rc<Self> {
        Rc::new(BinomialHeap(List::empty()))
    }

    fn is_empty(&self) -> bool {
        match self {
            BinomialHeap(list) => matches!(&**list, List::Nil),
        }
    }

    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self> {
        Rc::new(BinomialHeap(Tree::insert_tree(
            Rc::new(Node(R::zero(), element, Rc::new(List::Nil))),
            match &*self {
                BinomialHeap(list) => Rc::clone(list),
            },
        )))
    }

    fn merge(self: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        match &*rhs {
            BinomialHeap(r_list) => match &**r_list {
                List::Nil => self,
                List::Cons(r_head, r_tail) => match &*self {
                    BinomialHeap(s_list) => match &**s_list {
                        List::Nil => rhs,
                        List::Cons(s_head, s_tail) => {
                            if s_head.rank() < r_head.rank() {
                                Rc::new(BinomialHeap(List::cons(
                                    Rc::clone(s_head),
                                    match &*Self::merge(
                                        Rc::new(BinomialHeap(Rc::clone(s_tail))),
                                        rhs,
                                    ) {
                                        BinomialHeap(list) => Rc::clone(&*list),
                                    },
                                )))
                            } else if s_head.rank() > r_head.rank() {
                                Rc::new(BinomialHeap(List::cons(
                                    Rc::clone(r_head),
                                    match &*Self::merge(
                                        self,
                                        Rc::new(BinomialHeap(Rc::clone(r_tail))),
                                    ) {
                                        BinomialHeap(list) => Rc::clone(&*list),
                                    },
                                )))
                            } else {
                                Rc::new(BinomialHeap(Tree::insert_tree(
                                    Tree::link(Rc::clone(s_head), Rc::clone(r_head)),
                                    match &*Self::merge(
                                        Rc::new(BinomialHeap(Rc::clone(s_tail))),
                                        Rc::new(BinomialHeap(Rc::clone(r_tail))),
                                    ) {
                                        BinomialHeap(list) => Rc::clone(&*list),
                                    },
                                )))
                            }
                        }
                    },
                },
            },
        }
    }

    fn find_min(&self) -> Self::Element {
        match self {
            BinomialHeap(list) => {
                let (t, _) = Tree::remove_min_tree(list);
                t.root()
            }
        }
    }

    fn delete_min(self: Rc<Self>) -> Rc<Self> {
        match &*self {
            BinomialHeap(list) => {
                let (t, rest) = Tree::remove_min_tree(list);
                match &*t {
                    Node(_, _, list) => Self::merge(
                        Rc::new(BinomialHeap(Rc::clone(list).reverse())),
                        Rc::new(BinomialHeap(rest)),
                    ),
                }
            }
        }
    }
}

impl<T, R> Tree<T, R>
where
    T: PartialOrd + Clone,
    R: One + Zero + Copy + PartialOrd,
{
    fn rank(&self) -> R {
        match self {
            Node(r, _, _) => *r,
        }
    }

    fn root(&self) -> T {
        match self {
            Node(_, element, _) => element.clone(),
        }
    }

    fn insert_tree(tree: Rc<Self>, list: Rc<List<Rc<Self>>>) -> Rc<List<Rc<Self>>> {
        match &*list {
            List::Nil => List::cons(tree, Stack::empty()),
            List::Cons(head, tail) => {
                if tree.rank() < head.rank() {
                    List::cons(tree, Rc::clone(tail))
                } else {
                    Self::insert_tree(Self::link(tree, Rc::clone(head)), Rc::clone(tail))
                }
            }
        }
    }

    fn link(tree1: Rc<Self>, tree2: Rc<Self>) -> Rc<Self> {
        match &*tree1 {
            Node(r1, element1, list1) => match &*tree2 {
                Node(r2, element2, list2) => {
                    if r1 != r2 {
                        panic!()
                    } else if element1 < element2 {
                        Rc::new(Node(
                            *r1 + R::one(),
                            element1.clone(),
                            List::cons(tree2, Rc::clone(list1)),
                        ))
                    } else {
                        Rc::new(Node(
                            *r1 + R::one(),
                            element2.clone(),
                            List::cons(tree1, Rc::clone(list2)),
                        ))
                    }
                }
            },
        }
    }

    fn remove_min_tree(list: &Rc<List<Rc<Self>>>) -> (Rc<Self>, Rc<List<Rc<Self>>>) {
        match &**list {
            List::Nil => panic!(),
            List::Cons(tree, rest) => match &**rest {
                List::Nil => (Rc::clone(tree), List::empty()),
                _ => {
                    let (r_tree, r_list) = Tree::remove_min_tree(&Rc::clone(rest));
                    if tree.root() < r_tree.root() {
                        (Rc::clone(tree), Rc::clone(rest))
                    } else {
                        (r_tree, List::cons(Rc::clone(tree), r_list))
                    }
                }
            },
        }
    }
}
