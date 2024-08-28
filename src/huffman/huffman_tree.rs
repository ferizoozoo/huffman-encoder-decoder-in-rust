use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use super::huffman_node::HuffmanNode;

#[derive(PartialEq, PartialOrd, Eq, Debug)]
pub struct HuffmanTree {
    pub root: Option<Rc<HuffmanNode>>,
    pub left: Option<Rc<HuffmanTree>>,
    pub right: Option<Rc<HuffmanTree>>,
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Some(s_root), Some(o_root)) = (&self.root, &other.root) {
            s_root.cmp(&o_root)
        } else {
            Ordering::Equal
        }
    }
}

impl HuffmanTree {
    pub fn new(freq_table: HashMap<char, u32>) -> Self {
        let mut queue = BinaryHeap::new();

        for (k, v) in freq_table {
            queue.push(Self {
                root: Some(Rc::new(HuffmanNode {
                    weight: v,
                    element: Some(k),
                })),
                left: None,
                right: None,
            });
        }

        while queue.len() > 1 {
            let t1 = queue.pop().unwrap();
            let t2 = queue.pop().unwrap();

            let new_tree = HuffmanTree::merge(Rc::new(t1), Rc::new(t2));

            queue.push(new_tree);
        }

        return queue.pop().unwrap();
    }

    pub fn merge(first: Rc<HuffmanTree>, second: Rc<HuffmanTree>) -> Self {
        let w1 = first.root.as_ref().unwrap().weight;
        let w2 = second.root.as_ref().unwrap().weight;

        let new_root = HuffmanNode {
            weight: w1 + w2,
            element: None,
        };

        HuffmanTree {
            root: Some(Rc::new(new_root)),
            left: Some(first),
            right: Some(second),
        }
    }
}
