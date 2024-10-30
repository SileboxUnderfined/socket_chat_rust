use std::net::{TcpStream, Shutdown};
use std::io::Write;
use text_io::read;

use crate::message::Message;


pub struct Client {
    name: String
}

impl Client {
    pub fn new(name: String) -> Self {
        return Self {
            name
        };
    }
    pub fn start(&mut self) {
        loop {
            info!("1 - connect to address");
            info!("0 - quit");
            let choice: u8 = read!();

            match choice {
                1 => self.connect_to_server(),
                0 => break,
                _ => info!("Enter number!")
            };
        }
    }

    fn connect_to_server(&mut self) {
        info!("Enter ip_address: ");
        let ip_addr: String = read!();

        let mut stream: TcpStream = TcpStream::connect(ip_addr).expect("Error while connecting to server");

        self.handle_connection(&mut stream);

        stream.shutdown(Shutdown::Both).expect("Error while closing connection to server");
    }

    fn send_message(&self, message: Message, stream: &mut TcpStream) -> std::io::Result<()> {
        let serialized: String = String::from(&message);
        match stream.write(serialized.as_bytes()) {
            Ok(n) => info!("Message was sent successfully! Bytes count: {n}"),
            Err(error) => panic!("Error while sending messsage: {error}")
        };

        Ok(())
    }

    fn handle_connection(&self, stream: &mut TcpStream) {
        let mut choice: u8;
        loop {
            info!("Select action:\n1 - send message\n2 - close connection");
            choice = read!();

            match choice {
                1 => {
                    info!("Enter message: ");
                    let text: String = read!();
                    let message: Message = Message::new(self.name.to_string(),text,false);
                    
                    self.send_message(message, stream).expect("Error while sending message");
                },
                2 => break,
                _ => info!("Enter integer!")
            };
        }

        let close_message: Message = Message::new(self.name.to_string(), String::from("//close_conn"),true);
        self.send_message(close_message, stream).expect("Error while closing connection");
    }
}