use std::collections::{HashMap, VecDeque};

use super::huffman_tree::HuffmanTree;

pub type PrefixCodeTable = HashMap<char, String>;

pub trait TableMethods {
    fn create(tree: HuffmanTree) -> Self;
    fn stringify(&self) -> String;
    fn to_table(s: String) -> Self;
}

impl TableMethods for PrefixCodeTable {
    fn create(tree: HuffmanTree) -> Self {
        let mut table = PrefixCodeTable::new();
        let mut prefix = String::new();

        // breadth-first search
        let mut queue = VecDeque::new();
        queue.push_back((&tree, prefix.clone()));

        while let Some((current, mut current_prefix)) = queue.pop_front() {
            match &current.left {
                Some(left) => {
                    let mut left_prefix = current_prefix.clone();
                    left_prefix.push('0');

                    if let Some(left_element) = left.root.as_ref().unwrap().element {
                        table.insert(left_element, left_prefix.clone());
                    }

                    queue.push_back((left, left_prefix));
                }
                None => {}
            }

            match &current.right {
                Some(right) => {
                    current_prefix.push('1');

                    if let Some(right_element) = right.root.as_ref().unwrap().element {
                        table.insert(right_element, current_prefix.clone());
                    }

                    queue.push_back((right, current_prefix.clone()));
                }
                None => {}
            }
        }

        return table;
    }

    fn stringify(&self) -> String {
        return self
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(" ");
    }

    fn to_table(s: String) -> Self {
        let mut prefix_table = Self::new();
        s.split(" ").into_iter().for_each(|kv| {
            let mut kvs = kv.split("=");
            if let (Some(key), Some(value)) = (kvs.next(), kvs.next()) {
                if let Some(c) = key.chars().next() {
                    prefix_table.insert(c, value.trim().to_string());
                }
            }
        });
        return prefix_table;
    }
}
