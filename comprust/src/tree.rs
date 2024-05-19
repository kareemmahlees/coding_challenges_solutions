use bitvec::{
    prelude::*,
    prelude::{bitvec, BitVec},
};
use std::collections::HashMap;

use crate::heap::Heap;

#[derive(Debug)]
pub struct BTree {
    pub root: Node,
}

impl BTree {
    pub fn new(heap: &mut Heap) -> Self {
        while heap.size() > 1 {
            let tmp1 = heap.delete();
            let tmp2 = heap.delete();
            let tmp3 = Node::new(
                tmp1.weight() + tmp2.weight(),
                None,
                Some(Box::new(tmp1)),
                Some(Box::new(tmp2)),
                false,
            );
            heap.insert(tmp3);
        }
        let btree_root = heap.delete();
        BTree { root: btree_root }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    weight: usize,
    pub(crate) value: Option<String>,
    pub(crate) left: Option<Box<Self>>,
    pub(crate) right: Option<Box<Self>>,
    pub(crate) is_leaf: bool,
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
