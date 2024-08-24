use std::cmp::Ordering;

#[derive(Debug)]
pub struct HuffmanNode {
    pub weight: u32,
    pub element: Option<char>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Eq for HuffmanNode {}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        other.weight.eq(&self.weight)
    }
}
