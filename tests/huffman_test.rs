use std::{collections::HashMap, rc::Rc};

use huffman_encoder_decoder_in_rust::{huffman_node::HuffmanNode, huffman_tree::HuffmanTree};

#[test]
fn it_gives_correct_tree() {
    let freq_table = HashMap::from([
        ('C', 32),
        ('D', 42),
        ('E', 120),
        ('K', 7),
        ('L', 42),
        ('M', 24),
        ('U', 37),
        ('Z', 2),
    ]);

    let actual_tree = HuffmanTree::new(freq_table);

    let expected_tree = HuffmanTree {
        root: Some(Rc::new(HuffmanNode {
            weight: 306,
            element: None,
        })),
        left: Some(Rc::new(HuffmanTree {
            root: Some(Rc::new(HuffmanNode {
                weight: 120,
                element: Some('E'),
            })),
            left: None,
            right: None,
        })),
        right: Some(Rc::new(HuffmanTree {
            root: Some(Rc::new(HuffmanNode {
                weight: 186,
                element: None,
            })),
            left: Some(Rc::new(HuffmanTree {
                root: Some(Rc::new(HuffmanNode {
                    weight: 79,
                    element: None,
                })),
                left: Some(Rc::new(HuffmanTree {
                    root: Some(Rc::new(HuffmanNode {
                        weight: 37,
                        element: Some('U'),
                    })),
                    left: None,
                    right: None,
                })),
                right: Some(Rc::new(HuffmanTree {
                    root: Some(Rc::new(HuffmanNode {
                        weight: 42,
                        element: Some('D'),
                    })),
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Rc::new(HuffmanTree {
                root: Some(Rc::new(HuffmanNode {
                    weight: 107,
                    element: None,
                })),
                left: Some(Rc::new(HuffmanTree {
                    root: Some(Rc::new(HuffmanNode {
                        weight: 42,
                        element: Some('L'),
                    })),
                    left: None,
                    right: None,
                })),
                right: Some(Rc::new(HuffmanTree {
                    root: Some(Rc::new(HuffmanNode {
                        weight: 65,
                        element: None,
                    })),
                    left: Some(Rc::new(HuffmanTree {
                        root: Some(Rc::new(HuffmanNode {
                            weight: 32,
                            element: Some('C'),
                        })),
                        left: None,
                        right: None,
                    })),
                    right: Some(Rc::new(HuffmanTree {
                        root: Some(Rc::new(HuffmanNode {
                            weight: 33,
                            element: None,
                        })),
                        left: Some(Rc::new(HuffmanTree {
                            root: Some(Rc::new(HuffmanNode {
                                weight: 9,
                                element: None,
                            })),
                            left: Some(Rc::new(HuffmanTree {
                                root: Some(Rc::new(HuffmanNode {
                                    weight: 2,
                                    element: Some('Z'),
                                })),
                                left: None,
                                right: None,
                            })),
                            right: Some(Rc::new(HuffmanTree {
                                root: Some(Rc::new(HuffmanNode {
                                    weight: 7,
                                    element: Some('K'),
                                })),
                                left: None,
                                right: None,
                            })),
                        })),
                        right: Some(Rc::new(HuffmanTree {
                            root: Some(Rc::new(HuffmanNode {
                                weight: 24,
                                element: Some('M'),
                            })),
                            left: None,
                            right: None,
                        })),
                    })),
                })),
            })),
        })),
    };

    assert_eq!(
        actual_tree.root.unwrap().weight,
        expected_tree.root.unwrap().weight
    );

    assert_eq!(
        actual_tree
            .left
            .unwrap()
            .as_ref()
            .root
            .as_ref()
            .unwrap()
            .element
            .unwrap(),
        expected_tree
            .left
            .unwrap()
            .as_ref()
            .root
            .as_ref()
            .unwrap()
            .element
            .unwrap()
    );

    assert_eq!(
        actual_tree
            .right
            .unwrap()
            .as_ref()
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .root
            .as_ref()
            .unwrap()
            .element
            .unwrap(),
        expected_tree
            .right
            .unwrap()
            .as_ref()
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .root
            .as_ref()
            .unwrap()
            .element
            .unwrap(),
    )
}
