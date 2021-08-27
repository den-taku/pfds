use crate::set::Set;
use if_chain::if_chain;
use std::rc::Rc;
use Color::*;
use RedBlackTree::*;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub enum RedBlackTree<T> {
    Leaf,
    Node(Color, Rc<RedBlackTree<T>>, T, Rc<RedBlackTree<T>>),
}

impl<T> RedBlackTree<T>
where
    T: Clone,
{
    fn balance(self: Rc<Self>) -> Rc<Self> {
        if_chain! {
                if let Node(Black, left1, v1, right1) = &*self;
                if let Node(color2, left2, v2, right2) = &**left1;
                if let Node(color3, left3, v3, right3) = &**right1;
                then {
                    match (&color2, &**left2, &**right2, &color3, &**left3, &**right3) {
                        (Red, Node(Red, a, x, b), _, _, _, _) => Rc::new(Node(
                            Red,
                            Rc::new(Node(Black, Rc::clone(a), x.clone(), Rc::clone(b))),
                            v2.clone(),
                            Rc::new(Node(
                                Black,
                                Rc::clone(right2),
                                v1.clone(),
                                Rc::clone(right1),
                            )),
                        )),
                        (Red, _, Node(Red, b, y, c), _, _, _) => Rc::new(Node(
                            Red,
                            Rc::new(Node(Black, Rc::clone(left2), v2.clone(), Rc::clone(b))),
                            y.clone(),
                            Rc::new(Node(Black, Rc::clone(c), v1.clone(), Rc::clone(right1))),
                        )),
                        (_, _, _, Red, Node(Red, b, y, c), _) => Rc::new(Node(
                            Red,
                            Rc::new(Node(Black, Rc::clone(left2), v2.clone(), Rc::clone(b))),
                            y.clone(),
                            Rc::new(Node(Black, Rc::clone(c), v1.clone(), Rc::clone(right1))),
                        )),
                        (_, _, _, Red, _, Node(Red, c, z, d)) => Rc::new(Node(
                            Red,
                            Rc::new(Node(Black, Rc::clone(left2), v2.clone(), Rc::clone(right2))),
                            v3.clone(),
                            Rc::new(Node(Black, Rc::clone(c), z.clone(), Rc::clone(d))),
                        )),
                        _ => self,
                }
            } else {
                self
            }
        }
    }
}

impl<T> RedBlackTree<T>
where
    T: PartialOrd + Clone,
{
    fn ins(self: Rc<Self>, element: T) -> Rc<Self> {
        match &*self {
            Leaf => {
                let leaf = Self::empty();
                Rc::new(Node(Red, Rc::clone(&leaf), element, leaf))
            }
            Node(color, left, v, right) => {
                if &element < v {
                    Rc::new(Node(
                        *color,
                        Self::ins(Rc::clone(left), element),
                        v.clone(),
                        Rc::clone(right),
                    ))
                    .balance()
                } else if &element > v {
                    Rc::new(Node(
                        *color,
                        Rc::clone(left),
                        v.clone(),
                        Self::ins(Rc::clone(right), element),
                    ))
                } else {
                    self
                }
            }
        }
    }
}

impl<T> Set for RedBlackTree<T>
where
    T: PartialOrd + Clone,
{
    type Element = T;

    fn empty() -> Rc<Self> {
        Rc::new(Leaf)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Leaf)
    }

    fn insert(self: Rc<Self>, element: Self::Element) -> Rc<Self> {
        let tree = Self::ins(self, element);
        match &*tree {
            Leaf => Rc::new(Leaf),
            Node(_, left, v, right) => {
                Rc::new(Node(Black, Rc::clone(left), v.clone(), Rc::clone(right)))
            }
        }
    }

    fn member(&self, element: Self::Element) -> bool {
        match self {
            Leaf => false,
            Node(_, left, v, right) => {
                if v < &element {
                    right.member(element)
                } else if v == &element {
                    true
                } else {
                    left.member(element)
                }
            }
        }
    }
}
