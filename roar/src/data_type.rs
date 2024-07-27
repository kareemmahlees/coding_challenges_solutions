#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(usize),
    BulkString(String),
    Array(Vec<DataType>),
    Null,
}

impl DataType {
    pub fn serialize(&self) -> String {
        let serialized_data = match self {
            DataType::SimpleString(content) => format!("+{content}\r\n"),
            DataType::BulkString(content) => format!("${}\r\n{}\r\n", content.len(), content),
            DataType::Null => format!("$-1\r\n"),
            _ => todo!(),
        };
        serialized_data
    }

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
