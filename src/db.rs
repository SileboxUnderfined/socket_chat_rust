extern crate rusqlite;

use std::net::SocketAddr;
use std::path::Path;
use rusqlite::{Connection, Result, Statement};

use crate::message::Message;

pub struct DB {
    connection: Connection,
}

impl DB {
    pub fn new(name: String) -> Self {
        let exists: bool = Path::new(&name).exists();
        let connection: Connection = Connection::open(&name).expect("Error while opening database");
        
        if !exists {
            DB::initialize_db(&connection);
        }

        return DB {
            connection
        }
    }

    pub fn add_message(&mut self, ip_addr: SocketAddr, data: Message) -> Result<()> {
        let mut add_msg_st: Statement<'_> = self.connection.prepare("insert into messages (ip_addr, data) values (?, ?)")
        .expect("Error while preparing statement");

        add_msg_st.execute([ip_addr.to_string(), String::from(&data)])?;

        Ok(())
    }

    fn initialize_db(conn: &Connection) {
        conn.execute(
            "create table messages (
            id integer not null primary key autoincrement,
            ip_addr text not null,
            data text not null
        )",()
        ).expect("Error while creating table messages");
    }
}