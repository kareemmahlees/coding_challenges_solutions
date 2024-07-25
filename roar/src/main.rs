use std::fmt::write;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::StripPrefixError;

#[derive(Debug, PartialEq, Eq)]
enum DataType<'a> {
    SimpleString(&'a str),
    Error(&'a str),
    Integer(usize),
    BulkString(&'a str),
    Array(Vec<&'a str>),
    Null,
}

fn serialize_data(data: &[u8]) -> Vec<DataType> {
    let mut content = std::str::from_utf8(data).unwrap().split("\\r\\n");
    let mut data_types = Vec::<DataType>::new();
    while let Some(entry) = content.next() {
        // idk why this happens
        if entry == "" {
            continue;
        }
        let data_type = match entry {
            "$-1" | "*-1" => DataType::Null,
            other => {
                let mut other_iter = other.chars();
                let data_type = match other_iter.next().unwrap() {
                    '+' => DataType::SimpleString(&other[1..]),
                    '-' => DataType::Error(&other[1..]),
                    ':' => DataType::Integer(other[1..].parse::<usize>().unwrap()),
                    '$' => DataType::BulkString(content.next().unwrap()),
                    '*' => {
                        let num_of_items = other_iter
                            .next()
                            .unwrap()
                            .to_string()
                            .parse::<usize>()
                            .unwrap();
                        let mut result = Vec::<&str>::with_capacity(num_of_items);
                        for _ in 0..num_of_items {
                            result.push(content.next().unwrap());
                        }
                        DataType::Array(result)
                    }
                    _ => DataType::Error(&other[1..]),
                };
                data_type
            }
        };
        data_types.push(data_type);
    }
    data_types
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_data() {
        let data_types = serialize_data("$-1\\r\\n+OK\\r\\n-Some error here\\r\\n".as_bytes());
        assert_eq!(
            data_types,
            vec![
                DataType::Null,
                DataType::SimpleString("OK"),
                DataType::Error("Some error here")
            ]
        );

        let data_types =
            serialize_data("*2\\r\\n$4\\r\\necho\\r\\n$11\\r\\nhello world\\r\\n".as_bytes());
        assert_eq!(
            data_types,
            vec![
                DataType::Array(vec!["$4", "echo"]),
                DataType::BulkString("hello world")
            ]
        );
    }
}
