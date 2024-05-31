use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::node::Node;
use crate::rbnode::{NodeColor, OptionNode, RBNode, RcRefcellRBTNode};
use crate::tree::Tree;

pub struct RBTree {
    _root: OptionNode,
}

impl Tree<RBNode> for RBTree {
    fn new() -> Self {
        RBTree { _root: None }
    }

    fn get_root(&self) -> &OptionNode {
        &self._root
    }

    fn insert(&mut self, key: i64) {
        if self.is_empty() {
            let new_node = RBNode::new(key);
            new_node.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
            self._root = new_node;
            return;
        }
        let (exists, parent_option) = self.search(key);

        if exists {
            return;
        }

        let new_child_node = RBNode::new(key).as_ref().unwrap().clone();
        let new_child_ref_clone = new_child_node.clone();
        let new_child = Some(new_child_node);

        let parent_ref = Rc::clone(&parent_option.as_ref().unwrap());
        new_child.as_ref().unwrap().borrow_mut().parent = Some(parent_ref);

        let should_be_left_child = key < parent_option.as_ref().unwrap().borrow().key;
        if should_be_left_child {
            parent_option.as_ref().unwrap().borrow_mut().left = new_child;
        } else {
            parent_option.as_ref().unwrap().borrow_mut().right = new_child;
        }

        self._insert_repair(new_child_ref_clone);
    }

    fn delete(&mut self, key: i64) {
        let (exists, result) = self.search(key);
        if exists {
            let mut result_node_ref: &RcRefcellRBTNode = result.as_ref().unwrap();
            let _ = self._delete_private(&mut result_node_ref);
        }
    }

    fn print_tree(&self) {
        if let Some(root) = &self.get_root() {
            root.borrow()
                .print_node(&"".to_string(), "Root".to_string(), false);
        } else {
            println!("This tree is empty!");
        }
    }
}

impl RBTree {
    fn _insert_repair(&mut self, new_child: RcRefcellRBTNode) {
        let mut child: RcRefcellRBTNode = new_child;
        loop {
            if child.borrow().parent.is_none() {
                Self::_change_color(&mut self._root.as_ref().unwrap(), NodeColor::Black);
                return;
            }
            if !Self::_is_parent_red(&child) {
                return;
            }
            let mut parent = child.borrow().parent.clone().unwrap();
            let grandparent = parent.borrow().parent.clone().unwrap();
            let uncle: Rc<RefCell<RBNode>>;

            let parent_left_side: bool = RBTree::_is_left_child(&parent);
            if parent_left_side {
                if grandparent.borrow().right.is_none()
                    || grandparent.borrow().right.as_ref().unwrap().borrow().color
                        == NodeColor::Black
                {
                    if !RBTree::_is_left_child(&child) {
                        self._left_rotate(&parent);
                        parent = grandparent.borrow().left.as_ref().unwrap().clone();
                    }
                    RBTree::_change_color(&mut &parent, NodeColor::Black);
                    RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                    self._right_rotate(&grandparent);
                    return;
                }
                uncle = grandparent.borrow().right.as_ref().unwrap().clone();
                RBTree::_change_color(&mut &parent, NodeColor::Black);
                RBTree::_change_color(&mut &uncle, NodeColor::Black);
                RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                child = grandparent;
                continue;
            }
            if grandparent.borrow().left.is_none()
                || grandparent.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black
            {
                if RBTree::_is_left_child(&child) {
                    self._right_rotate(&parent);
                    parent = grandparent.borrow().right.as_ref().unwrap().clone();
                }
                RBTree::_change_color(&mut &parent, NodeColor::Black);
                RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                self._left_rotate(&grandparent);
                return;
            }
            uncle = grandparent.borrow().left.as_ref().unwrap().clone();
            RBTree::_change_color(&mut &parent, NodeColor::Black);
            RBTree::_change_color(&mut &uncle, NodeColor::Black);
            RBTree::_change_color(&mut &grandparent, NodeColor::Red);
            child = grandparent;
        }
    }

