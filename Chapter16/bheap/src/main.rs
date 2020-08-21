use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
    if let Some(n) = heap.peek() {
        println!("max: {}", *n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    if let Some(n) = heap.pop() {
        println!("max: {}", n);
    }
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
    while let Some(task) = heap.pop() {
        handle(task);
    }

}
