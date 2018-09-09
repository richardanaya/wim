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
