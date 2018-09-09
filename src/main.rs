extern crate pancurses;
use pancurses::{endwin, initscr, noecho, Input};
use std::env;
use std::fs::File;
use std::io::*;
use std::path::Path;
mod gap_buffer;
mod renderer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 1 {
        println!("command usage: wim <filename>");
        return;
    }
    let filename = args[1].to_owned();
    let path = Path::new(&filename);
    let exists = path.exists();
    let mut initial_text = String::new();
    if exists {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut initial_text).unwrap();
    }

    let mut gb = gap_buffer::GapBuffer::new_from_string(initial_text);
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    renderer::render(&window, &mut gb);
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                let key_code = c as i32;
                let key_name = pancurses::keyname(key_code).unwrap();
                if key_code == 27 {
                    break;
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
            Some(_input) => {
                //println!("something else {:?}",input)
                ()
            }
            None => (),
        }
        renderer::render(&window, &mut gb);
    }
    endwin();
}

#[cfg(test)]
mod test;
