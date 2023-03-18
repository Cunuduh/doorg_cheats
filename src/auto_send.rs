use std::{io, thread, time::Duration};
use crossterm::event::{read, Event, KeyCode, KeyModifiers, KeyEvent};
use enigo::{Enigo, Key, KeyboardControllable};
pub fn send() {
    let mut enigo = Enigo::new();
    let mut delay = String::new();
    println!("Press ESC to quit while focused on the console application.");
    println!("Enter delay in milliseconds: ");
    io::stdin().read_line(&mut delay).unwrap();

    let delay = Duration::from_millis(delay.trim().parse::<u64>().unwrap());
    let mut input = String::new();
    println!("Enter text to send: ");
    io::stdin().read_line(&mut input).unwrap();
    
    thread::spawn(listen);
    println!("Starting in 5 seconds . . . ");
    thread::sleep(Duration::from_secs(5));
    loop {
        thread::sleep(delay);
        enigo.key_sequence(&input);
        enigo.key_click(Key::Return);
    }
}
fn listen() {
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE, .. } ) => {
                println!("Quitting in 1 second . . .");
                thread::sleep(Duration::from_secs(1));
                std::process::exit(0);
            }
            _ => {}
        }
    }
}