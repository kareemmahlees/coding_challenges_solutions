use std::io::{BufRead, ErrorKind};
use std::str::from_utf8;
use std::usize;

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(usize),
    BulkString(String),
    Array(Vec<DataType>),
    Null,
}

pub(crate) struct DeSerializer<'a> {
    payload: &'a [u8],
}

impl<'a> DeSerializer<'a> {
    pub fn new(payload: &'a [u8]) -> Self {
        Self { payload }
    }

    pub fn deserialize(&mut self) -> Vec<DataType> {
        let mut data_types = Vec::<DataType>::new();

        while let Ok(chunk) = self.read_next_chunk(None) {
            let data_type = self.get_data_type(chunk);
            data_types.push(data_type);
        }

        data_types
    }

    fn get_data_type(&mut self, chunk: Vec<u8>) -> DataType {
        let mark = chunk[0];
        let content = from_utf8(&chunk[1..]).unwrap().to_string();
        match mark {
            b'+' => DataType::SimpleString(content),
            b':' => DataType::Integer(content.parse::<usize>().unwrap()),
            b'-' => DataType::Error(content),
            b'$' if content == "-1" => DataType::Null,
            b'*' if content == "-1" => DataType::Null,
            b'$' => {
                let bulk_string = self
                    .read_next_chunk(Some(content.parse::<usize>().unwrap()))
                    .unwrap();
                let bulk_string = from_utf8(&bulk_string).unwrap().to_string();
                DataType::BulkString(bulk_string)
            }
            b'*' => {
                let num_of_array_members = content.parse::<usize>().unwrap();
                let mut array_data_types = Vec::with_capacity(num_of_array_members);
                for _ in 0..num_of_array_members {
                    let chunk = self.read_next_chunk(None).unwrap();
                    let data_type = self.get_data_type(chunk);
                    array_data_types.push(data_type);
                }
                DataType::Array(array_data_types)
            }
            _ => DataType::Error("Invalid data type".to_string()),
        }
    }

    fn read_next_chunk(&mut self, size: Option<usize>) -> Result<Vec<u8>, ErrorKind> {
        let mut next_chunk = if let Some(size) = size {
            Vec::with_capacity(size + 2) // + \n + \r
        } else {
            Vec::new()
        };
        let num_of_bytes = self.payload.read_until(b'\n', &mut next_chunk).unwrap();
        if num_of_bytes == 0 {
            return Err(ErrorKind::UnexpectedEof);
        }

        // remove \n
        next_chunk.pop();
        // remove \r
        next_chunk.pop();
        Ok(next_chunk)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_data() {
        let test_data = [
            ("$-1\r\n", vec![DataType::Null]),
            (
                "*1\r\n$4\r\nping\r\n",
                vec![DataType::Array(vec![DataType::BulkString(
                    "ping".to_string(),
                )])],
            ),
            (
                "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n",
                vec![DataType::Array(vec![
                    DataType::BulkString("echo".to_string()),
                    DataType::BulkString("hello world".to_string()),
                ])],
            ),
            (
                "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n",
                vec![DataType::Array(vec![
                    DataType::BulkString("get".to_string()),
                    DataType::BulkString("key".to_string()),
                ])],
            ),
            ("+OK\r\n", vec![DataType::SimpleString("OK".to_string())]),
            (
                "-Error message\r\n",
                vec![DataType::Error("Error message".to_string())],
            ),
            ("$0\r\n\r\n", vec![DataType::BulkString("".to_string())]),
            (
                "+hello world\r\n",
                vec![DataType::SimpleString("hello world".to_string())],
            ),
        ];

        for (input, expected) in test_data {
            let mut deser = DeSerializer::new(input.as_bytes());
            let data_types = deser.deserialize();
            assert_eq!(data_types, expected);
        }
    }
}
