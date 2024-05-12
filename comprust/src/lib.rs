use std::collections::HashMap;

use anyhow::{Context, Ok, Result};

pub fn run() -> Result<()> {
    let file_path = std::env::args().nth(1).context("get file_path")?;

    let contents = std::fs::read_to_string(file_path).context("read file contents")?;

    let table = formulate_table(contents)?;

    dbg!(table);

    Ok(())
}

/// Create a HashMap of char -> occurrence.
fn formulate_table(contents: String) -> Result<HashMap<String, u64>> {
    let mut table = HashMap::<String, u64>::new();

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
