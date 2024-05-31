use crate::node::Node;
use std::cell::RefCell;
use std::cmp::max;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

type RcRefcellAVLNode = Rc<RefCell<AVLNode>>;
pub type OptionNode = Option<RcRefcellAVLNode>;

#[derive(Debug)]
pub struct AVLNode {
    pub key: i64,
    left: OptionNode,
    right: OptionNode,
    height: u32,
}

impl Node for AVLNode {
    fn new(key: i64) -> OptionNode {
        Some(Rc::new(RefCell::new(AVLNode {
            key,
            left: None,
            right: None,
            height: 1,
        })))
    }

    fn get_left(&self) -> &OptionNode {
        &self.left
    }

    fn get_right(&self) -> &OptionNode {
        &self.right
    }

    fn get_key(&self) -> i64 {
        self.key
    }

    fn print_node(&self, prefix_space: &String, child_prefix: String, is_right: bool) {
        let mut new_prefix_space_right: String;
        let mut new_prefix_space_left: String;
        if child_prefix == "Root" {
            print!("\n");
            new_prefix_space_right = String::from("    ");
            new_prefix_space_left = String::from("    ");
        } else {
            new_prefix_space_right = String::from(prefix_space);
            new_prefix_space_left = String::from(prefix_space);
        }

        if let Some(left) = self.get_left() {
            match is_right {
                true => new_prefix_space_left.push_str("|      "),
                false => new_prefix_space_left.push_str("       "),
            }

            left.borrow()
                .print_node(&new_prefix_space_left, "L".to_string(), false);
        }
        if self.get_left().is_none() && self.get_right().is_none() {
            println!("{}{} {:?}", prefix_space, child_prefix, self.key);
        } else {
            println!("{}{} {:?}----|", prefix_space, child_prefix, self.key);
        }
        if let Some(right) = self.get_right() {
            match is_right {
                true => new_prefix_space_right.push_str("       "),
                false => {
                    if child_prefix == "Root" {
                        new_prefix_space_right.push_str("       ")
                    } else {
                        new_prefix_space_right.push_str("|      ")
                    }
                }
            }

            right
                .borrow()
                .print_node(&new_prefix_space_right, "R".to_string(), true);
        }
    }
}

impl AVLNode {
    fn _get_height(node: Option<RcRefcellAVLNode>) -> u32 {
        node.map_or(0, |this_node| this_node.borrow().height)
    }

    fn _get_left_height(node: &RcRefcellAVLNode) -> u32 {
        Self::_get_height(node.borrow().left.clone())
    }

    fn _get_right_height(node: &RcRefcellAVLNode) -> u32 {
        Self::_get_height(node.borrow().right.clone())
    }

    fn _get_balance_factor(node: &RcRefcellAVLNode) -> i64 {
        Self::_get_left_height(node) as i64 - Self::_get_right_height(node) as i64
    }

    fn _left_rotate(root: RcRefcellAVLNode) -> RcRefcellAVLNode {
        let new_root = root.borrow().right.clone().unwrap();
        root.borrow_mut().right = new_root.borrow().left.clone().take();
        root.borrow_mut().height = 1 + Self::_max_height(&root);

        new_root.borrow_mut().left = Some(root);
        new_root.borrow_mut().height = 1 + Self::_max_height(&new_root);

        new_root
    }

    fn _max_height(node: &RcRefcellAVLNode) -> u32 {
        max(
            Self::_get_left_height(&node),
            Self::_get_right_height(&node),
        )
    }

    fn _right_rotate(root: RcRefcellAVLNode) -> RcRefcellAVLNode {
        let new_root = root.borrow().left.clone().unwrap();
        root.borrow_mut().left = new_root.borrow().right.clone().take();

        root.borrow_mut().height = 1 + Self::_max_height(&root);

        new_root.borrow_mut().right = Some(root);
        new_root.borrow_mut().height = 1 + Self::_max_height(&new_root);

        new_root
    }

    fn _left_right_rotate(root: RcRefcellAVLNode) -> RcRefcellAVLNode {
        let left = root.borrow().left.clone().take().unwrap();
        root.borrow_mut().left = Some(Self::_left_rotate(left));
        Self::_right_rotate(root)
    }

    fn _right_left_rotate(root: RcRefcellAVLNode) -> RcRefcellAVLNode {
        let right = root.borrow().right.clone().take().unwrap();
        root.borrow_mut().right = Some(Self::_right_rotate(right));
        Self::_left_rotate(root)
    }

