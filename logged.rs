use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Connect to the server
    let server = "chat.freenode.net:6667";
    let nick = "my_nickname";
    let channel = "#my_channel";
    let mut stream = TcpStream::connect(server).unwrap();
    let (tx, rx) = channel::<String>();
    let mut file = File::create("messages.txt").unwrap();

    // Send user and nick information
    write!(stream, "USER {0} {0} {0} :{0}\r\n", nick).unwrap();
    write!(stream, "NICK {}\r\n", nick).unwrap();

    // Join a channel
    write!(stream, "JOIN {}\r\n", channel).unwrap();

    // Spawn a thread to read messages from the server
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let reader = BufReader::new(&stream);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains("PRIVMSG") {
                let msg_parts: Vec<&str> = line.split("PRIVMSG").collect();
                let msg = format!("{}\n", msg_parts[1].trim());
                tx_clone.send(msg).unwrap();
            }
        }
    });

    // Spawn a thread to send messages to the server
    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            write!(stream, "PRIVMSG {} :{}\r\n", channel, msg).unwrap();
            file.write_all(msg.as_bytes()).unwrap();
        }
    });

    // Main thread for user input
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == ":q" {
            break;
        }
        if !input.trim().is_empty() {
            tx.send(input.trim().to_owned()).unwrap();
        }
    }
}
