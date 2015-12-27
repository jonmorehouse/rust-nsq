use std::net::{TcpStream};
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, io, net, sync};

use response::Response;
use config::Config;
use command::Command;

use std::time::Duration;

enum Status {
    Connected,
    NotConnected,
}

enum Error {
    NotInitialized,
    UnableToConnect,
    DroppedConnection,
}

struct Connection {
    config: Config,
    command_sender: Option<Sender<Command>>,
    command_receiver: Option<Receiver<Command>>,
}

impl Connection {
    fn new() -> Result<Connection, io::Error> {
        let conn = Connection{
            config: Config::new(),
            command_sender: None,
            command_receiver: None,
        };
        return Ok(conn);
    }

    fn handler(mut stream: TcpStream, receiver: Receiver<Command>, status_channel: Sender<Result<Status, Error>>) {
        loop {
            match receiver.recv() {
                Ok(command) => {
                    command.respond();
                    //stream.write(&command.bytes);
                },
                Err(e) => {
                    // pass
                }
            }
        }
    }

    pub fn send_command(&self, command: Command) -> Option<Error> {
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

        // initialize the sender/reciever channel for communication with the run loop
        let (sender, receiver) = channel::<Command>();
        self.command_sender = Some(sender.clone());

        // statusSender is an optional channel which can allow clients to listen to error / status
        // events from the connection loop.
        let (status_sender, status_receiver) = channel::<Result<Status, Error>>();

        let address = config.nsqd_tcp_address.parse::<net::SocketAddr>().unwrap();
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.set_write_timeout(Some(config.write_timeout));
                stream.set_read_timeout(Some(config.read_timeout));

                status_sender.send(Ok(Status::Connected));
                thread::spawn(move || {
                    Connection::handler(stream, receiver, status_sender);
                });
            },
            Err(_) => {
                status_sender.send(Err(Error::UnableToConnect));
            }
        }

        status_receiver
    }
}

#[test]
fn it_works() {
    let mut c = Connection::new().unwrap();
    c.start();

    let (command, responseChannel) = Command::version();
    c.send_command(command);
    let response = responseChannel.recv().unwrap();
    println!("{}", response.t);
}
