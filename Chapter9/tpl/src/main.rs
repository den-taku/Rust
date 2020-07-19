use std::cell::RefCell;

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least    { least    = &slice[i]; } 
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema { greatest, least }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}

fn main() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    let p = Point { x:3.2, y:3.2 };
    let q = p;
    println!("{}", p.x);

    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
   
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }

    let mut w = ref_cell.borrow_mut();
    w.push_str(" world");

    println!("Hello, world!");
}