    fn _is_left_child(node: &RcRefcellRBTNode) -> bool {
        node.borrow().parent.as_ref().map_or(false, |parent| {
            parent
                .borrow()
                .left
                .as_ref()
                .map_or(false, |left| left.borrow().key == node.borrow().key)
        })
    }

    fn _is_parent_red(node: &RcRefcellRBTNode) -> bool {
        node.borrow()
            .parent
            .as_ref()
            .map_or(false, |parent| parent.borrow().color == NodeColor::Red)
    }

    fn _left_rotate(&mut self, rotation_node: &RcRefcellRBTNode) {
        {
            let parent: &OptionNode = &rotation_node.borrow().parent;
            let right: &OptionNode = &rotation_node.borrow().right;
            if rotation_node.borrow().parent.is_none() {
                self._root = right.clone();
            }
            if let Some(node) = parent {
                if Self::_is_left_child(rotation_node) {
                    node.borrow_mut().left = right.clone();
                } else {
                    node.borrow_mut().right = right.clone();
                }
            }
            right.as_ref().unwrap().borrow_mut().parent = parent.clone();
        }
        let right_node: Rc<RefCell<RBNode>> =
            rotation_node.borrow().right.as_ref().unwrap().clone();
        rotation_node.borrow_mut().parent = Some(Rc::clone(&right_node));

        if right_node.borrow().left.is_some() {
            rotation_node.borrow_mut().right =
                Some(right_node.borrow().left.as_ref().unwrap().clone());
            right_node
                .borrow_mut()
                .left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .parent = Some(Rc::clone(rotation_node));
        } else {
            rotation_node.borrow_mut().right = None;
        }
        right_node.borrow_mut().left = Some(rotation_node.clone());
    }

    fn _right_rotate(&mut self, rotation_node: &RcRefcellRBTNode) {
        {
            let parent: &OptionNode = &rotation_node.borrow().parent;
            let left: &OptionNode = &rotation_node.borrow().left;
            if rotation_node.borrow().parent.is_none() {
                self._root = left.clone();
            }
            if let Some(node) = parent {
                if Self::_is_left_child(rotation_node) {
                    node.borrow_mut().left = left.clone();
                } else {
                    node.borrow_mut().right = left.clone();
                }
            }
            left.as_ref().unwrap().borrow_mut().parent = parent.clone();
        }
        let left_node: Rc<RefCell<RBNode>> = rotation_node.borrow().left.as_ref().unwrap().clone();
        rotation_node.borrow_mut().parent = Some(Rc::clone(&left_node));
        if left_node.borrow().right.is_some() {
            rotation_node.borrow_mut().left =
                Some(left_node.borrow().right.as_ref().unwrap().clone());
            left_node
                .borrow_mut()
                .right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .parent = Some(Rc::clone(rotation_node));
        } else {
            rotation_node.borrow_mut().left = None;
        }
        left_node.borrow_mut().right = Some(rotation_node.clone());
    }

    fn _recur_right_child(node: OptionNode) -> OptionNode {
        if let Some(inner_node) = &node {
            if inner_node.borrow().right.is_some() {
                return Self::_recur_right_child(inner_node.borrow().right.clone());
            }
        }
        node
    }

    fn _find_replacement_node(node: &RcRefcellRBTNode) -> OptionNode {
        return if node.borrow().left.is_some() {
            Self::_recur_right_child(node.borrow().left.clone())
        } else if node.borrow().right.is_some() {
            node.borrow().right.clone()
        } else {
            None
        };
    }