    pub fn insert(node: OptionNode, key: i64) -> OptionNode {
        let return_node = match node {
            None => AVLNode::new(key).unwrap(),
            Some(this_node) => {
                let node_key = this_node.borrow().key;
                match key.cmp(&node_key) {
                    Ordering::Less => {
                        let left: OptionNode = this_node.borrow().left.clone();
                        this_node.borrow_mut().left = Self::insert(left, key);
                    }
                    Ordering::Greater => {
                        let right: OptionNode = this_node.borrow().right.clone();
                        this_node.borrow_mut().right = Self::insert(right, key);
                    }
                    Ordering::Equal => {}
                }
                this_node
            }
        };
        let balance_factor = Self::_get_balance_factor(&return_node);
        let new_return_node: RcRefcellAVLNode = match balance_factor {
            2 => {
                let new_key = return_node.borrow().left.clone().unwrap().borrow().key;
                match key.cmp(&new_key) {
                    Ordering::Less => Self::_right_rotate(return_node),
                    Ordering::Greater => Self::_left_right_rotate(return_node),
                    _ => return_node,
                }
            }
            -2 => {
                let new_key = return_node.borrow().right.clone().unwrap().borrow().key;
                match key.cmp(&new_key) {
                    Ordering::Less => Self::_right_left_rotate(return_node),
                    Ordering::Greater => Self::_left_rotate(return_node),
                    _ => return_node,
                }
            }
            _ => return_node,
        };
        new_return_node.borrow_mut().height = max(
            Self::_get_left_height(&new_return_node),
            Self::_get_right_height(&new_return_node),
        ) + 1;
        Some(new_return_node)
    }

    pub fn delete(node: OptionNode, key: i64) -> OptionNode {
        if node.is_none() {
            return node;
        }
        let this_node = node.unwrap();
        let return_node: OptionNode;
        let node_key = this_node.borrow().key;
        return_node = match node_key.cmp(&key) {
            Ordering::Greater => {
                let left: OptionNode = this_node.borrow().left.clone();
                match left {
                    None => return Some(this_node),
                    Some(_) => {
                        let left: OptionNode = this_node.borrow().left.clone().take();
                        this_node.borrow_mut().left = Self::delete(left, key);
                    }
                }
                Some(this_node)
            }
            Ordering::Less => {
                let right: OptionNode = this_node.borrow().right.clone();
                if right.is_none() {
                    return Some(this_node);
                }

                let right: OptionNode = this_node.borrow().right.clone().take();
                this_node.borrow_mut().right = Self::delete(right, key);
    
                Some(this_node)
            }
            Ordering::Equal => {
                let left: OptionNode = this_node.borrow().left.clone();
                let right: OptionNode = this_node.borrow().right.clone();
                if right.is_none() && left.is_none() {
                    return None;
                } else if right.is_none() {
                    let inner_left = left.clone().unwrap();
                    return Some(inner_left)
                } else if left.is_none() {
                    let inner_right = right.clone().unwrap();
                    return Some(inner_right)
                } else {
                    let inner_right = right.clone().unwrap();
                    let min_value = inner_right.borrow().get_min();
                    this_node.borrow_mut().key = min_value;
                    let right = this_node.borrow().right.clone().take();
                    this_node.borrow_mut().right = Self::delete(right, min_value);
                    return Some(this_node)
                }
            }
        };

        if return_node.is_none() {
            return return_node;
        }
        let this_node = return_node.unwrap();
        let balance_factor = Self::_get_balance_factor(&this_node);
        let return_node = match balance_factor {
            2 => {
                let left_child = this_node.borrow().left.clone().unwrap();
                let (left_height, right_height) = (
                    Self::_get_left_height(&left_child),
                    Self::_get_right_height(&left_child),
                );

                if left_height >= right_height {
                    Self::_right_rotate(this_node)
                } else {
                    Self::_left_right_rotate(this_node)
                }
            }
            -2 => {
                let right_child = &this_node.borrow().right.clone().unwrap();
                let (left_height, right_height) = (
                    Self::_get_left_height(right_child),
                    Self::_get_right_height(right_child),
                );
                if right_height >= left_height {
                    Self::_left_rotate(this_node)
                } else {
                    Self::_right_left_rotate(this_node)
                }
            }
            _ => this_node,
        };

        return_node.borrow_mut().height = max(
            Self::_get_left_height(&return_node),
            Self::_get_right_height(&return_node),
        ) + 1;
        Some(return_node)
    }
}
