use std::{sync::mpsc::{Receiver, self, Sender}, thread};

use crossterm::event;
use term::{carat_blinker, InteractiveTermRecievers};


mod pack;

mod term;


fn main() -> std::io::Result<()> {

    let mut stdout = std::io::stdout();

    // Carat Blinker Thread
    let (cb_tx, cb_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    thread::spawn(move || carat_blinker(cb_tx));

    // Event Recievers
    let (event_tx, event_rx): (Sender<crossterm::event::Event>, Receiver<crossterm::event::Event>) = mpsc::channel();
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = event::read() {
                if event_tx.send(event).is_err() {
                    eprintln!("[-] Error: Failed To Send Event To Reciever.");
                    break;
                }
            }
        }
    });




    let recievers = InteractiveTermRecievers {
        carat_blinker: cb_rx,
        event: event_rx,
    };

    term::interactive_term(&mut stdout, &recievers)
}
