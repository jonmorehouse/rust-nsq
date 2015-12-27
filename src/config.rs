use std::time::Duration;

enum BackoffStrategy {
    Exponential, 
}

pub struct Config {
    dial_timeout: Duration,

    pub read_timeout: Duration,
    pub write_timeout: Duration,

    pub nsqd_tcp_address: String,

    lookupd_poll_interval: Duration,
    lookupd_poll_jitter: f64,

    max_requeue_delay: Duration,
    default_requeue_delay: Duration,

    backoff_strategy: BackoffStrategy,
    max_backoff_duration: Duration,
    backoff_multiplier: Duration,

    max_attempts: u64,

    low_rdy_idle_timeout: Duration,
    rdy_redistribute_interval: Duration,

    client_id: String,
    hostname: String,
    user_agent: String,

    heartbeat_interval: Duration,
    sample_rate: i32,

    output_buffer_size: i64,
    output_buffer_timeout: Duration,

    max_in_flight: i32,
    message_timeout: Duration,
    auth_secret: String
        //
    // TODO: add in support for TLS configuration, snappy and deflate
}

impl Config {
    pub fn new() -> Config {
        let hostname = "localhost".to_string();

        Config {
            dial_timeout: Duration::new(5, 0),
            read_timeout: Duration::new(60, 0),
            write_timeout: Duration::new(1, 0),
            nsqd_tcp_address: "127.0.0.1:4150".to_string(),
            lookupd_poll_interval: Duration::new(60, 0),
            lookupd_poll_jitter: 1.0,
            max_requeue_delay: Duration::new(60*15, 0),
            default_requeue_delay: Duration::new(90, 0),
            backoff_strategy: BackoffStrategy::Exponential,
            max_backoff_duration: Duration::new(90*2, 0),
            backoff_multiplier: Duration::new(1, 0),
            max_attempts: 5,
            low_rdy_idle_timeout: Duration::new(10, 0),
            rdy_redistribute_interval: Duration::new(5, 0),
            client_id: "localhost".to_string(),
            hostname: hostname,
            user_agent: "rust-nsq/localhost".to_string(),
            heartbeat_interval: Duration::new(30, 0),
            sample_rate: 0,
            output_buffer_size: 16384,
            output_buffer_timeout: Duration::new(1, 0),
            max_in_flight: 1,
            message_timeout: Duration::new(60, 0),
            auth_secret: "secret".to_string(),
        }
    }

    fn validate() -> bool {
        // TODO: validate the configuration here. Its expectant, that as a first pass clients using
        // this library will manually modify attributes of the config struct as needed
        true
    }
}