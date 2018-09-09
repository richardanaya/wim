use super::*;
use pancurses;

pub fn render(window: &pancurses::Window, gb: &mut gap_buffer::GapBuffer) {
    let text = gb.to_string();
    let lines: Vec<&str> = text.split('\n').collect();
    window.clear();
    for (i, line) in lines.iter().enumerate() {
        window.mv(i as i32, 0);
        window.addstr(line);
    }
    let remaining_row_count = window.get_max_y() - lines.len() as i32;
    for i in 0..remaining_row_count {
        window.mv(lines.len() as i32 + i, 0);
        window.addstr("~");
    }
    window.mv(0, gb.gap_start as i32);
}
