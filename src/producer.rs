use std::thread;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use config::Config;
use connection::Connection;

// The producer class uses the TCP connection aspect of NSQ
pub struct Producer<'a> {
    config: &'a Config,
    connection: Connection<'a>,
}

impl <'a> Producer<'a> {
    pub fn new(config: &'a Config) -> Producer<'a> {
        let p = Producer {
            config: config,
            connection: Connection::new(config),
        };
        p
    }

    //pub fn publish(&mut self) -> mpsc::Receiver<Result<i8, bool>> {

    //}

    //pub fn publish(&mut self) -> mpsc::Receiver<i32> {
        //self.connection.connect();

    //}
}


