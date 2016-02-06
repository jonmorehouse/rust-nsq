use std::net::{TcpStream};
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, io, net, sync};

use response::Response;
use config::Config;
use command::{Command, HasData, HasResponse, ProtocolCommand};

enum Status {
    Connected,
    NotConnected,
}

enum Error {
    NotInitialized,
    UnableToConnect,
    DroppedConnection,
}

pub struct Connection<T: HasData + HasResponse +'static> {
    config: Config,
    command_sender: Option<Sender<T>>,
    command_receiver: Option<Receiver<T>>,
}

impl <T: HasData + HasResponse + 'static> Connection<T> {
    pub fn new() -> Result<Connection<T>, io::Error> {
        let conn = Connection{
            config: Config::new(),
            command_sender: None,
            command_receiver: None,
        };
        return Ok(conn);
    }

    fn handler() -> () {
    //fn handler(mut stream: TcpStream, receiver: Receiver<T>, status_channel: Sender<Result<Status, Error>>) {
        //loop {
            //match receiver.recv() {
                //Ok(command) => {
                    //command.respond();
                    ////stream.write(&command.data());
                    ////stream.write(&command.bytes);
                //},
                //Err(e) => {
                    //// pass
                //}
            //}
        //}
    }

    pub fn send_command(&self, command: T) -> Option<Error> {
        match self.command_sender {
            Some(ref sender) => {
                sender.send(command);
                None
            },
            None => Some(Error::NotInitialized),
        }
    }

    pub fn start(&mut self) -> Receiver<Result<Status, Error>> {
        let ref config = self.config;

        // initialize the sender/reciever channel for command communication with the run loop
        let (sender, receiver) = channel::<T>();
        self.command_sender = Some(sender.clone());

        // status sender is a method which returns status events about the state of the connection
        let (status_sender, status_receiver) = channel::<Result<Status, Error>>();
        let address = config.nsqd_tcp_address.parse::<net::SocketAddr>().unwrap();

        thread::spawn(move || {
            match TcpStream::connect(address) {
                Ok(mut stream) => {
                    // We want to be able to consume commands that uphold the T trait contract
                    let command = receiver.recv().unwrap();

                },
                Err(_) => {
                    status_sender.send(Err(Error::UnableToConnect));
                }
            }
        });
        
        status_receiver
    }
}
