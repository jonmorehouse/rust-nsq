use std::sync::mpsc::{channel, Receiver, Sender};
use std::string::{String};

pub struct Response {
    pub t: bool,
}

pub struct Command {
    success: bool,
    sender: Sender<Response>,
    bytes: Vec<u8>,
}

impl Command {
    fn new_from_string(command: String) -> (Command, Receiver<Response>) {
        let (sender, receiver) = channel::<Response>();
        let bytes = command.into_bytes();

        let command = Command{
            sender: sender.clone(),
            success: true,
            bytes: bytes,
        };

        (command, receiver)
    }

    pub fn respond(&self) {
        println!("hello world");
        self.sender.send(Response{t: true});
    }

    pub fn version() -> (Command, Receiver<Response>) {
        let (command, receiver) = Command::new_from_string("  V2".to_string());

        (command, receiver)
    }

    fn identify() -> (Command, Receiver<Response>) {
        // TODO: build identify command here
        Command::version()
    }
}
