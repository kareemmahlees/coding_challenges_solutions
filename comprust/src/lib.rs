mod heap;
mod tree;

use crate::tree::{BTree, Node};
use anyhow::{Context, Ok, Result};
use heap::Heap;
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let file_path = std::env::args().nth(1).context("get file_path")?;

    let contents = std::fs::read_to_string(file_path).context("read file contents")?;

    let table = formulate_table(contents)?;

    let nodes: Vec<Node> = table
        .iter()
        .map(|(k, v)| Node::new(*v, Some(k.to_string()), None, None, true))
        .collect();

    let mut h = Heap::default();

    for node in nodes {
        h.insert(node);
    }

    let btree = create_btree(&mut h);

    Ok(())
}

/// Create a HashMap of char -> frequency.
fn formulate_table(contents: String) -> Result<HashMap<String, usize>> {
    let mut table = HashMap::<String, usize>::new();

    for c in contents.chars() {
        if c == ' ' || c == '\n' || c == '\r' {
            continue;
        }

        if let Some(key) = table.get_mut(&c.to_string()) {
            *key += 1
        } else {
            table.insert(c.to_string(), 1);
        };
    }

    Ok(table)
}

fn create_btree(heap: &mut Heap) -> BTree {
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
    BTree::new(btree_root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_formulate_table() {
        let table = formulate_table(String::from("aaabbc")).unwrap();

        assert_eq!(*table.get("a").unwrap(), 3);
        assert_eq!(*table.get("b").unwrap(), 2);
        assert_eq!(*table.get("c").unwrap(), 1);
    }
}
