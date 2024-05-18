use bitvec::{
    prelude::*,
    prelude::{bitvec, BitVec},
};

use std::collections::HashMap;

#[derive(Debug)]
pub struct BTree {
    pub root: Node,
}

impl BTree {
    pub fn new(root: Node) -> Self {
        BTree { root }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    weight: usize,
    value: Option<String>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    is_leaf: bool,
}

impl Node {
    pub fn new(
        weight: usize,
        value: Option<String>,
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
        is_leaf: bool,
    ) -> Self {
        Node {
            weight,
            value,
            left,
            right,
            is_leaf,
        }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}

pub fn create_lookup_table(node: Node, bin: Option<BitVec>) -> HashMap<String, BitVec> {
    let empty_vec = bitvec![];
    let bin = bin.unwrap_or(empty_vec);
    if node.is_leaf {
        let mut res = HashMap::new();
        res.insert(node.value.unwrap(), bin);
        return res;
    }

    let mut hash_map = HashMap::new();
    if let Some(left) = node.left {
        let mut left_bin = bin.clone();
        left_bin.push(false);
        hash_map.extend(create_lookup_table(*left, Some(left_bin)));
    }

    if let Some(right) = node.right {
        let mut right_bin = bin.clone();
        right_bin.push(true);
        hash_map.extend(create_lookup_table(*right, Some(right_bin)));
    }

    hash_map
}