    fn _delete_private(&mut self, node: &mut &RcRefcellRBTNode) -> Result<(), String> {
        let replacement = Self::_find_replacement_node(node);
        let parent = node.borrow().parent.clone();
        let double_black = Self::_return_color(node) == NodeColor::Black
            && (replacement.is_none()
                || Self::_return_color(replacement.as_ref().unwrap()) == NodeColor::Black);

        if replacement.is_none() {
            if node.borrow().parent.is_none() {
                self._root = None;
                return Ok(());
            }
            if double_black {
                self._delete_repair(node);
            }
            if Self::_is_left_child(node) {
                parent.as_ref().unwrap().borrow_mut().left = None;
            } else {
                parent.as_ref().unwrap().borrow_mut().right = None;
            }

            return Ok(());
        }
        if node.borrow().left.is_none() || node.borrow().right.is_none() {
            if node.borrow().parent.is_none() {
                let temp = replacement.as_ref().unwrap().borrow().key;
                let mut root: RefMut<RBNode> = self._root.as_ref().unwrap().borrow_mut();
                root.key = temp;
                root.left = None;
                root.right = None;
                return Ok(());
            }
            if !Self::_is_left_child(node) {
                parent.as_ref().unwrap().borrow_mut().right = replacement.clone();
            } else {
                parent.as_ref().unwrap().borrow_mut().left = replacement.clone();
            }
            replacement.as_ref().unwrap().borrow_mut().parent = parent.clone();
            if !double_black {
                Self::_change_color(&mut replacement.as_ref().unwrap(), NodeColor::Black);
            } else {
                self._delete_repair(&replacement.unwrap());
            }

            return Ok(());
        }
        node.borrow_mut().key = replacement.as_ref().unwrap().borrow().key;
        self._delete_private(&mut replacement.as_ref().unwrap())
            .unwrap();

        Ok(())
    }

    fn _delete_repair(&mut self, node: &RcRefcellRBTNode) {
        if node.borrow().parent.is_none() {
            return;
        }
        let parent = Some(Rc::clone(node.borrow().parent.as_ref().unwrap()));
        let sibling = Self::_return_node_same_level(node);

        if sibling.is_none() {
            self._delete_repair(&parent.unwrap());
            return;
        }
        if Self::_return_color(sibling.as_ref().unwrap()) == NodeColor::Black {
            if !Self::_has_red_child(sibling.as_ref().unwrap()) {
                Self::_change_color(&mut sibling.as_ref().unwrap(), NodeColor::Red);
                if Self::_return_color(parent.as_ref().unwrap()) == NodeColor::Red {
                    Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                    return;
                }
                self._delete_repair(&parent.unwrap());

                return;
            }
            if !Self::_is_left_child(node) {
                if sibling.as_ref().unwrap().borrow().left.is_some()
                    && Self::_return_color(
                        sibling.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                    ) == NodeColor::Red
                {
                    Self::_change_color(
                        &mut sibling.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                        NodeColor::Black,
                    );
                    let parent_color = Self::_return_color(parent.as_ref().unwrap());
                    Self::_change_color(&mut sibling.as_ref().unwrap(), parent_color);
                    self._right_rotate(parent.as_ref().unwrap());
                    Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                    return;
                }
                let parent_color = Self::_return_color(parent.as_ref().unwrap());
                Self::_change_color(
                    &mut sibling.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                    parent_color,
                );
                self._left_rotate(sibling.as_ref().unwrap());
                self._right_rotate(parent.as_ref().unwrap());
                Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);

                return;
            }
            if sibling.as_ref().unwrap().borrow().left.is_some()
                && Self::_return_color(sibling.as_ref().unwrap().borrow().left.as_ref().unwrap())
                    == NodeColor::Red
            {
                let parent_color: NodeColor = Self::_return_color(parent.as_ref().unwrap());
                Self::_change_color(
                    &mut sibling.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                    parent_color,
                );
                self._right_rotate(sibling.as_ref().unwrap());
                self._left_rotate(parent.as_ref().unwrap());
                Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                return;
            }
            Self::_change_color(
                &mut sibling.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                NodeColor::Black,
            );
            let parent_color: NodeColor = Self::_return_color(parent.as_ref().unwrap());
            Self::_change_color(&mut sibling.as_ref().unwrap(), parent_color);
            self._left_rotate(parent.as_ref().unwrap());
            Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);

