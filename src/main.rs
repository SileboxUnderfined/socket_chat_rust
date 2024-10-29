use clap::{Parser, ValueEnum};
use std::thread;
use simple_logger::SimpleLogger;

mod server;
mod client;
mod message;
use crate::server::Server;
use crate::client::Client;

#[macro_use]
extern crate log;

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_enum, default_value = "both")]
    start_mode: StartMode
}

#[derive(ValueEnum, Clone, Debug)]
enum StartMode {
    Client,
    Server,
    Both
}

fn run_server() {
    let server: Server = Server::new();

    server.listen();
}

fn run_client() {
    let mut client: Client = Client::new(String::from("lol"));

    client.start();
}

fn run_both() {
    let handle_server: thread::JoinHandle<_> = thread::spawn(run_server);
    let handle_client: thread::JoinHandle<_> = thread::spawn(run_client);

    handle_server.join().unwrap();
    handle_client.join().unwrap();
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args: Args = Args::parse();

    match args.start_mode {
        StartMode::Client => run_client(),
        StartMode::Server => run_server(),
        StartMode::Both => run_both()
    };
}
