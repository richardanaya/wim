extern crate pancurses;
use pancurses::{endwin, initscr, noecho, Input};
use std::env;
use std::fs::File;
use std::io::*;
use std::path::Path;
mod gap_buffer;
mod renderer;

fn run(window: &pancurses::Window, gb: &mut gap_buffer::GapBuffer, path: &Path) {
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                let key_code = c as i32;
                let key_name = pancurses::keyname(key_code).unwrap();
                // If the press Escape, exit
                if key_code == 27 {
                    break;
                // If they press CTR+S, save and exit
                } else if key_name == "^S" {
                    let mut file = File::create(path).unwrap();
                    file.write_all(gb.to_string().as_bytes()).unwrap();
                    break;
                } else {
                    gb.insert_char(c);
                }
            }
            Some(Input::KeyBackspace) => {
                gb.remove_char();
            }
            Some(Input::KeyLeft) => {
                gb.shift_gap_backward();
            }
            Some(Input::KeyRight) => {
                gb.shift_gap_forward();
            }
            _ => (),
        }
        renderer::render(&window, &gb);
    }
}

fn main() {
    // Get file specified to create/open
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("command usage: wim <filename>");
        return;
    }

    // See if this path exists
    let filename = args[1].to_owned();
    let path = Path::new(&filename);
    let exists = path.exists();

    // If this file exists get its text to initialize the gap buffer
    let mut initial_text = String::new();
    if exists {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut initial_text).unwrap();
    }

    // Create gap buffer
    let mut gb = gap_buffer::GapBuffer::new_from_string(initial_text);

    // Start up ncurses
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    // Render first time
    renderer::render(&window, &gb);

    // Start render loop
    run(&window, &mut gb, path);
    endwin();
}

#[cfg(test)]
mod test;
