use super::*;
use std::io::*;

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct TextDocument<'a> {
    pub filepath: &'a Path,
    pub gap_buffer: gap_buffer::GapBuffer,
}

impl<'a> TextDocument<'a> {
    pub fn create_or_open(filepath: &'a Path) -> TextDocument<'a> {
        let exists = filepath.exists();
        let mut initial_text = String::new();
        if exists {
            let mut file = File::open(filepath).unwrap();
            file.read_to_string(&mut initial_text).unwrap();
        }
        TextDocument {
            filepath: &filepath,
            gap_buffer: gap_buffer::GapBuffer::new_from_string(initial_text),
        }
    }

    pub fn save(&mut self) {
        let mut file = File::create(self.filepath).unwrap();
        file.write_all(self.gap_buffer.to_string().as_bytes())
            .unwrap();
    }

    pub fn add_char(&mut self, c: char) {
        self.gap_buffer.insert_char(c);
    }

    pub fn remove_char(&mut self) {
        self.gap_buffer.remove_char();
    }

    pub fn move_cursor(&mut self, dir: CursorDirection) {
        match dir {
            CursorDirection::Left => self.gap_buffer.shift_gap_backward(),
            CursorDirection::Right => self.gap_buffer.shift_gap_forward(),
            CursorDirection::Up => self.gap_buffer.shift_gap_backward_by(10),
            CursorDirection::Down => self.gap_buffer.shift_gap_forward_by(10),
        }
    }
}
