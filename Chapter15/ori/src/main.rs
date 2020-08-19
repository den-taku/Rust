use self::BinaryTree::*;

struct I32Range {
    start: i32,
    end: i32
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

// The state of an in-order traversal of a `BinaryTree`.

struct TreeIter<'a, T: 'a> {
    // A stack of references to tree nodes. Since we use `Vec`'s
    // `push` and `pop` method, the top of the stack is the end of the vector.

    // The node the iterator will visit next is at the top of the stack,
    // with those ancestors still unvisited below it. If the stack is empty,
    // the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce,
        // of finish the iteration.
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n
        };

        // The next node after this one is the leftmost child of 
        // this node's right child, so push the path from here down.
        self.push_left_edge(&node.right);

        // Produce a reference to this node's value.
        Some(&node.element)
    }
}

fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) 
    -> BinaryTree<T> 
{
    NonEmpty(Box::new(TreeNode { left, element, right }))
}

fn main() {
    let mut pi = 0.0;
    let mut numerator = 1.0;

    for k in (I32Range{ start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);

    assert_eq!(pi as f32, std::f32::consts::PI);

    // Build a small tree.
    let subtree_l = make_node(Empty, "mecha", Empty);
    let subtree_rl = make_node(Empty, "droid", Empty);
    let subtree_r = make_node(subtree_rl, "robot", Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);

    // Iterate over it.
    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    for e in &v {
        println!("{}", e);
    }
    for e in &tree.iter() 
                  .map(|name| format!("mega-{}", name))
                  .collect::<Vec<_>>() {
        println!("{}", e);
    }
}
