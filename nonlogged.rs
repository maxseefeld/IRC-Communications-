use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

use gtk::{Application, ApplicationWindow, Box, Button, Entry, Label, Orientation};

fn main() -> io::Result<()> {
    let server = "chat.freenode.net:6667";
    let nick = "rust_irc_bot";
    let channel = "#rust";
    let mut stream = TcpStream::connect(server)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream.try_clone()?;

    // Send initial messages to server
    writeln!(writer, "NICK {}", nick)?;
    writeln!(writer, "USER {} 0 * :{}", nick, nick)?;
    writeln!(writer, "JOIN {}", channel)?;

    // Spawn a new thread to read messages from the server
    let recv_stream = stream.try_clone()?;
    let send_channel = channel.to_string();
    let recv_handle = thread::spawn(move || {
        let mut reader = BufReader::new(recv_stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            println!("{}", line.trim());
        }
    });

    // Set up the GUI
    let app = Application::new(Some("com.example.rust_irc_bot"), Default::default());
    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Rust IRC Bot");
        window.set_default_size(400, 300);

        let chat_box = Box::new(Orientation::Vertical, 0);
        let chat_label = Label::new(Some("Chat:"));
        let chat_entry = Entry::new();
        let chat_button = Button::new_with_label("Send");
        chat_button.connect_clicked(move |_| {
            let message = format!("PRIVMSG {} :{}\r\n", send_channel, chat_entry.get_text().unwrap().as_str());
            writer.write_all(message.as_bytes()).unwrap();
            chat_entry.set_text("");
        });
        chat_box.pack_start(&chat_label, false, false, 0);
        chat_box.pack_start(&chat_entry, true, true, 0);
        chat_box.pack_start(&chat_button, false, false, 0);

        let loop_box = Box::new(Orientation::Vertical, 0);
        let loop_label = Label::new(Some("Loop:"));
        let loop_entry = Entry::new();
        let loop_button = Button::new_with_label("Send Loop");
        loop_button.connect_clicked(move |_| {
            let message = format!("PRIVMSG {} :{}\r\n", send_channel, loop_entry.get_text().unwrap().as_str());
            writer.write_all(message.as_bytes()).unwrap();
        });
        loop_box.pack_start(&loop_label, false, false, 0);
        loop_box.pack_start(&loop_entry, true, true, 0);
        loop_box.pack_start(&loop_button, false, false, 0);

        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.pack_start(&chat_box, true, true, 0);
        main_box.pack_start(&loop_box, true, true, 0);

        window.add(&main_box);
        window.show_all();
    });
    app.run(&[]);

    // Wait for the receive thread to finish before exiting
    recv_handle.join().unwrap();

    Ok(())
}
