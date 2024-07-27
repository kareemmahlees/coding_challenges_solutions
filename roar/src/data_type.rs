#[derive(Debug, PartialEq, Eq)]
/// A representation of the `RESP` protocol data types.
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<DataType>),
    Null,
}

impl DataType {
    /// Convert the data type into a format understandable by the client
    /// according to the `RESP` protocol.
    pub fn serialize(&self) -> String {
        let serialized_data = match self {
            DataType::SimpleString(content) => format!("+{content}\r\n"),
            DataType::BulkString(content) => format!("${}\r\n{}\r\n", content.len(), content),
            DataType::Null => format!("$-1\r\n"),
            DataType::Integer(content) => format!(":{}\r\n", content),
            DataType::Error(content) => format!("-{}\r\n", content),
            _ => todo!(),
        };
        serialized_data
    }

    /// Get the value stored inside the data type.
    ///
    /// **Not implemented for all types**
    pub fn inner(&self) -> &String {
        let inner = match self {
            DataType::SimpleString(content) => content,
            DataType::BulkString(content) => content,
            DataType::Error(content) => content,
            _ => todo!(),
        };
        inner
    }
}
