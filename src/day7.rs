use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::utils::*;

const TOTAL_SPACE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

struct Node {
    name: String,
    size: u32,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn read_file_tree() -> Rc<Node> {
    let root_dir = Rc::new(Node {
        name: "/".to_string(),
        size: 0,
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(None),
    });

    let mut current_dir: Weak<Node> = Rc::downgrade(&root_dir);
    let mut is_listing = false;

    for line in read_lines("./src/day7_input") {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts[0] == "$" {
            is_listing = false;

            let command_name: &str = parts[1];
            match command_name {
                "cd" => {
                    let dir_name = parts[2];
                    match dir_name {
                        "/" => {
                            current_dir = Rc::downgrade(&root_dir);
                        }
                        ".." => {
                            let curr_dir = current_dir.upgrade().unwrap();
                            let parent = curr_dir.parent.borrow();
                            if let Some(p) = &*parent {
                                current_dir = Weak::clone(p);
                            }
                        }
                        _ => {
                            let curr_dir = current_dir.upgrade().unwrap();
                            let children = curr_dir.children.borrow();
                            let child = children.iter().find(|c| c.name == dir_name);
                            if let Some(c) = child {
                                current_dir = Rc::downgrade(c);
                            }
                        }
                    }
                }
                "ls" => {
                    is_listing = true;
                }
                _ => {
                    unimplemented!("unrecognized command");
                }
            }
        } else if is_listing {
            let size: u32 = parts[0].parse().unwrap_or(0);

            let node = Node {
                name: parts[1].to_string(),
                size,
                children: RefCell::new(Vec::new()),
                parent: RefCell::new(Some(Weak::clone(&current_dir))),
            };

            let curr_dir = current_dir.upgrade().unwrap();
            let mut children = curr_dir.children.borrow_mut();
            children.push(Rc::new(node));
        } else {
            unimplemented!("invalid input");
        }
    }

    root_dir.to_owned()
}

fn get_dir_sizes(node: &Node, dir_sizes: &mut Vec<u32>) -> u32 {
    let children = node.children.borrow();
    if children.len() == 0 {
        return node.size
    }
    
    let size = children.iter().fold(node.size, |acc, c| acc + get_dir_sizes(c, dir_sizes));
    dir_sizes.push(size);
    size
}

#[allow(dead_code)]
pub fn problem1() {
    let tree = read_file_tree();
    let mut dir_sizes: Vec<u32> = Vec::new();
    get_dir_sizes(&tree, &mut dir_sizes);
    let total: u32 = dir_sizes.into_iter().filter(|size| *size < 100000).sum();
    println!("{}", total);
}

#[allow(dead_code)]
pub fn problem2() {
    let tree = read_file_tree();
    let mut dir_sizes: Vec<u32> = Vec::new();
    let used_space: u32 = get_dir_sizes(&tree, &mut dir_sizes);
    let remaining_space: u32 = TOTAL_SPACE - used_space;
    let space_required = UPDATE_SIZE - remaining_space;

    dir_sizes.sort();
    let size_to_delete: u32 = dir_sizes.into_iter().find(|size| *size > space_required).expect("no result found");
    println!("{}", size_to_delete);
}
