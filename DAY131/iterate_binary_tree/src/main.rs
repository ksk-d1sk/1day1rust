use self::BinaryTree::*;

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    fn insert(&mut self, value: T) {
        match *self {
            Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: Empty,
                    right: Empty,
                }));
            },
            NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.insert(value);
                } else {
                    node.right.insert(value);
                }
            }
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

impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(node) = tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.insert("jaeger");
    tree.insert("robot");
    tree.insert("droid");
    tree.insert("mecha");

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);

    assert_eq!(
        tree.iter()
            .map(|name| format!("mega-{}", name))
            .collect::<Vec<_>>(),
        ["mega-droid", "mega-jaeger", "mega-mecha", "mega-robot"]
    );
}