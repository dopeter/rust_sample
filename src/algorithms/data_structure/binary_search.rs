pub type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V: std::fmt::Display> {
    pub left: TreeNode<K, V>,
    pub right: TreeNode<K, V>,
    pub key: K,
    pub value: V,
}

pub trait BinaryTree<K, V> {
    fn pre_order(&self);
    fn in_order(&self);
    fn pos_order(&self);
}

pub trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
    fn insert(&mut self, key: K, value: V);
}

impl<K, V: std::fmt::Display> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            left: None,
            right: None,
            value: value,
            key: key,
        }
    }
}

impl<K: PartialOrd+ std::fmt::Display, V: std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

impl<K: std::fmt::Display, V: std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
    fn pre_order(&self) {
        println!("{}:{}", self.key,self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{}:{}", self.key,self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }

    fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pos_order();
        }
        if let Some(ref right) = self.right {
            right.pos_order();
        }
        println!("{}:{}", self.key,self.value);
    }
}