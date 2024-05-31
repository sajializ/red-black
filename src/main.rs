use std::io;
use tree::avlnode::AVLNode;
use tree::avltree::AVLTree;
use tree::rbtree::RBTree;
use tree::tree::Tree;
use tree::node::Node;
use tree::rbnode::RBNode;

fn run<TN: Node, T: Tree<TN>> (mut tree: T) {
    loop {
        println!("Please select an operation.");
        println!("1 - Insert a node to the tree.");
        println!("2 - Delete a node from the tree.");
        println!("3 - Count the number of leaves in the tree.");
        println!("4 - Return the height of the tree.");
        println!("5 - Print inorder traversal of the tree.");
        println!("6 - Print preorder traversal of the tree.");
        println!("7 - Print postorder traversal of the tree.");
        println!("8 - Check if the tree is empty.");
        println!("9 - Print the tree, showing it's structure.");
        println!("10 - Return the max element of the tree.");
        println!("11 - Return the min element of the tree.");
        println!("12 - Search the tree for the given key.");
        println!("13 - Count the number of nodes.");
        println!("14 - Exit");
        let operation = get_number_from_stdin();

        match operation {
            1 => {
                println!("Please enter the key to insert");
                let key = get_number_from_stdin();
                tree.insert(key);
                println!("Inserted {}", key);
            }
            2 => {
                println!("Please enter the key to delete");
                let key = get_number_from_stdin();
                tree.delete(key);
                println!("Deleted {}", key);
            }
            3 => {
                println!("Number of leaves: {}", tree.count_leaves());
            }
            4 => {
                println!("Height of the tree: {}", tree.get_height());
            }
            5 => {
                println!("In order traversal of the tree: {:?}", tree.in_order_traversal());
            }
            6 => {
                println!("Pre order traversal of the tree: {:?}", tree.pre_order_traversal());
            }
            7 => {
                println!("Post order traversal of the tree: {:?}", tree.post_order_traversal());
            }
            8 => {
                if tree.is_empty() {
                    println!("The tree is empty.");
                } else {
                    println!("The tree is not empty.");
                }
            }
            9 => {
                tree.print_tree();
            }
            10 => {
                println!("Max element of the tree: {}", tree.get_max().unwrap_or(0));
            }
            11 => {
                println!("Min element of the tree: {}", tree.get_min().unwrap_or(0));
            }
            12 => {
                println!("Please enter the key to search");
                let key = get_number_from_stdin();
                println!("Existance of the key in tree: {}", tree.contain(key));
            }
            13 => {
                println!("Number of nodes in tree: {}", tree.count_nodes());
            }
            14 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}


fn get_number_from_stdin() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i64>() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("Please enter an integer");
                continue;
            }
        };
    }
}


fn main() {
    println!("Please select the desired tree (Insert 1 or 2):");
    println!("1- RB Tree");
    println!("2- AVL Tree");
    let tree_type = get_number_from_stdin();

    match tree_type {
        1 => {
            println!("RB tree is selected!");

            let tree = RBTree::new();
            run::<RBNode, RBTree>(tree);
        },
        2 => {
            println!("AVL tree is selected!");
            let tree = AVLTree::new();

            run::<AVLNode, AVLTree>(tree);
        },
        _ => println!("Invalid tree type"),
    }
}




