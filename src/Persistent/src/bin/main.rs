use persistent::set::*;
use persistent::stack::*;

fn main() {
    println!("\nStack: ");
    let stack1 = List::<usize>::empty();
    if !stack1.is_empty() {
        panic!()
    }
    let stack2 = Stack::cons(2, stack1.clone());
    println!("stack2: {:?}", stack2);
    let stack3 = Stack::cons(3, stack2.clone());
    println!("stack3.head(): {}", stack3.head());
    let stack4 = Stack::cons(4, stack3.clone());
    println!("tail(stack4.clone()): {:?}", Stack::tail(stack4.clone()));
    println!("suffixes(stack4.clone()): {:?}", suffixes(stack4.clone()));
    println!("stack1: {:?}", stack1);
    println!("stack2: {:?}", stack2);
    println!("stack3: {:?}", stack3);
    println!("stack4: {:?}", stack4);
    println!("");

    println!("BinarySearchTree:");
    let tree1 = Tree::empty();
    let tree2 = tree1.clone().insert("d");
    println!("tree1: {:?}", tree1);
    println!("tree2: {:?}", tree2);
    let tree3 = tree2.insert("b");
    let tree4 = tree3.insert("g");
    let tree5 = tree4.insert("a");
    let tree6 = tree5.insert("c");
    let tree7 = tree6.insert("f");
    let tree8 = tree7.clone().insert("h");
    let tree9 = tree8.clone().insert("e");
    println!("tree7: {:?}", tree7);
    println!("tree8: {:?}", tree8);
    println!("tree9: {:?}", tree9);
    println!("{:?}", complete("d", 3));
    println!("tree9.member(\"c\") is {}", tree9.member("c"));
    println!("tree9.member(\"x\") is {}", tree9.member("x"));
    println!("");
}
