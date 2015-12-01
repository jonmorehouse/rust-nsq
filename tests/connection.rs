extern crate nsq;

use std::thread::sleep;
use std::time::Duration;

#[test]
fn connection_connect_test() {
    let config = nsq::config::Config::new();
    let mut conn = nsq::connection::Connection::new(&config);
    match conn.connect() {
        Err(e) => println!("{}", e),
        Ok(()) => println!("success"),
    };

    sleep(Duration::new(5, 0));
    conn.disconnect();
}
