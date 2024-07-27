use crate::{Command, DataType, DeSerializer};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

/// A simple TCP server to which `redis-cli` will connect.
pub struct Server {
    addr: SocketAddrV4,
}

impl Server {
    /// Create a new instance of `Server`.
    pub fn new(addr: SocketAddrV4) -> Self {
        Self { addr }
    }

    /// Main entry point of the program, listens for incoming requests
    /// and handles them accordingly.
    pub fn listen_and_serve(self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        let dict = Arc::new(Mutex::new(HashMap::new()));
        let dict_vec = Arc::new(Mutex::new(HashMap::new()));

        println!("Server listening on addr {}", self.addr);
        for stream in listener.incoming() {
            let dict_clone = Arc::clone(&dict);
            let dict_vec_clone = Arc::clone(&dict_vec);
            let stream = stream.unwrap();
            println!("New connection: {}", stream.peer_addr().unwrap());
            std::thread::spawn(move || handle_client(stream, dict_clone, dict_vec_clone));
        }
    }
}

/// Responds to the clients commands.
fn handle_client(
    mut stream: TcpStream,
    dict: Arc<Mutex<HashMap<String, String>>>,
    dict_vec: Arc<Mutex<HashMap<String, Vec<String>>>>,
) {
    let mut buffer = [0; 1024];
    let size = stream
        .read(&mut buffer)
        .expect("Failed to read from client");

    let mut deser = DeSerializer::new(&buffer[..size]);

    let data_types = deser.deserialize();

    // NOTE: sorry for this ugly bit but I just realized
    // that redis-cli *Always* sends commands in this format:
    // Array(BulkString(command),BulkString(other_data)).
    match &data_types[0] {
        DataType::Array(data) => match &data[0] {
            DataType::BulkString(command) => {
                let response_data_type =
                    Command::handle_command(command, &data[1..], dict, dict_vec);
                stream
                    .write_all(response_data_type.serialize().as_bytes())
                    .unwrap();
            }
            _ => {}
        },
        _ => {}
    };
    stream.flush().expect("Failed to flush");
}
