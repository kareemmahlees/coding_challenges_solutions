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

pub fn create_lookup_table(node: Node, bin_string: Option<Vec<u8>>) -> HashMap<String, Vec<u8>> {
    let mut bin_string = bin_string.unwrap_or_default();

    if node.is_leaf {
        let mut res = HashMap::new();
        res.insert(node.value.unwrap(), bin_string);
        return res;
    }

    let mut hash_map = HashMap::new();
    if let Some(left) = node.left {
        bin_string.push(0);
        hash_map.extend(create_lookup_table(*left, Some(bin_string.clone())));
    }

    if let Some(right) = node.right {
        bin_string.push(1);
        hash_map.extend(create_lookup_table(*right, Some(bin_string)));
    }

    hash_map
}
