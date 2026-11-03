use std::{collections::VecDeque, io};

use clap::Parser;
use tokio::sync::{mpsc};
use crate::env::{IFS, NEWLINE};

mod args;
mod stdin;
mod env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = args::Args::parse();
    let ifs: Vec<u8> = get_ifs();
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
    let mut colour_deque = VecDeque::from(args.colours);

    while let Some(line) = rx.recv().await {
        if ignored_count != target_ignore_count {
            println!("{line}");

            ignored_count += 1;
        } else {
            //TODO: impl circular queue
            let current_colour = colour_deque.pop_front()
                .expect("could not get next colour");
            colour_deque.push_back(current_colour.clone());

            println!("{}{}", current_colour, line)
        }
    };

    Ok(())
}

fn get_ifs() -> Vec<u8> {
    let var = env::var_or(IFS, NEWLINE);

    //TODO: if IFS contains multi byte characters how are we supposed to handle that?
    // We can read them but does it violate spec? (what spec?)
    var.as_bytes().to_vec()
}

fn split_ignored_lines(input: Vec<String>, ignored_lines_count: usize) -> (Vec<String>, Vec<String>) {
    if ignored_lines_count > input.len() {
        return (input, vec![]);
    }
    
    //TODO: this might get intensive with big stdin
    let mut original = input.clone();
    let xs = original.split_off(ignored_lines_count);

    (original, xs)
}

