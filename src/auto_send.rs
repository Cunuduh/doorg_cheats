use std::{io::{self, stdout, Write}, thread, time::Duration};
use crossterm::{cursor::{self}, event::{read, Event, KeyCode, KeyModifiers, KeyEvent}, QueueableCommand, terminal::{self}};
use enigo::{Enigo, Key, KeyboardControllable};
pub fn send() {
    let mut stdout = stdout();
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
    for i in (0..=5).rev() {
        stdout.queue(cursor::SavePosition).unwrap();
        stdout.write_all(format!("Sending in {} second(s) . . .", i).as_bytes()).unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
    loop {
        enigo.key_sequence(&input);
        enigo.key_click(Key::Return);
        thread::sleep(delay);
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