use std::{io, str::FromStr};

use alt_ansi::colour::Colours;
use alt_dsa::queue::circular::CircularQueue;
use clap::Parser;
use tokio::sync::{mpsc};

mod args;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let (tx, mut rx) = mpsc::channel::<String>(32);

    //TODO: handle shutdown? like ctrl c etc?

    tokio::spawn(async move {
        let mut buf = String::new();

        while let Ok(bytes_read) = io::stdin().read_line(&mut buf) {
            if bytes_read == 0 { break; }

            //TODO: cloning cannot be good here
            tx.send(buf.clone()).await.expect("Could not send msg");

            buf.clear();
        };

    });

    let target_ignore_count = args.ignore_lines.unwrap_or(0);
    let mut ignored_count = 0;
    let mut colours: CircularQueue<String> = args.colours.into();

    while let Some(line) = rx.recv().await {
        if ignored_count != target_ignore_count {
            println!("{line}");

            ignored_count += 1;
        } else {
            let current = colours.next();
            let current_colour: Colours = Colours::from_str(current)
                .expect(&format!("Unknown colour '{}'", current));
            let ansi = Colours::get_full_ansi_code(current_colour, Colours::Black);

            print!("{}{}", ansi, line)
        }
    };

    Ok(())
}

