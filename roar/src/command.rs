pub enum Command {
    Ping,
    Echo,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ping" => Command::Ping,
            "echo" => Command::Echo,
            _ => todo!(),
        }
    }
}
