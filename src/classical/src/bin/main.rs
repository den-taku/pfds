use classical::{bheap::*, heap::*, lheap::*, red_black::*, set::*};
use std::rc::Rc;

fn main() {
    println!();
    println!("LeftistHeap:");
    let tree1 = LeftistHeap::<_, usize>::empty();
    if !tree1.is_empty() {
        panic!()
    }
    let tree2 = Rc::clone(&tree1).insert(3);
    println!("tree1: {:?}", tree1);
    println!("tree2: {:?}", tree2);
    let tree3 = Rc::clone(&tree2).insert(5);
    println!("tree3: {:?}", tree3);
    let tree4 = Rc::clone(&tree3).merge(Rc::clone(&tree2));
    println!("tree4: {:?}", tree4);
    println!("tree4.find_min(): {}", tree4.find_min());
    let tree5 = tree4.insert(7);
    let tree5 = tree5.insert(2);
    println!("tree5: {:?}", tree5);
    let tree6 = Rc::clone(&tree5).merge(tree5);
    println!("tree6: {:?}", tree6);
    println!("tree6.delete_min(): {:?}", Rc::clone(&tree6).delete_min());
    println!("tree6: {:?}", tree6);
    let tree7 = Rc::clone(&tree1)
        .insert(3)
        .insert(5)
        .insert(8)
        .insert(2)
        .merge(tree1.insert(9).insert(4));
    println!("tree7: {:#?}", tree7);
    println!("tree7.delete_min(): {:#?}", tree7.delete_min());
    println!();

    println!("BinomialHeap:");
    let tree1 = BinomialHeap::<_, usize>::empty();
    if !tree1.is_empty() {
        panic!()
    }
    let tree2 = Rc::clone(&tree1).insert(3);
    println!("tree1: {:?}", tree1);
    println!("tree2: {:?}", tree2);
    let tree3 = Rc::clone(&tree2).insert(5);
    println!("tree3: {:?}", tree3);
    let tree4 = Rc::clone(&tree3).merge(Rc::clone(&tree2));
    println!("tree4: {:?}", tree4);
    println!("tree4.find_min(): {}", tree4.find_min());
    let tree5 = tree4.insert(7);
    let tree5 = tree5.insert(2);
    println!("tree5: {:?}", tree5);
    let tree6 = Rc::clone(&tree5).merge(tree5);
    println!("tree6: {:?}", tree6);
    println!("tree6.delete_min(): {:?}", Rc::clone(&tree6).delete_min());
    println!("tree6: {:?}", tree6);
    let tree7 = Rc::clone(&tree1)
        .insert(3)
        .insert(5)
        .insert(8)
        .insert(2)
        .merge(tree1.insert(9).insert(4));
    println!("tree7: {:#?}", tree7);
    println!("tree7.delete_min(): {:#?}", tree7.delete_min());
    println!();

    println!("RedBlackTree:");
    let tree1 = RedBlackTree::empty();
    if !tree1.is_empty() {
        panic!()
    }
    let tree2 = Rc::clone(&tree1).insert("d");
    println!("tree1: {:?}", tree1);
    println!("tree2: {:?}", tree2);
    let tree3 = tree2.insert("b");
    let tree4 = tree3.insert("g");
    let tree5 = tree4.insert("a");
    let tree6 = tree5.insert("c");
    let tree7 = tree6.insert("f");
    let tree8 = Rc::clone(&tree7).insert("h");
    let tree9 = Rc::clone(&tree8).insert("e");
    println!("tree7: {:?}", tree7);
    println!("tree8: {:?}", tree8);
    println!("tree9: {:#?}", tree9);
    println!("tree9.member(\"c\") is {}", tree9.member("c"));
    println!("tree9.member(\"x\") is {}", tree9.member("x"));
    println!();
}
