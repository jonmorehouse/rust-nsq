use config::{Config};
use command::{ProtocolCommand, Command, HasData, HasResponse};
use connection::{Connection};

pub struct Producer {
    t: bool
}

impl Producer {
    fn  new(config: Config) -> Producer{
        // create a new producer 
        Producer{
            t: true
        }
    }

    // publish many messages at once 
    fn mpublish() -> bool {
        true
    }

    fn publish() -> bool {

        true
    }

    // TODO: decide how we should handle this and whether or not all publish commands should use
    // channel based communication to handle all of this in multi threaded environments
    fn sync_publish() -> bool {
        true
    }

    fn sync_mpublish() -> bool {
        true
    }
}