            return;
        }
        Self::_change_color(&mut sibling.as_ref().unwrap(), NodeColor::Black);
        Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Red);
        if Self::_is_left_child(node) {
            self._left_rotate(parent.as_ref().unwrap());
        } else {
            self._right_rotate(parent.as_ref().unwrap());
        }
        self._delete_repair(node);
    }

    fn _return_color(node: &RcRefcellRBTNode) -> NodeColor {
        node.borrow().color.clone()
    }

    fn _change_color(node: &mut &RcRefcellRBTNode, color: NodeColor) {
        node.borrow_mut().color = color;
    }

    fn _return_node_same_level(node: &RcRefcellRBTNode) -> OptionNode {
        if let Some(parent) = &node.borrow().parent {
            let parent_node = &parent.borrow();
            return if Self::_is_left_child(node) {
                parent_node.right.clone()
            } else {
                parent_node.left.clone()
            };
        }
        None
    }

    fn _has_red_child(node: &RcRefcellRBTNode) -> bool {
        let left_red = node
            .borrow()
            .left
            .as_ref()
            .map_or(false, |left| Self::_return_color(left) == NodeColor::Red);
        let right_red = node
            .borrow()
            .right
            .as_ref()
            .map_or(false, |right| Self::_return_color(right) == NodeColor::Red);

        left_red || right_red
    }
}



#[cfg(test)]
mod test {
    use crate::rbtree;
    use crate::tree::Tree;

    #[test]
    fn test_rbtree() {
        let mut rb_tree: rbtree::RBTree = rbtree::RBTree::new();
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
        assert_eq!(rb_tree.get_height(), 0);
        assert_eq!(rb_tree.is_empty(), true);
        assert_eq!(rb_tree.count_nodes(), 0);
        // Insert items
        for number in input_slice {
            rb_tree.insert(*number);
        }
        // Check tree properties
        assert_eq!(rb_tree.count_nodes(), input_slice.len().try_into().unwrap());
        assert_eq!(rb_tree.get_min().unwrap(), *input_slice.iter().min().unwrap());
        assert_eq!(rb_tree.get_max().unwrap(), *input_slice.iter().max().unwrap());
        assert_eq!(rb_tree.is_empty(), false);
        assert_eq!(rb_tree.get_height(), 5);
        assert_eq!(rb_tree.count_leaves(), 9);
        assert_eq!(rb_tree.in_order_traversal(), sorted_input);
        rb_tree.print_tree();

        // Check if items are in the tree
        for number in input_slice {
            assert_eq!(rb_tree.contain(*number), true);
        }
        // Delete items
        for number in to_delete_slice {
            rb_tree.delete(*number);
        }
        // Check tree properties after deletion
        assert_eq!(rb_tree.count_nodes(), remaining_slice.len().try_into().unwrap());
        assert_eq!(rb_tree.get_min().unwrap(), *remaining_slice.iter().min().unwrap());
        assert_eq!(rb_tree.get_max().unwrap(), *remaining_slice.iter().max().unwrap());
        assert_eq!(rb_tree.is_empty(), false);
        assert_eq!(rb_tree.get_height(), 4);
        assert_eq!(rb_tree.count_leaves(), 4);
        
        // Check if items are not in the tree anymore
        for number in to_delete_slice {
            assert_eq!(rb_tree.contain(*number), false);
        }
        // Check if other items are still in the tree
        for number in remaining_slice {
            assert_eq!(rb_tree.contain(*number), true);
        }
        // Delete all items
        for number in input_slice {
            rb_tree.delete(*number);
        }
        // Check tree properties after deletion
        assert_eq!(rb_tree.get_height(), 0);
        assert_eq!(rb_tree.is_empty(), true);
        assert_eq!(rb_tree.count_nodes(), 0);
    }
}
