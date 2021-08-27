use classical::{bheap::*, heap::*, lheap::*};
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
    println!("tree7: {:?}", tree7);
    println!("tree7.delete_min(): {:?}", tree7.delete_min());
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
}
