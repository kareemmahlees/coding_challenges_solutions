use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::DataType;

pub enum Command {
    Ping,
    Echo,
    Set,
    Get,
    Exists,
    Del,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ping" => Command::Ping,
            "echo" => Command::Echo,
            "set" => Command::Set,
            "get" => Command::Get,
            "exists" => Command::Exists,
            "del" => Command::Del,
            _ => todo!(),
        }
    }
}

impl Command {
    pub fn handle_command(
        command: &String,
        arguments: &[DataType],
        dict: Arc<Mutex<HashMap<String, String>>>,
    ) -> DataType {
        match command.to_lowercase().into() {
            Command::Ping => DataType::SimpleString("PONG".to_string()),
            Command::Echo => DataType::BulkString(arguments[0].inner().to_string()),
            Command::Set => {
                dict.lock().unwrap().insert(
                    arguments[0].inner().to_string(),
                    arguments[1].inner().to_string(),
                );
                DataType::SimpleString("OK".to_string())
            }
            Command::Get => match dict.lock().unwrap().get(arguments[0].inner()) {
                Some(entry) => DataType::BulkString(entry.to_string()),
                None => DataType::Null,
            },
            Command::Exists => match dict.lock().unwrap().contains_key(arguments[0].inner()) {
                true => DataType::Integer(1),
                false => DataType::Integer(0),
            },
            Command::Del => {
                let mut num_of_deleted_entries = 0;
                for key in arguments {
                    if dict.lock().unwrap().remove_entry(key.inner()).is_some() {
                        num_of_deleted_entries += 1;
                    };
                }
                DataType::Integer(num_of_deleted_entries)
            }
        }
    }
}
