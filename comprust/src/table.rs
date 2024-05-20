use crate::tree::{BTree, Node};
use bitvec::prelude::{bitvec, BitVec, Lsb0};
use serde::{ser::SerializeMap, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
/// Table storing chars and their `bit` codes.
pub struct LookupTable(pub(crate) HashMap<String, BitVec>);

impl LookupTable {
    pub fn new(btree: BTree) -> Self {
        let table = create_lookup_table(btree.root, None);
        LookupTable(table)
    }
}

impl Serialize for LookupTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut simplified_hashmap = HashMap::<String, String>::new();

        for (k, v) in self.0.iter() {
            let mut contents = String::new();
            for bit in v {
                if bit == true {
                    contents.push('1');
                } else {
                    contents.push('0');
                }
            }
            // we intentionally reverse the hashmap here to make it
            // easier to interpret codes.
            simplified_hashmap.insert(contents, k.to_owned());
        }
        let mut map = serializer.serialize_map(Some(simplified_hashmap.len()))?;
        for (k, v) in simplified_hashmap {
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

/// Traverses the binary tree recursively and formulates `HashMap`
/// which will be used in creating `LookupTable`.
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
