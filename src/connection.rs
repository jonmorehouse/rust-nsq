use std::io::prelude::*;
use std::net::TcpStream;
use std::cell::Cell;
use std::io;
use std::io::ErrorKind;
use std::net::Shutdown;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc;


use config::Config;

pub struct Connection<'a> {
    config: &'a Config,
    stream: Option<TcpStream>,
}

impl <'a> Connection<'a> {
    pub fn new(config: &'a Config) -> Connection<'a> {
        Connection{
            config: config,
            stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), io::Error> {
        let cls = || -> Option<TcpStream> {
            match TcpStream::connect("127.0.0.1:4150") {
                Ok(conn) => Some(conn),
                Err(e) => None,
            }
        };

        self.stream = cls();
        match self.stream {
            Some(_) => {
                match self.configure_connection() {
                    Ok(_) => {
                        self.write_loop();
                        Ok(())
                    }
                    Err(e) => Err(e)
                }
            }
            None => Err(io::Error::new(ErrorKind::NotConnected, "not connected")),
        }
    }

    pub fn configure_connection(&mut self) -> Result<(), io::Error> {
        match self.stream {
            Some(ref mut stream) => {
                stream.set_write_timeout(Some(self.config.write_timeout));
                stream.set_read_timeout(Some(self.config.write_timeout));

                let res = match stream.write_fmt(format_args!("  V2")) {
                    Ok(_) => {
                        stream.flush()
                    },
                    Err(e) => Err(e),
                };
                return res;
            },
            None => Err(io::Error::new(ErrorKind::NotConnected, "no connection")),
        }
    }

    fn identify(&mut self) -> Result<(), io::Error> {
        match self.stream {
            Some(ref mut c) => {
                Ok(())
            },
            None => Ok(())
        }
    }

    pub fn disconnect(&mut self) -> Result<(), io::Error> {
        match self.stream {
            None => Err(io::Error::new(ErrorKind::NotConnected, "no connection")),
            Some(ref mut conn) => {
               conn.shutdown(Shutdown::Both) 
            }
        }
    }

    pub fn write_loop(&mut self) -> mpsc::Sender<i8> {
        let (sender, receiver) = channel();

        thread::spawn(move|| {
            while true {
                println!("started receiving");
                let t = receiver.recv();
                match t {
                    Ok(msg) => {
                        println!("message was received");
                    },
                    Err(e) => {
                        // there was an error - need to notify another channel?
                        println!("timed out");
                    }
                }
            }
        });

        sender
    }
}


