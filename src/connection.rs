use std::net::{TcpStream};
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, io, net, sync};

use response::Response;
use config::Config;
use command::Command;

enum ConnectionStatus {
    Connected,
    NotConnected,
}

enum ConnectionError {
    NotInitialized,
    UnableToConnect(io::Error),
    DroppedConnection(io::Error),
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

    fn handler(mut stream: TcpStream, receiver: Receiver<Command>, status_channel: Sender<Result<ConnectionStatus, ConnectionError>>) {
        loop {
            match receiver.recv() {
                Ok(command) => {
                    //let e = stream.write(command.bytes);
                    command.respond();
                },

                Err(e) => {
                    // pass
                }
            }
        }
    }

    pub fn send_command(self, command: Command) -> Option<ConnectionError> {
        match self.command_sender {
            Some(sender) => {
                sender.send(command);
                None
            },
            None => Some(ConnectionError::NotInitialized),
        }
    }

    pub fn start(&mut self) -> Receiver<Result<ConnectionStatus, ConnectionError>> {
        let ref config = self.config;

        // initialize the sender/reciever channel for communication with the run loop
        let (sender, receiver) = channel::<Command>();
        self.command_sender = Some(sender.clone());

        // statusSender is an optional channel which can allow clients to listen to error / status
        // events from the connection loop.
        let (status_sender, status_receiver) = channel::<Result<ConnectionStatus, ConnectionError>>();

        let address = config.nsqd_tcp_address.parse::<net::SocketAddr>().unwrap();
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.set_write_timeout(Some(config.write_timeout));
                stream.set_read_timeout(Some(config.read_timeout));

                thread::spawn(move || {
                    Connection::handler(stream, receiver, status_sender);
                });
            },
            Err(_) => {
                status_sender.send(ConnectionError::UnableToConnect);
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
