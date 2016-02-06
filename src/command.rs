extern crate byteorder;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::string::{String};
use std::{fmt};

use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt};

enum ResponseType {
    Empty,
    SizePrefixed,
}

pub struct Response {
    pub t: bool,
}

pub struct Command {
    response_type: ResponseType,
    sender: Sender<Response>,
    pub bytes: Vec<u8>,
}

pub trait HasData {
    fn data(self) -> Vec<u8>;
}

pub trait HasResponse {
    fn response_type(self) -> ResponseType;
    fn respond(&self);
}

static ByteNewLine: &'static [u8]  = b"\n";
static ByteSpace: &'static [u8] = b" ";

impl Command {
    fn new_from_string(command: String) -> (Command, Receiver<Response>) {
        let (sender, receiver) = channel::<Response>();
        let bytes = command.into_bytes();

        let command = Command{
            response_type: ResponseType::Empty,
            sender: sender.clone(),
            bytes: bytes,
        };

        (command, receiver)
    }

    pub fn version() -> (Command, Receiver<Response>) {
        let (mut command, receiver) = Command::new_from_string("  V2".to_string());
        (command, receiver)
    }
}

impl HasData for Command {
    // TODO share a scoped reference here
    fn data(self) -> Vec<u8> {
        self.bytes
    }
}

impl HasResponse for Command {
    fn respond(&self) {
        self.sender.send(Response{t: true});
    }

    fn response_type(self) -> ResponseType {
        self.response_type
    }
}

pub struct ProtocolCommand {
    // a protocol command is a command which adheres to the NSQ protocol. These sorts of commands
    // use JSON
    response_type: ResponseType,
    sender: Sender<Response>,
    pub bytes: Vec<u8>,
}

impl ProtocolCommand {
    pub fn new(command: String, params: String, data: String) -> (ProtocolCommand, Receiver<Response>) {
        // TODO: clean up this function
        let mut buffer: Vec<u8> = Vec::new();

        for byte in command.into_bytes() {
            buffer.push(byte);
        }

        let space = " ".to_string().into_bytes();
        let new_line = "\n".to_string().into_bytes();

        buffer.push(space[0]);
        for byte in params.into_bytes() {
            buffer.push(byte);
        }
        buffer.push(new_line[0]);

        {
            let data_bytes = data.into_bytes();
            // TODO: handle if the len is greater than a u32 value
            let len:u32 = data_bytes.len() as u32;
            buffer.write_u32::<BigEndian>(len);

            // now write the data into the command
            for byte in data_bytes {
                buffer.push(byte);
            }
        }

        let (sender, receiver) = channel::<Response>();
        let command = ProtocolCommand{
            sender: sender,
            bytes: buffer,
            response_type: ResponseType::SizePrefixed,
        };

        (command, receiver)
    }

    pub fn identify() -> (ProtocolCommand, Receiver<Response>) {
        ProtocolCommand::new("IDENTIFY".to_string(), "".to_string(), "{\"client_id\":\"test\"}".to_string())
    }
}

impl HasData for ProtocolCommand {
    fn data(self) -> Vec<u8> {
        self.bytes
    }
}

impl HasResponse for ProtocolCommand {
    fn respond(&self) {
        self.sender.send(Response{t: true});
    }

    fn response_type(self) -> ResponseType {
        self.response_type
    }
}

impl fmt::Display for ProtocolCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = String::from_utf8(self.bytes.clone()).unwrap();
        write!(f, "{}", string)
    }
}

#[test]
fn test_protocol_command() {
    let (protocol_command, receiver) = ProtocolCommand::new("IDENTIFY".to_string(), "".to_string(), "{\"client_id\":\"test\"}".to_string());
    println!("{}", protocol_command);
}

#[test]
fn test_protocol_command_traits() {
    let (protocol_command, receiver) = ProtocolCommand::new("IDENTIFY".to_string(), "".to_string(), "".to_string());
    println!("{}", protocol_command);
}
