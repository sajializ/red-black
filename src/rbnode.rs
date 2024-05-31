use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

pub type RcRefcellRBTNode = Rc<RefCell<RBNode>>;
pub type OptionNode = Option<RcRefcellRBTNode>;

#[derive(Debug)]
pub struct RBNode {
    pub key: i64,
    pub color: NodeColor,
    pub parent: OptionNode,
    pub left: OptionNode,
    pub right: OptionNode,
}

impl Node for RBNode {
    fn new(key: i64) -> OptionNode {
        Some(Rc::new(RefCell::new(RBNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
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
        if child_prefix == "Root" {
            print!("\n")
        }
        let color = if self.color == NodeColor::Black {"Black"} else {"Red"};
        
        let mut new_prefix_space_right: String = String::from(prefix_space);
        let mut new_prefix_space_left: String = String::from(prefix_space);
        if let Some(left) = self.get_left() {
          match is_right{
                    true => {
                        new_prefix_space_left.push_str("|     ")
                    },
                    false => {
                        new_prefix_space_left.push_str( "      ")
                    },
                }
            
            left.borrow()
                .print_node(&new_prefix_space_left, "L".to_string(), false);
        }
        println!("{}{} {:?} {}", prefix_space, child_prefix, self.key, color);
        if let Some(right) = self.get_right() {
            match is_right{
                    true => {
                        new_prefix_space_right.push_str( "      ")
                    },
                    false => {
                        if child_prefix == "Root" {
                            new_prefix_space_right.push_str( "      ")
                        } else {
                            new_prefix_space_right.push_str( "|     ")
                        }
                    },
                }
            
            right
                .borrow()
                .print_node(&new_prefix_space_right, "R".to_string(), true);
        }
    }
}
