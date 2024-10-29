use core::str;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Read;
use std::thread;
use rand::Rng;
use rand::rngs::ThreadRng;
use serde_json;

use crate::message::Message;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let port: u16 = rng.gen::<u16>();
        let ip_adrr: String = String::from("127.0.0.1");
        let bind_addr: String = format!("{}:{}", ip_adrr, port.to_string());
        let listener: TcpListener = TcpListener::bind(bind_addr).unwrap();

        return Self {
            listener
        };
    }

    fn handle_client(mut stream: TcpStream) {
        /*let ip_addr: SocketAddr = match stream.peer_addr() {
            Ok(ip) => ip,
            Err(error) => panic!("Error occured on peer_addr(): {error}")
        };*/
        
        let mut buffer: [u8; 1024] = [0; 1024];
        
        loop {
            debug!("Reading...");
            let buffer_size: usize = match stream.read(&mut buffer) {
                Ok(0) => continue,
                Ok(n) => n,
                Err(error) => panic!("Error occured while reading stream: {error}")
            };
            debug!("Read..");
    
            let serialized: String = match str::from_utf8(&buffer[..buffer_size]) {
                Ok(v) => String::from(v),
                Err(error) => panic!("Error occured while converting bytes to utf-8: {error}")
            };

            let deserialized: Message = match serde_json::from_str(&serialized) {
                Ok(m) => m,
                Err(error) => panic!("Error while deserializing message: {error}")
            };

            if deserialized.get_text() == String::from("//close_conn") {
                break;
            }

            info!("{deserialized}");
        }
    }

    pub fn listen(&self) {
        let ip_addr: SocketAddr = self.listener.local_addr().unwrap();
        info!(target: "Server", "Listening on: {ip_addr}");

        for stream in self.listener.incoming() {
            match stream {
                Ok(st) => {
                    let handle_stream: thread::JoinHandle<_> = thread::spawn(move || {
                        Server::handle_client(st);
                    });

                    handle_stream.join().unwrap();
                },
                Err(error) => panic!("Error occured while listening stream: {error}")
            };
        }
    }
}