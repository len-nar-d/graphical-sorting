use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, io::stdin};
use termion::{event::Key, input::TermRead};

mod output;
mod algorithms;

/*
messages between threads:

[u16; 3] = [a, b, c]

a <- Identifier(0 - Place, 1 - Swap, 2 - Verify, 3 - Clear)
b <- Number 1
c <- Number 2
*/

fn main() {
    let (transmitter, receiver): (Sender<[u16; 3]>, Receiver<[u16; 3]>) = channel();

    thread::Builder::new().name("output".to_string()).spawn(move || {
        output::run(receiver);
    }).expect("output thread couldn't start");

    thread::Builder::new().name("ctlrc".to_string()).spawn( || {
        for c in stdin().keys() {
            match c.expect("unknown user input") {
                Key::Ctrl('c') => {
                    std::process::exit(0);
                },
                _ => {
                    continue;
                }
            }
        }
    }).expect("ctlrc thread couldn't start");

    loop {
        let mut bubble_sort = algorithms::BubbleSort::new(transmitter.clone());
        bubble_sort.sort();
    }
}

