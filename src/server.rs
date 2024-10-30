use core::str;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Read;
use std::thread;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::message::Message;
use crate::db::DB;

pub struct Server {
    listener: TcpListener,
    db: DB
}

impl Server {
    pub fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let port: u16 = rng.gen::<u16>();
        let ip_adrr: String = String::from("127.0.0.1");
        let bind_addr: String = format!("{}:{}", ip_adrr, port.to_string());
        let listener: TcpListener = TcpListener::bind(bind_addr).unwrap();
        let db: DB = DB::new(String::from("server.db"));

        return Self {
            listener,
            db
        };
    }

    fn handle_client(mut stream: TcpStream) -> (SocketAddr, Vec<Message>) {
        let ip_addr: SocketAddr = match stream.peer_addr() {
            Ok(ip) => ip,
            Err(error) => panic!("Error occured on peer_addr(): {error}")
        };

        let mut result: (SocketAddr, Vec<Message>) = (ip_addr, Vec::<Message>::new());
        let mut buffer: [u8; 1024] = [0; 1024];
        
        loop {
            debug!("Reading...");
            let buffer_size: usize = match stream.read(&mut buffer) {
                Ok(0) => continue,
                Ok(n) => n,
                Err(error) => panic!("Error occured while reading stream: {error}")
            };
            debug!("Read..");

            let serialized: String = String::from(str::from_utf8(&buffer[..buffer_size]).expect("Error occured while converting bytes to utf-8"));

            let deserialized: Message = Message::from(&serialized);

            if deserialized.text == String::from("//close_conn") {
                return result;
            }
            
            result.1.push(deserialized.clone());

            info!("{deserialized}");
        }
    }

    pub fn listen(&mut self) {
        let ip_addr: SocketAddr = self.listener.local_addr().unwrap();
        info!(target: "Server", "Listening on: {ip_addr}");

        for stream in self.listener.incoming() {
            match stream {
                Ok(st) => {
                    let handle_stream: thread::JoinHandle<_> = thread::spawn(move || {
                        let result: (SocketAddr, Vec<Message>) = Server::handle_client(st);
                        result
                    });

                    let result: (SocketAddr, Vec<Message>) = handle_stream.join()
                    .expect("Error while joining thread stream");

                    for message in result.1 {
                        self.db.add_message(result.0, message)
                        .expect("Error while adding messages from stream");
                    }
                },
                Err(error) => panic!("Error occured while listening stream: {error}")
            };
        }
    }
}