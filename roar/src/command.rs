use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::DataType;

/// Supported commands by `Roar`.
pub enum Command {
    Ping,
    Echo,
    Set,
    Get,
    Exists,
    Del,
    Incr,
    Decr,
    Rpush,
    Lpush,
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
            "incr" => Command::Incr,
            "decr" => Command::Decr,
            "rpush" => Command::Rpush,
            "lpush" => Command::Lpush,
            _ => todo!(),
        }
    }
}

impl Command {
    /// Execute commands sent by the client.
    pub fn handle_command(
        command: &String,
        arguments: &[DataType],
        dict: Arc<Mutex<HashMap<String, String>>>,
        dict_vec: Arc<Mutex<HashMap<String, Vec<String>>>>,
    ) -> DataType {
        let mut dict = dict.lock().unwrap();
        match command.to_lowercase().into() {
            Command::Ping => DataType::SimpleString("PONG".to_string()),
            Command::Echo => DataType::BulkString(arguments[0].inner().to_string()),
            Command::Set => {
                dict.insert(
                    arguments[0].inner().to_string(),
                    arguments[1].inner().to_string(),
                );
                DataType::SimpleString("OK".to_string())
            }
            Command::Get => match dict.get(arguments[0].inner()) {
                Some(entry) => DataType::BulkString(entry.to_string()),
                None => DataType::Null,
            },
            Command::Exists => match dict.contains_key(arguments[0].inner()) {
                true => DataType::Integer(1),
                false => DataType::Integer(0),
            },
            Command::Del => {
                let mut num_of_deleted_entries = 0;
                for key in arguments {
                    if dict.remove_entry(key.inner()).is_some() {
                        num_of_deleted_entries += 1;
                    };
                }
                DataType::Integer(num_of_deleted_entries)
            }
            Command::Incr => {
                let key = arguments[0].inner();
                let value = dict.entry(key.to_string()).or_insert("0".to_string());
                match value.parse::<i64>() {
                    Ok(parsed_value) => {
                        let new_value = parsed_value + 1;
                        dict.insert(key.to_string(), new_value.to_string());
                        DataType::Integer(new_value)
                    }
                    Err(_) => {
                        DataType::Error("value is not an integer or out of range".to_string())
                    }
                }
            }
            Command::Decr => {
                let key = arguments[0].inner();
                let value = dict.entry(key.to_string()).or_insert("0".to_string());
                match value.parse::<i64>() {
                    Ok(parsed_value) => {
                        let new_value = parsed_value - 1;
                        dict.insert(key.to_string(), new_value.to_string());
                        DataType::Integer(new_value)
                    }
                    Err(_) => {
                        DataType::Error("value is not an integer or out of range".to_string())
                    }
                }
            }
            Command::Rpush => {
                if dict.get(arguments[0].inner()).is_some() {
                    return DataType::Error("value is not an array".to_string());
                }
                let mut dict_vec = dict_vec.lock().unwrap();
                let array = dict_vec
                    .entry(arguments[0].inner().to_string())
                    .or_default();
                for arg in arguments[1..].iter() {
                    array.push(arg.inner().to_string());
                }
                DataType::Integer(array.len() as i64)
            }
            Command::Lpush => {
                if dict.get(arguments[0].inner()).is_some() {
                    return DataType::Error("value is not an array".to_string());
                }
                let mut dict_vec = dict_vec.lock().unwrap();
                let array = dict_vec
                    .entry(arguments[0].inner().to_string())
                    .or_default();
                for arg in arguments[1..].iter() {
                    array.insert(0, arg.inner().to_string());
                }
                DataType::Integer(array.len() as i64)
            }
        }
    }
}
