use std::net::{TcpStream};
use std::sync::mpsc::{channel};
use std::{thread, io};

use response::Response;
use config::Config;

struct Connection {
    config: Config,
}


impl Connection {
    fn new() -> Result<Connection, io::Error> {
        // TODO call the handler method and start the TCP connection
        let (tx, rx) = channel::<Response>();

        let conn = Connection{config: Config::new()};
        return Ok(conn);
    }


    // TODO: run in background thread and listen for new commands
    fn handler(&self) {
        loop {

        }
    }
    
    // THOUGHT: consumer/producer will be responsible for passing in an Identify command here
    // TODO: figure out some sort of Command object
    //fn Execute() -> Result<Response> {

    //}
}

#[test]
fn it_works() {
    let c = Connection::new();
}
