use crate::deserializer::DataType;
use crate::DeSerializer;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};

pub struct Server {
    addr: SocketAddrV4,
}

impl Server {
    pub fn new(addr: SocketAddrV4) -> Self {
        Self { addr }
    }

    pub fn listen_and_serve(self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        println!("Server listening on addr {}", self.addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("New connection: {}", stream.peer_addr().unwrap());
            std::thread::spawn(move || handle_client(stream));
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let size = stream
        .read(&mut buffer)
        .expect("Failed to read from client");

    let mut deser = DeSerializer::new(&buffer[..size]);

    let data_types = deser.deserialize();
    match &data_types[0] {
        DataType::Array(data) => match &data[0] {
            DataType::BulkString(content) => {
                match content.to_lowercase().as_str() {
                    "ping" => stream.write_all("$4\r\nPONG\r\n".as_bytes()).unwrap(),
                    // "echo" => stream.write_all("")
                    _ => {}
                };
            }
            _ => {}
        },
        _ => {}
    };
    stream.flush().expect("Failed to flush");
}
