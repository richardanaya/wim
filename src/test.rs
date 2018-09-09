use super::*;

#[test]
fn basic_creation() {
    let gb = gap_buffer::GapBuffer::new();
    assert_eq!(gb.data.len(), 0);
    assert_eq!(gb.gap_start, 0);
    assert_eq!(gb.gap_end, 0);
}

#[test]
fn basic_insert1() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data.len(), 1);
    assert_eq!(gb.gap_start, 1);
    assert_eq!(gb.gap_end, 1);
}

#[test]
fn basic_insert2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data.len(), 2);
    assert_eq!(gb.gap_start, 2);
    assert_eq!(gb.gap_end, 2);
}

#[test]
fn basic_insert3() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 3);
    assert_eq!(gb.gap_end, 4);
}

#[test]
fn basic_to_string() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_to_string2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    assert_eq!(gb.to_string(), "ab");
}

#[test]
fn basic_remove() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.insert_char('d');
    gb.remove_char();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 3);
    assert_eq!(gb.gap_end, 4);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_remove2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.insert_char('d');
    gb.remove_char();
    gb.remove_char();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data.len(), 2);
    assert_eq!(gb.gap_start, 2);
    assert_eq!(gb.gap_end, 2);
    assert_eq!(gb.to_string(), "ab");
}

#[test]
fn basic_remove_empty() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.remove_char();
    assert_eq!(gb.data.len(), 0);
    assert_eq!(gb.gap_start, 0);
    assert_eq!(gb.gap_end, 0);
    assert_eq!(gb.to_string(), "");
}

#[test]
fn basic_beyond_empty() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.remove_char();
    gb.remove_char();
    assert_eq!(gb.data.len(), 0);
    assert_eq!(gb.gap_start, 0);
    assert_eq!(gb.gap_end, 0);
    assert_eq!(gb.to_string(), "");
}

#[test]
fn basic_move_left() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[3], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 2);
    assert_eq!(gb.gap_end, 3);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_move_backward_2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[2], 'b');
    assert_eq!(gb.data[3], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 1);
    assert_eq!(gb.gap_end, 2);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_move_backward_beyond() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    assert_eq!(gb.data[1], 'a');
    assert_eq!(gb.data[2], 'b');
    assert_eq!(gb.data[3], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 0);
    assert_eq!(gb.gap_end, 1);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_move_forward() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.shift_gap_forward();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[3], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 2);
    assert_eq!(gb.gap_end, 3);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_move_forward_beyond() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.shift_gap_forward();
    gb.shift_gap_forward();
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 3);
    assert_eq!(gb.gap_end, 4);
    assert_eq!(gb.to_string(), "abc");
}

#[test]
fn basic_middle_insert() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.insert_char('d');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'd');
    assert_eq!(gb.data[3], 'c');
    assert_eq!(gb.data.len(), 4);
    assert_eq!(gb.gap_start, 3);
    assert_eq!(gb.gap_end, 3);
    assert_eq!(gb.to_string(), "abdc");
}

#[test]
fn basic_middle_insert_too_long() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.insert_char('d');
    gb.insert_char('e');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'd');
    assert_eq!(gb.data[3], 'e');
    assert_eq!(gb.data[7], 'c');
    assert_eq!(gb.data.len(), 8);
    assert_eq!(gb.gap_start, 4);
    assert_eq!(gb.gap_end, 7);
    assert_eq!(gb.to_string(), "abdec");
}

#[test]
fn basic_middle_insert_too_long2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.insert_char('d');
    gb.insert_char('e');
    gb.shift_gap_backward();
    gb.insert_char('f');
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data[1], 'b');
    assert_eq!(gb.data[2], 'd');
    assert_eq!(gb.data[3], 'f');
    assert_eq!(gb.data[6], 'e');
    assert_eq!(gb.data[7], 'c');
    assert_eq!(gb.data.len(), 8);
    assert_eq!(gb.gap_start, 4);
    assert_eq!(gb.gap_end, 6);
    assert_eq!(gb.to_string(), "abdfec");
}

#[test]
fn basic_repeat() {
    let mut gb = gap_buffer::GapBuffer::new();
    for _i in 0..100 {
        gb.insert_char('a');
    }
    assert_eq!('a', gb.data[99]);
    assert_eq!(' ', gb.data[100]);
    assert_eq!(128, gb.data.len());
}

#[test]
fn basic_insert_shift() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.shift_gap_backward();
    println!("{:?}", gb);
    assert_eq!(gb.data[0], 'a');
    assert_eq!(gb.data.len(), 1);
    assert_eq!(gb.gap_start, 0);
    assert_eq!(gb.gap_end, 0);
    assert_eq!(gb.to_string(), "a");
}

#[test]
fn basic_insert_shift_insert() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.shift_gap_backward();
    gb.insert_char('b');
    assert_eq!(gb.data[0], 'b');
    assert_eq!(gb.data[1], 'a');
    assert_eq!(gb.data.len(), 2);
    assert_eq!(gb.gap_start, 1);
    assert_eq!(gb.gap_end, 1);
    assert_eq!(gb.to_string(), "ba");
}

#[test]
fn complex() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.shift_gap_backward();
    gb.insert_char('b');
    gb.insert_char('c');
    assert_eq!(gb.to_string(), "bca");
}

#[test]
fn complex2() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.shift_gap_backward();
    gb.insert_char('b');
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    gb.insert_char('c');
    assert_eq!(gb.to_string(), "cba");
}

#[test]
fn complex3() {
    let mut gb = gap_buffer::GapBuffer::new();
    gb.insert_char('a');
    gb.insert_char('b');
    gb.insert_char('c');
    gb.shift_gap_backward();
    gb.shift_gap_backward();
    println!("{:?}",gb);
    gb.shift_gap_forward();
    println!("{:?}",gb);
    gb.shift_gap_forward();
    println!("{:?}",gb);
    assert_eq!(gb.to_string(), "abc");
}
