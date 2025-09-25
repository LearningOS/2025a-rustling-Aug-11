/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(ref mut root_node) = self.root {
            root_node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // 如果树为空，返回false
        if let Some(ref root_node) = self.root {
            root_node.search(value)
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // 比较新值和当前节点值
        match value.cmp(&self.value) {
            // 如果新值小于当前节点值，插入到左子树
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    left_node.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            // 如果新值大于当前节点值，插入到右子树
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    right_node.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            },
            // 如果值相等，不做任何操作（二叉搜索树不允许重复值）
            Ordering::Equal => {}
        }
    }
    
    // Search for a value in the tree
    fn search(&self, value: T) -> bool {
        // 比较目标值和当前节点值
        match value.cmp(&self.value) {
            // 如果目标值小于当前节点值，在左子树中搜索
            Ordering::Less => {
                if let Some(ref left_node) = self.left {
                    left_node.search(value)
                } else {
                    false
                }
            },
            // 如果目标值大于当前节点值，在右子树中搜索
            Ordering::Greater => {
                if let Some(ref right_node) = self.right {
                    right_node.search(value)
                } else {
                    false
                }
            },
            // 如果值相等，找到目标值
            Ordering::Equal => true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}


