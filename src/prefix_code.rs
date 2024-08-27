use std::collections::{HashMap, VecDeque};

use crate::huffman_tree::HuffmanTree;

pub type PrefixCodeTable = HashMap<char, u64>;

pub trait TableMethods {
    fn create(tree: HuffmanTree) -> Self;
}

impl TableMethods for PrefixCodeTable {
    fn create(tree: HuffmanTree) -> Self {
        let mut table = PrefixCodeTable::new();
        let mut prefix: u64 = 0;

        // breadth-first search
        let mut queue = VecDeque::new();
        queue.push_back(&tree);

        while queue.len() > 0 {
            match queue.pop_front() {
                Some(current) => {
                    match &current.left {
                        Some(left) => {
                            if let Some(left_element) = left.root.as_ref().unwrap().element {
                                table.insert(left_element, prefix);
                            }
                            queue.push_back(left);
                        }
                        None => {}
                    }

                    match &current.right {
                        Some(right) => {
                            if let Some(right_element) = right.root.as_ref().unwrap().element {
                                prefix |= 1;
                                table.insert(right_element, prefix);
                            }
                            queue.push_back(right);
                        }
                        None => {}
                    }
                }
                None => {}
            }
            prefix <<= 1;
        }

        return table;
    }
}
