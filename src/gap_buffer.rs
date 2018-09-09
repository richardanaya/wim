#[derive(Debug)]
pub struct GapBuffer {
    pub data: Vec<char>,
    pub gap_start: usize,
    pub gap_end: usize,
}

impl GapBuffer {
    pub fn new() -> GapBuffer {
        GapBuffer {
            data: vec![],
            gap_start: 0,
            gap_end: 0,
        }
    }

    fn resize(&mut self, required_size: usize) {
        let mut len = self.data.len();
        let end_count = len - self.gap_end;

        if required_size <= 2 {
            self.data.resize(required_size, ' ');
            if required_size == 2 && end_count == 1 {
                self.data[1] = self.data[0]
            }
            self.gap_end = required_size - end_count;
        } else {
            if required_size > len {
                //if we exceed size double size of vector
                len *= 2;
                self.data.resize(len, ' ');
                //if we grow our size, move ending characters to new end
                if end_count > 0 {
                    for i in 0..end_count {
                        self.data[len - end_count + i] = self.data[self.gap_end + i];
                    }
                }
            }
            self.gap_end = len - end_count;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let front_count = self.gap_start + 1;
        let end_count = self.data.len() - self.gap_end;
        self.resize(front_count + end_count);
        self.data[self.gap_start] = c;
        self.gap_start = front_count;
    }

    pub fn remove_char(&mut self) {
        //if gap is at start do nothing
        if self.gap_start == 0 {
            return;
        }
        let front_count = self.gap_start - 1;
        let end_count = self.data.len() - self.gap_end;
        self.resize(front_count + end_count);
        self.gap_start = front_count;
    }

    pub fn shift_gap_backward(&mut self) {
        //if gap is already at the beginning do nothing
        if self.gap_start == 0 {
            return;
        }
        self.gap_start -= 1;
        self.gap_end -= 1;
        self.data[self.gap_end] = self.data[self.gap_start];
    }

    pub fn shift_gap_forward(&mut self) {
        //if we are at the end, do nothing
        if self.gap_end == self.data.len() {
            return;
        }
        self.gap_start += 1;
        self.gap_end += 1;
        if self.gap_end != self.data.len() {
            self.data[self.gap_start] = self.data[self.gap_end];
        }
    }

    pub fn to_string(&self) -> String {
        //combine front and back characters into a string
        let mut front: String = self.data[..self.gap_start].into_iter().collect();
        let back: String = self.data[self.gap_end..].into_iter().collect();
        println!("{:?}", front);
        println!("{:?}", back);
        front.push_str(&back);
        front
    }
}
