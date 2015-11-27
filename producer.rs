//use std::thread;

struct Config {
    nsqd_tcp_addresses: String,
}

// The producer class uses the TCP connection aspect of NSQ
struct Producer {
    config: Config,
}

impl Producer {

    fn new() -> Producer {
        Producer {
            config: Config{
                nsqd_tcp_addresses: "asdf".to_string(),
            }
        }
    }
}

pub fn new_producer() {
    let p = Producer::new();
    println!("{} test.", p.config.nsqd_tcp_addresses);
    //println!(p.config.nsqd_tcp_addresses);

}

