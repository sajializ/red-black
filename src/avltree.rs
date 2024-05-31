use crate::avlnode::{AVLNode, OptionNode};
use crate::node::Node;
use crate::tree::Tree;

pub struct AVLTree {
    _root: OptionNode,
}

impl Tree<AVLNode> for AVLTree {
    fn new() -> Self {
        Self {_root: None}
    }

    fn get_root(&self) -> &OptionNode {
        &self._root
    }

    fn insert(&mut self, key: i64) {
        match self._root.take() {
            Some(root) => self._root = AVLNode::insert(Some(root), key),
            None => self._root = AVLNode::new(key),
        }
    }

    fn delete(&mut self, key: i64) {
        match self._root.take() {
            Some(root) => self._root = AVLNode::delete(Some(root), key),
            None => return,
        }
    }

    fn print_tree(&self) {
        if &self.is_empty() == &true {
            println!("This tree is empty!");
        } else {
            self._root.as_ref().unwrap().borrow()
                .print_node(&"".to_string(), "Root".to_string(), false);
        }
    }
}


#[cfg(test)]
mod test {
    use crate::avltree;
    use crate::tree::Tree;

    #[test]
    fn test_avl() {
        let mut avl_tree = avltree::AVLTree::new();
        let input = vec![9, 5, 15, 11, 19, 10, 14, 20, 13, 12, 18, 25, 6, 16, 4, 3, 2, 1, 7, 8];
        let mut sorted_input = input.clone();
        sorted_input.sort();
        let input_slice = input.as_slice();
        let to_delete = vec![9, 20, 13, 6, 11, 15, 1,2,3,25,14];
        let to_delete_slice = to_delete.as_slice();
        let remaining: Vec<_> = input_slice
            .iter()
            .filter(|&x| !to_delete_slice.contains(x))
            .cloned()
            .collect();
        let remaining_slice = remaining.as_slice();

        
        // Initial checks
        assert_eq!(avl_tree.get_height(), 0);
        assert_eq!(avl_tree.is_empty(), true);
        assert_eq!(avl_tree.count_nodes(), 0);
        // Insert items
        for number in input_slice {
            avl_tree.insert(*number);
        }
        // Check tree properties
        assert_eq!(avl_tree.count_nodes(), input_slice.len().try_into().unwrap());
        assert_eq!(avl_tree.get_min().unwrap(), *input_slice.iter().min().unwrap());
        assert_eq!(avl_tree.get_max().unwrap(), *input_slice.iter().max().unwrap());
        assert_eq!(avl_tree.is_empty(), false);
        assert_eq!(avl_tree.get_height(), 5);
        assert_eq!(avl_tree.count_leaves(), 9);
        assert_eq!(avl_tree.in_order_traversal(), sorted_input);
        avl_tree.print_tree();

        // Check if items are in the tree
        for number in input_slice {
            assert_eq!(avl_tree.contain(*number), true);
        }
        // Delete items
        for number in to_delete_slice {
            avl_tree.delete(*number);
        }
        // Check tree properties after deletion
        assert_eq!(avl_tree.count_nodes(), remaining_slice.len().try_into().unwrap());
        assert_eq!(avl_tree.get_min().unwrap(), *remaining_slice.iter().min().unwrap());
        assert_eq!(avl_tree.get_max().unwrap(), *remaining_slice.iter().max().unwrap());
        assert_eq!(avl_tree.is_empty(), false);
        assert_eq!(avl_tree.get_height(), 4);
        assert_eq!(avl_tree.count_leaves(), 5);
        // Check if items are not in the tree anymore
        for number in to_delete_slice {
            assert_eq!(avl_tree.contain(*number), false);
        }
        // Check if other items are still in the tree
        for number in remaining_slice {
            assert_eq!(avl_tree.contain(*number), true);
        }
        // Delete all items
        for number in input_slice {
            avl_tree.delete(*number);
        }
        // Check tree properties after deletion
        assert_eq!(avl_tree.get_height(), 0);
        assert_eq!(avl_tree.is_empty(), true);
        assert_eq!(avl_tree.count_nodes(), 0);
    }
}