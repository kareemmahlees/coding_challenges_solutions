mod heap;
mod table;
mod tree;

use crate::tree::{BTree, Node};
use anyhow::{Context, Ok, Result};
use clap::Parser;
use heap::Heap;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
use table::LookupTable;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file to be compressed
    file_path: String,

    /// Number of times to greet
    #[arg(short, long)]
    output: PathBuf,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let contents = std::fs::read_to_string(&args.file_path).context("read file contents")?;

    let table = create_frequency_table(&contents)?;

    let nodes: Vec<Node> = table
        .iter()
        .map(|(k, v)| Node::new(*v, Some(k.to_string()), None, None, true))
        .collect();

    let mut heap = Heap::default();

    for node in nodes {
        heap.insert(node);
    }

    let btree = BTree::new(&mut heap);

    let lookup_table = LookupTable::new(btree);

    let mut file = File::create(&args.output).context("creating output file")?;

    serde_json::to_writer(&file, &lookup_table).context("write header to file")?;

    let comp_bytes = compress(contents, lookup_table);

    file.write_all(&comp_bytes)
        .context("write compressed bytes")?;

    Ok(())
}

/// Create a HashMap of char -> frequency.
fn create_frequency_table(contents: &str) -> Result<HashMap<String, usize>> {
    let mut table = HashMap::<String, usize>::new();

    for c in contents.chars() {
        if let Some(key) = table.get_mut(&c.to_string()) {
            *key += 1
        } else {
            table.insert(c.to_string(), 1);
        };
    }

    Ok(table)
}

fn compress(content: String, lookup_table: LookupTable) -> Vec<u8> {
    let mut comp_letters = Vec::with_capacity(content.len());
    let mut comp_byte = 0b0000_0000;
    let mut bit_ptr = 7;
    for letter in content.chars() {
        let code = lookup_table.0.get(&letter.to_string()).unwrap();
        for bit in code {
            // set bit on current byte
            comp_byte |= (*bit as u8) << bit_ptr;
            // if filled comp_byte
            if bit_ptr == 0 {
                comp_letters.push(comp_byte);
                comp_byte = 0b0000_0000;
                bit_ptr = 7;
            } else {
                bit_ptr -= 1
            };
        }
    }
    // calculate the compressed_letters' padding bits
    let padding_bits = if bit_ptr == 7 { 0 } else { bit_ptr + 1 };
    if padding_bits != 0 {
        comp_letters.push(comp_byte);
    }
    comp_letters
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_formulate_table() {
        let table = create_frequency_table(&String::from("aaabbc")).unwrap();

        assert_eq!(*table.get("a").unwrap(), 3);
        assert_eq!(*table.get("b").unwrap(), 2);
        assert_eq!(*table.get("c").unwrap(), 1);
    }
}
