use super::*;
use pancurses;

pub fn render(window: &pancurses::Window, gb: &mut gap_buffer::GapBuffer) {
    let text = gb.to_string();
    let lines: Vec<&str> = text.split('\n').collect();
    window.clear();
    let mut char_count: usize = 0;
    let mut cursor_pos = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        if gb.gap_start >= char_count && gb.gap_start <= char_count + line.len() {
            cursor_pos.0 = i;
            cursor_pos.1 = gb.gap_start.wrapping_sub(char_count);
        }
        window.mv(i as i32, 0);
        window.addstr(line);
        char_count += line.len() + 1;
    }
    let remaining_row_count = window.get_max_y() - lines.len() as i32;
    for i in 0..remaining_row_count {
        window.mv(lines.len() as i32 + i, 0);
        window.addstr("~");
    }
    window.mv(cursor_pos.0 as i32, cursor_pos.1 as i32);
}
