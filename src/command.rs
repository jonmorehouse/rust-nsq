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
    success: bool,
    sender: Sender<Response>,
    pub bytes: Vec<u8>,
}

trait HasResponse {
    fn response_type(&self) -> ResponseType;
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
            success: true,
            bytes: bytes,
        };

        (command, receiver)
    }

    pub fn respond(&self) {
        self.sender.send(Response{t: true});
    }

    pub fn response_type(self) -> ResponseType {
        self.response_type
    }

    pub fn version() -> (Command, Receiver<Response>) {
        let (mut command, receiver) = Command::new_from_string("  V2".to_string());
        (command, receiver)
    }
}

pub struct ProtocolCommand {
    // a protocol command is a command which adheres to the NSQ protocol. These sorts of commands
    // use JSON
    response_type: ResponseType,
    pub bytes: Vec<u8>,
}

impl ProtocolCommand {
    fn new(command: String, params: String, data: String) -> ProtocolCommand {
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

        ProtocolCommand{
            bytes: buffer,
            response_type: ResponseType::SizePrefixed,
        }
    }

    pub fn identify() -> (Command, Receiver<Response>) {
        let identify = "IDENTIFY\n{\"client_id\": \"test\"}";
        let (mut command, receiver) = Command::new_from_string(identify.to_string());
        
        (command, receiver)
    }
}

//impl fmt::Display for ProtocolCommand {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}", String::from_utf8(self.bytes).unwrap())
    //}
//}

#[test]
fn test_protocol_command() {
    let protocolCommand = ProtocolCommand::new("IDENTIFY".to_string(), "".to_string(), "{\"client_id\":\"test\"}".to_string());
}

