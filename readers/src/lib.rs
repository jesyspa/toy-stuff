use std::io::BufRead;
use std::marker::PhantomData;

const NEWLINE: u8 = 0x0A;
const SPACE: u8 = 0x20;
const MINUS: u8 = 0x2d;
const DIGIT_0: u8 = 0x30;
const DIGIT_9: u8 = 0x39;

fn digit_to_num(c: u8) -> Option<u64> {
    if DIGIT_0 <= c && c <= DIGIT_9 {
        Some((c - DIGIT_0) as u64)
    } else { None }
}

fn is_whitespace(c: u8) -> bool {
    c == NEWLINE || c == SPACE
}

// f returns true if it consumed its input.
fn process_buf<R: BufRead, F>(reader: &mut R, mut f: F)
where F: FnMut(u8) -> bool {
    loop {
        let buf = reader.fill_buf().unwrap();
        if buf.is_empty() {
            return;
        }
        for i in 0..buf.len() {
            if !f(buf[i]) {
                drop(buf);
                reader.consume(i);
                return;
            }
        }
        let read = buf.len();
        drop(buf);
        reader.consume(read);
    }
}

pub trait InPlaceRead {
    fn read_in_place<R: BufRead>(reader: &mut R) -> Self;
}

impl InPlaceRead for u64 {
    fn read_in_place<R: BufRead>(reader: &mut R) -> u64 {
        let mut r = 0;
        process_buf(reader, |c| {
            if let Some(k) = digit_to_num(c) {
                r *= 10;
                r += k;
                true
            } else {
                false
            }
        });
        r
    }
}

impl InPlaceRead for i64 {
    fn read_in_place<R: BufRead>(reader: &mut R) -> i64 {
        let buf = reader.fill_buf().unwrap();
        let sign;
        if buf[0] == MINUS {
            drop(buf);
            reader.consume(1);
            sign = -1;
        } else {
            sign = 1;
        }
        u64::read_in_place(reader) as i64 * sign
    }
}

impl InPlaceRead for String {
    fn read_in_place<R: BufRead>(reader: &mut R) -> String {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        if !s.is_empty() && s.bytes().last().unwrap() == NEWLINE {
            s.pop();
        }
        s
    }
}

fn drop_whitespace<R: BufRead>(reader: &mut R) {
    process_buf(reader, is_whitespace);
}

fn drop_spaces<R: BufRead>(reader: &mut R) {
    process_buf(reader, |c| c == SPACE);
}

fn is_eol<R: BufRead>(reader: &mut R) -> bool{
    let buf = reader.fill_buf().unwrap();
    buf.is_empty() || buf[0] == NEWLINE
}

pub struct ProblemLineReader<'a, R: BufRead, T>(&'a mut R, PhantomData<T>);

impl<'a, R: BufRead, T: InPlaceRead> Iterator for ProblemLineReader<'a, R, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        drop_spaces(self.0);
        if is_eol(self.0) {
            None
        } else {
            Some(T::read_in_place(self.0))
        }
    }
}

pub struct ProblemCountReader<'a, R: BufRead, T>(&'a mut R, usize, PhantomData<T>);

impl<'a, R: BufRead, T: InPlaceRead> Iterator for ProblemCountReader<'a, R, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.1 > 0 {
            drop_whitespace(self.0);
            self.1 -= 1;
            Some(T::read_in_place(self.0))
        } else {
            None
        }
    }
}

pub struct ProblemReader<R: BufRead, S>(R, S);

impl<R: BufRead, S> ProblemReader<R, S> {
    #[allow(dead_code)]
    pub fn read<T: InPlaceRead>(&mut self) -> T {
        drop_whitespace(&mut self.0);
        T::read_in_place(&mut self.0)
    }

    // Warning: not usable with String.
    #[allow(dead_code)]
    pub fn read_line<T: InPlaceRead>(&mut self) -> ProblemLineReader<R, T> {
        drop_whitespace(&mut self.0);
        ProblemLineReader(&mut self.0, PhantomData)
    }

    #[allow(dead_code)]
    pub fn read_count<T: InPlaceRead>(&mut self, n: usize) -> ProblemCountReader<R, T> {
        ProblemCountReader(&mut self.0, n, PhantomData)
    }
}

use std::io::BufReader;

impl ProblemReader<BufReader<&[u8]>, String> {
    #[allow(dead_code)]
    fn from_string(s: String) -> Self {
        ProblemReader(BufReader::new(s.as_bytes()), s)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};
    use super::{InPlaceRead, drop_whitespace, ProblemReader};

    #[test]
    fn read_unsigned_test() {
        let s = "1234 4321".as_bytes();
        let mut reader = BufReader::new(s);
        assert_eq!(u64::read_in_place(&mut reader), 1234);
        reader.fill_buf().unwrap();
        reader.consume(1);
        assert_eq!(u64::read_in_place(&mut reader), 4321);
    }

    #[test]
    fn read_signed_test() {
        let s = "-1234 4321 -999".as_bytes();
        let mut reader = BufReader::new(s);
        assert_eq!(i64::read_in_place(&mut reader), -1234);
        reader.fill_buf().unwrap();
        reader.consume(1);
        assert_eq!(i64::read_in_place(&mut reader), 4321);
        reader.fill_buf().unwrap();
        reader.consume(1);
        assert_eq!(i64::read_in_place(&mut reader), -999);
    }

    #[test]
    fn read_string_test() {
        let s = "FOO BAR\nBAZ".as_bytes();
        let mut reader = BufReader::new(s);
        assert_eq!(String::read_in_place(&mut reader), "FOO BAR");
        // no need to skip whitespace
        assert_eq!(String::read_in_place(&mut reader), "BAZ");
    }

    #[test]
    fn drop_whitespace_test() {
        let s = "  -1234 4321 999  ".as_bytes();
        let mut reader = BufReader::new(s);
        drop_whitespace(&mut reader);
        assert_eq!(i64::read_in_place(&mut reader), -1234);
        drop_whitespace(&mut reader);
        assert_eq!(i64::read_in_place(&mut reader), 4321);
        drop_whitespace(&mut reader);
        assert_eq!(u64::read_in_place(&mut reader), 999);
        drop_whitespace(&mut reader);
    }

    /*
    #[test]
    fn problem_reader_test() {
        let s = "  -1234 4321 999  ".as_bytes();
        let mut reader = BufReader::new(s);
        let mut problem = ProblemReader(&mut reader);
        assert_eq!(problem.read::<i64>(), -1234);
        assert_eq!(problem.read::<i64>(), 4321);
        assert_eq!(problem.read::<u64>(), 999);
    }

    #[test]
    fn problem_line_reader_test() {
        let s = "-1234 4321\n999".as_bytes();
        let mut reader = BufReader::new(s);
        let mut problem = ProblemReader(&mut reader);
        assert_eq!(problem.read_line::<i64>().collect::<Vec<_>>(),
                   vec![-1234, 4321]);
        assert_eq!(problem.read::<u64>(), 999);
    }

    #[test]
    fn problem_count_reader_test() {
        let s = "-1234 4321\n999 111".as_bytes();
        let mut reader = BufReader::new(s);
        let mut problem = ProblemReader(&mut reader);
        assert_eq!(problem.read_count::<i64>(3).collect::<Vec<_>>(),
                   vec![-1234, 4321, 999]);
        assert_eq!(problem.read::<u64>(), 111);
    }
    */
}
