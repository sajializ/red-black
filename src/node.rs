use std::cell::RefCell;
use std::cmp::{max, Ordering};
use std::rc::Rc;

pub trait Node {
    fn new(key: i64) -> Option<Rc<RefCell<Self>>>;

    fn print_node(&self, prefix_space: &String, child_prefix: String, is_right: bool);

    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_key(&self) -> i64;

    fn get_height(&self) -> u32 {
        match (self.get_left(), self.get_right()) {
            (Some(left), Some(right)) => max(left.borrow().get_height(), right.borrow().get_height()) + 1,
            (Some(left), _) => left.borrow().get_height() + 1,
            (_, Some(right)) => right.borrow().get_height() + 1,
            (_, _) => 1,
        }
    }
    

    fn get_min(&self) -> i64 {
        self.get_left().as_ref().map_or_else(|| self.get_key(), |left| left.borrow().get_min())
    }
    
    fn get_max(&self) -> i64 {
        self.get_right().as_ref().map_or_else(|| self.get_key(), |right| right.borrow().get_max())
    }
    

    fn count_leaves(&self) -> u32 {
        if let (Some(left), Some(right)) = (self.get_left(), self.get_right()) {
            return left.borrow().count_leaves() + right.borrow().count_leaves();
        }
    
        if let Some(left) = self.get_left() {
            return left.borrow().count_leaves();
        }
    
        if let Some(right) = self.get_right() {
            return right.borrow().count_leaves();
        }
    
        1
    }
    

    fn count_nodes(&self) -> u32 {
        match (self.get_left(), self.get_right()) {
            (Some(left), Some(right)) => left.borrow().count_nodes() + right.borrow().count_nodes() + 1,
            (Some(left), _) => left.borrow().count_nodes() + 1,
            (_, Some(right)) => right.borrow().count_nodes() + 1,
            (_, _) => 1,
        }
    }
    

    fn contains(&self, key: i64) -> bool {
        match self.get_key().cmp(&key) {
            Ordering::Less => match self.get_right() {
                None => false,
                Some(right) => right.borrow().contains(key),
            },
            Ordering::Greater => match self.get_left() {
                None => false,
                Some(left) => left.borrow().contains(key),
            },
            Ordering::Equal => true,
        }
    }
}
