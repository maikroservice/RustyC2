// implants are called piglets, they can run code / execute stuff
// they have a list of tasks, bind_port, call_home_addr, protocol
//
use std::fmt;
use std::net::TcpListener;
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
        TcpListener::bind(format!("{}:{}", piglet.call_home_addr, piglet.bind_port,)).unwrap(); // Result[OK] -> T -> unwrap takes this T or if Result[Err] takes the Err

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");

        Command::new("ls")
            .spawn()
            .expect("ls command failed to start");
    }
}

//println!("{} {} {}", piglet.id, piglet.bind_port, piglet.call_home_addr);

//println!("{}", piglet);

//println!("{:?}", piglet);
