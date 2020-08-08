// An ordered celloction of 'T's.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

// A part of a BinaryTee.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
} 

fn main() {
    let int = 2;
    match int {
        0 ... 2 => println!("0...2"),
        _       => ()
    }
    let tpl = (32, 32);
    match tpl {
        (a, b) if b < 30 => { println!("qawsedrftgyhujiolp( {}, {} )", a, b); }
        (a, b) if b > 30 => { println!("( {}, {} )", a, b); }
        _      => {}
    }
    match tpl {
        // x @ (a,b) => {} // err: pattern bindings after an `@` are unsttable.
        x @ (..) => { println!("( {}, {} )", x.0, x.1); }
    }

    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
}
