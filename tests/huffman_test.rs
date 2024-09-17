use std::{collections::HashMap, fs, rc::Rc};

use huffman_encoder_decoder_in_rust::{
    codec::{Codec, Decoder, Encoder},
    huffman_node::HuffmanNode,
    huffman_tree::HuffmanTree,
    prefix_code::{PrefixCodeTable, TableMethods},
    word_frequency::get_word_frequency,
};

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

#[test]
fn header_parsed_correctly_from_encoded_file() {
    let input_filename = String::from("135-0.txt");
    let output_filename = String::from("test_encode.enc");

    let contents = fs::read_to_string(&input_filename).expect("Could not read the file");
    let freq_table = get_word_frequency(contents);
    let tree = HuffmanTree::new(freq_table);

    let expected_prefix_code_table = PrefixCodeTable::create(tree);
    match Codec::encode(
        expected_prefix_code_table.clone(),
        input_filename.clone(),
        output_filename.clone(),
    ) {
        Ok(r) => (),
        Err(e) => panic!("{}", e),
    }

    match Codec::parse_header_into_prefix_code_table(output_filename) {
        Ok(actual_prefix_code_table) => {
            for (key, expected_value) in &expected_prefix_code_table {
                if let Some(actual_value) = actual_prefix_code_table.get(key) {
                    assert_eq!(
                        actual_value, expected_value,
                        "Don't have the same values, actual: {}, expected: {}",
                        actual_value, expected_value
                    );
                }
            }

            assert_eq!(actual_prefix_code_table, expected_prefix_code_table);
        }
        Err(e) => panic!("{}", e),
    }
}
