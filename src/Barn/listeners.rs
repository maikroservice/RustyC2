// a listener is running on the attacker TeamServer / Host side
// a listener can have different protocols (e.g. MQTT/HTTP/HTTPS/SMB/LDAP?)
// a listener has a name, a tasklist, a bindport, and?
//

// extern crate chrono;
// extern crate uuid;
use std::net::TcpListener;
use tasks;
mod tasks;

pub struct Listener {
    pub name: String,
    pub tasks: tasks::Tasks,
    pub bind_port: u16,
}

impl Listener {
    fn main() {
        let listener = TcpListener::bind("0.0.0.0:{}", self.bind_port).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
        }
    }
}
