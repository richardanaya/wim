extern crate pancurses;
use pancurses::{endwin, initscr, noecho, Input};
use std::env;
use std::fs::File;
use std::path::Path;
use text_document::CursorDirection;
mod gap_buffer;
mod renderer;
mod text_document;

fn run(window: &pancurses::Window, mut doc: text_document::TextDocument) {
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
                    doc.save();
                    break;
                } else {
                    doc.add_char(c);
                }
            }
            Some(Input::KeyBackspace) => {
                doc.remove_char();
            }
            Some(Input::KeyLeft) => {
                doc.move_cursor(CursorDirection::Left);
            }
            Some(Input::KeyRight) => {
                doc.move_cursor(CursorDirection::Right);
            }
            Some(Input::KeyUp) => {
                doc.move_cursor(CursorDirection::Up);
            }
            Some(Input::KeyDown) => {
                doc.move_cursor(CursorDirection::Down);
            }
            _ => (),
        }
        renderer::render(&window, &doc.gap_buffer);
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
    let path_arg = args[1].to_owned();
    let path = &Path::new(&path_arg);
    let doc = text_document::TextDocument::create_or_open(path);

    // Start up ncurses
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    // Render first time
    renderer::render(&window, &doc.gap_buffer);

    // Start render loop
    run(&window, doc);
    endwin();
}

#[cfg(test)]
mod test;
