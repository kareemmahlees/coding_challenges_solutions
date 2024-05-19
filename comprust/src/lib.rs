mod heap;
mod table;
mod tree;

use crate::tree::{BTree, Node};
use anyhow::{Context, Error, Ok, Result};
use bitvec::{bitvec, order::Lsb0};
use clap::{Parser, Subcommand};
use heap::Heap;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};
use table::LookupTable;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compress data
    Compress {
        /// path to file to be compressed
        file_path: PathBuf,

        /// compressed output path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Decompress data.
    ///
    /// __Must be initially compressed with comprust__.
    DeCompress {
        /// path to file to be decompressed
        file_path: PathBuf,

        /// compressed output path
        #[arg(short, long)]
        output: PathBuf,
    },
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Compress { file_path, output } => {
            let contents =
                std::fs::read_to_string(file_path).context("Failed to read file contents")?;

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

            let mut file = File::create(output).context("Failed to create output file")?;

            serde_json::to_writer(&file, &lookup_table).context("Failed to write file header")?;

            let comp_bytes = compress(contents, lookup_table);

            writeln!(file).context("Failed to write newline separator")?;

            file.write_all(&comp_bytes)
                .context("Failed to write compressed content")?;
        }
        Commands::DeCompress { file_path, output } => {
            let file = File::open(file_path).context("Failed to open compressed file")?;
            let mut reader = BufReader::new(file);

            let first_line = reader
                .by_ref()
                .lines()
                .next()
                .context("Invalid compressed file format")??;
            let lookup_table = serde_json::from_str::<HashMap<String, String>>(&first_line)
                .context("Failed to deserialize lookup table")?;

            let mut buf = Vec::<u8>::new();

            reader.read_to_end(&mut buf).unwrap();

            let content = decompress(buf, lookup_table);
            let mut file = File::create(output).context("Failed to create output file")?;
            write!(file, "{content}")
                .context("Failed to write decompressed content to output file")?;
        }
    }

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

fn decompress(data: Vec<u8>, lookup_table: HashMap<String, String>) -> String {
    let mut content = String::new();
    let mut buf = String::new();
    for byte in &data[..data.len()] {
        for bit_ptr in 0..8 {
            match (byte >> (7 - bit_ptr)) & 1 == 1 {
                true => {
                    buf.push('1');
                }
                false => {
                    buf.push('0');
                }
            }

            if let Some(char) = lookup_table.get(&buf) {
                content.push_str(char);
                buf.clear()
            }
        }
    }
    content
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
