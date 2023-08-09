mod stack;
use stack::stack::Stack;
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_stack() {
    let mut  s = Stack::new();
    s.push(1);
    s.push(1);
    s.push(1);
    assert_eq!(s.len(), 3);
}
