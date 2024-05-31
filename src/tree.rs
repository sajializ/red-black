use crate::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Tree<TN: Node> {
    fn new() -> Self;

    fn get_root(&self) -> &Option<Rc<RefCell<TN>>>;

    fn insert(&mut self, key: i64);

    fn delete(&mut self, key: i64);

    fn print_tree(&self);

    fn get_height(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }

    fn get_min(&self) -> Option<i64> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_min()),
        }
    }

    fn get_max(&self) -> Option<i64> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_max()),
        }
    }

    fn count_leaves(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    fn count_nodes(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_nodes(),
        }
    }

    fn contain(&self, key: i64) -> bool {
        match &self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(key),
        }
    }

    fn is_empty(&self) -> bool {
        match &self.get_root() {
            Some(_) => false,
            None => true,
        }
    }

    fn search(&self, key: i64) -> (bool, Option<Rc<RefCell<TN>>>) {
        let mut parent = None;
        let mut current = self.get_root().clone();
    
        while let Some(node) = current {
            let node_key = node.borrow().get_key();
    
            if node_key == key {
                return (true, Some(node.clone()));
            } else if node_key > key {
                parent = Some(node.clone());
                current = node.borrow().get_left().clone();
            } else {
                parent = Some(node.clone());
                current = node.borrow().get_right().clone();
            }
        }
    
        (false, parent)
    }

    fn in_order_traversal(&self) -> Vec<i64> {
        let mut result = Vec::new();
        if self.get_root().is_none() {
            return result;
        }
    
        let mut stack = Vec::new();
        let mut node = self.get_root().clone();
    
        while !stack.is_empty() || node.is_some() {
            while let Some(current) = node {
                stack.push(current.clone());
                node = current.borrow().get_left().clone();
            }
    
            if let Some(current) = stack.pop() {
                result.push(current.borrow().get_key());
                node = current.borrow().get_right().clone();
            }
        }
    
        result
    }

    fn pre_order_traversal(&self) -> Vec<i64> {
        let result = Vec::new();
        if self.get_root().is_none() {
            return result;
        }
    
        let mut result = Vec::new();
        let root = self.get_root().clone();
        if let Some(node) = root {
            let mut stack = vec![node.clone()];
    
            while let Some(curr) = stack.pop() {
                result.push(curr.borrow().get_key());
                
                if let Some(right) = curr.borrow().get_right() {
                    stack.push(right.clone());
                }
    
                if let Some(left) = curr.borrow().get_left() {
                    stack.push(left.clone());
                }
            }
        }
        result
    }

    fn post_order_traversal(&self) -> Vec<i64> {
        let result = Vec::new();
        if self.get_root().is_none() {
            return result;
        }
    
        let mut result = Vec::new();
        let root = self.get_root().clone();

        if let Some(node) = root {
            let mut stack = vec![node.clone()];
    
            while let Some(curr) = stack.pop() {
                result.insert(0, curr.borrow().get_key());
                
                if let Some(left) = curr.borrow().get_left() {
                    stack.push(left.clone());
                }
    
                if let Some(right) = curr.borrow().get_right() {
                    stack.push(right.clone());
                }
            }
        }
        result
    }
}
