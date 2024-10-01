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
        match (&self.root, &other.root) {
            (Some(s), Some(o)) => s.cmp(o),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            _ => Ordering::Equal,
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

    fn merge(first: Rc<HuffmanTree>, second: Rc<HuffmanTree>) -> Self {
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

    pub fn depth(&self) -> usize {
        if let (Some(left), Some(right)) = (self.left.as_ref(), self.right.as_ref()) {
            return 1 + usize::max(left.depth(), right.depth());
        }
        return 0;
    }
}
