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

    fn resize(&mut self, new_len: usize) {
        println!("resize to {}",new_len);
        let mut l = self.data.len();
        if new_len <= 2 {
            self.data.resize(new_len, ' ');
            self.gap_end = new_len;
        } else {
            if new_len > l {
                l *= 2;
            }
            self.data.resize(l, ' ');
            self.gap_end = l;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let l = self.gap_start;
        self.resize(l + 1);
        self.data[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn remove_char(&mut self) {
        if self.gap_start == 0 {
            return;
        }
        
        let mut l = self.gap_start-1;
        if self.gap_start >= 0 {
            self.resize(l);
            self.gap_start = l;
        }
    }

    pub fn to_string(&self) -> String {
        return self.data[..self.gap_start].into_iter().collect();
    }
}
