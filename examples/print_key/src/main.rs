//! cd to the print_key repository then do `cargo run`
use {
    crokey::*,
    crossterm::{
        event::{read, Event},
        style::Stylize,
        terminal,
    },
};

pub fn main() {
    let fmt = KeyEventFormat::default();
    println!("\nType any key combination (remember that your terminal intercepts many ones)");
    loop {
        terminal::enable_raw_mode().unwrap();
        let e = read();
        terminal::disable_raw_mode().unwrap();
        match e {
            Ok(Event::Key(key_event)) => match key_event {
                key!(ctrl-c) => {
                    println!(
                        "Arg! You savagely killed me with a {}",
                        fmt.to_string(key_event).red()
                    );
                    break;
                }
                key!(ctrl-q) => {
                    println!(
                        "You typed {} which gracefully quits",
                        fmt.to_string(key_event).green()
                    );
                    break;
                }
                key!('?') | key!(shift-'?') => {
                    println!("There's no help on this app");
                }
                _ => {
                    println!("You typed {}", fmt.to_string(key_event).blue());
                }
            },
            e => {
                // any other even, for example a resize, we quit
                eprintln!("quitting on {:?}", e);
                break;
            }
        }
    }
}
