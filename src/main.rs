extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input};
mod gap_buffer;

fn main() {
    let mut gb = gap_buffer::GapBuffer::new();
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                let key_code = c as i32;
                if key_code == 27 {
                    break;
                } else {
                    gb.insert_char(c);
                }
            }
            Some(Input::KeyBackspace) => {
                gb.remove_char();
            },
            Some(Input::KeyLeft) => {
                gb.shift_gap_backward();
            },
            Some(Input::KeyRight) => {
                gb.shift_gap_forward();
            },
            Some(_input) => {
                //println!("something else {:?}",input)
                ()
            }
            None => (),
        }
        window.clear();
        window.addstr(&gb.to_string());
        window.mv(0,gb.gap_start as i32);
    }
    endwin();
}

#[cfg(test)]
mod test;
