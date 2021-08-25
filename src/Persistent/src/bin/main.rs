use persistent::set::*;
use persistent::stack::*;
use std::rc::Rc;

fn main() {
    println!();
    println!("Stack: ");
    let stack1 = List::<usize>::empty();
    if !stack1.is_empty() {
        panic!()
    }
    let stack2 = Stack::cons(2, Rc::clone(&stack1));
    println!("stack2: {:?}", stack2);
    let stack3 = Stack::cons(3, Rc::clone(&stack2));
    println!("stack3.head(): {}", stack3.head());
    let stack4 = Stack::cons(4, Rc::clone(&stack3));
    println!(
        "tail(stack4.clone()): {:?}",
        Stack::tail(Rc::clone(&stack4))
    );
    println!(
        "suffixes(stack4.clone()): {:?}",
        suffixes(Rc::clone(&stack4))
    );
    println!("stack1: {:?}", stack1);
    println!("stack2: {:?}", stack2);
    println!("stack3: {:?}", stack3);
    println!("stack4: {:?}", stack4);
    println!();

    println!("BinarySearchTree:");
    let tree1 = Tree::empty();
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
    println!("tree9: {:?}", tree9);
    println!("{:?}", complete("d", 3));
    println!("tree9.member(\"c\") is {}", tree9.member("c"));
    println!("tree9.member(\"x\") is {}", tree9.member("x"));
    println!();
}
