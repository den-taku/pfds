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
    println!("");
}
