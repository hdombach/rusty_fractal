use std::{path::PathBuf, mem::size_of};

use crate::util::error::Error;

use super::{mesh::Mesh, resource_file::load_file_raw_vec};

type ParserIterator<'a> = std::slice::Iter<'a, u8>;

pub unsafe fn slice_from_ptr<'a>(begin: *const u8, end: *const u8) -> &'a [u8] {
    let len = end.offset_from(begin) as usize;
    let slice = std::slice::from_raw_parts(begin, len);
    return slice;
}

pub unsafe fn str_from_ptr<'a>(begin: *const u8, end: *const u8) -> &'a str {
    let slice = slice_from_ptr(begin, end);
    let result = std::str::from_utf8_unchecked(slice);
    return result;
}

#[derive(Clone)]
pub struct TextParser<'a> {
    iterator: ParserIterator<'a>,
    current_line: usize,
    current_column: usize,
}

impl<'a> TextParser<'a> {
    pub fn create_from_str(value: &'a str) -> Self {
        Self {
            iterator: value.as_bytes().iter(),
            current_line: 0,
            current_column: 0,
        }
    }
    pub fn create_from_vec(value: &'a mut Vec<u8>) -> Self {
        
        Self {
            iterator: value.iter(),
            current_line: 0,
            current_column: 0,
        }
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.iterator.as_ref().as_ptr()
    }
    pub fn next_char(&mut self) -> Result<char, Error> {
        if let Some(letter) = self.iterator.next() {
            let letter = *letter as char;
            if letter == '\n' {
                self.current_line += 1;
                self.current_column = 0
            } else {
                self.current_column += 1;
            }
            return Ok(letter);
        } else {
            return Err(Error::parser_end_of_file());
        }
    }
    pub fn peek_char(&self) -> Result<char, Error> {
        self.clone().next_char()
    }
    pub fn next_line(&mut self) -> Result<&'a str, Error> {
        let start = self.get_ptr();
        while self.peek_char().unwrap_or('\n') != '\n' {
            _ = self.next_char();
        }
        let end = self.get_ptr();
        let eof = self.next_char().is_err();
        if eof && start == end {
            return Err(Error::parser_end_of_file());
        }
        unsafe { return Ok(str_from_ptr(start, end)); }
    }

    pub fn peek_line(&self) -> Result<&'a str, Error> {
        self.clone().next_line()
    }
    pub fn next_word(&mut self) -> Result<&'a str, Error> {
        while self.peek_char().unwrap_or('_').is_ascii_whitespace() {
            _ = self.next_char();
        }
        let start = self.get_ptr();
        let mut end = self.get_ptr();
        while !self.peek_char().unwrap_or('_').is_ascii_whitespace() {
            _ = self.next_char();
            end = self.get_ptr();
        }
        if start == end {
            return Err(Error::parser_end_of_file());
        } else {
            unsafe { return Ok(str_from_ptr(start, end)); }
        }
    }
    pub fn peek_word(&self) -> Result<&'a str, Error> {
        self.clone().next_word()
    }
    pub fn next_words(&mut self) -> Vec<&'a str> {
        let mut result:Vec<&'a str> = Vec::new();
        loop {
            loop {
                if self.peek_char().unwrap_or('\n') == '\n' {
                    _ = self.next_char();
                    return result
                } else if self.peek_char().unwrap().is_ascii_whitespace() {
                    _ = self.next_char();
                } else {
                    break
                }
            }
            let start = self.get_ptr();
            let mut end = self.get_ptr();

            while !self.peek_char().unwrap_or(' ').is_ascii_whitespace() {
                _ = self.next_char();
                end = self.get_ptr();
            }

            unsafe { result.push(str_from_ptr(start, end)); }
        }
    }
    pub fn next_ascii_uint32(&mut self) -> Result<u32, Error> {
        self.skip_whitespace();
        let mut result = 0;
        let mut is_empty = true;
        let start = self.get_ptr();
        loop {
            if let Ok(letter) = self.next_char() {
                if letter.is_ascii_whitespace()  {
                    return Ok(result);
                } else if letter.is_ascii_digit() {
                    let value = letter.to_digit(10).unwrap();
                    result = result * 10 + value;
                    is_empty = false;
                } else {
                    while !self.next_char().unwrap_or('\n').is_ascii_whitespace() {};
                    let received_word: &str;
                    unsafe { received_word = str_from_ptr(start, self.get_ptr().offset(-1)) };
                    let received_word = String::from(received_word);
                    return Err(Error::parser_invalid_ascii_int(received_word, self.current_line, self.current_column));
                }
            } else {
                if is_empty {
                    return Err(Error::parser_end_of_file());
                } else {
                    return Ok(result);
                }
            }
        }
    }
    //Skips a comment if it exists
    pub fn skip_single_line_comment(&mut self, comment_syntax: &str) {
        if self.require_str(comment_syntax).is_err() {return;}
        while self.next_char().unwrap_or('\n') != '\n' {}
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek_char().unwrap_or('_').is_ascii_whitespace() {
            _ = self.next_char();
        }
    }

    pub fn skip_line_white_space(&mut self) {
        loop {
            let peek = self.peek_char().unwrap_or('_');
            if peek.is_ascii_whitespace() && peek != '\\' {
                _ = self.next_char();
            } else {
                break
            }
        }
    }

    pub fn require_char(&mut self, expected_char: char) -> Result<(), Error> {
        let received_char = self.peek_char()?;
        if received_char == expected_char {
            _ = self.next_char();
            return Ok(());
        } else {
            return Err(Error::parser_unknown_char(expected_char, received_char, self.current_line, self.current_column));
        }
    }
    pub fn require_str(&mut self, expected_str: &str) -> Result<(), Error> {
        let mut temp_parser = self.clone();
        for c in expected_str.chars() {
            if temp_parser.require_char(c).is_err() {
                let start = self.get_ptr();
                let end = temp_parser.get_ptr();
                let received_str: &str;
                unsafe { received_str = str_from_ptr(start, end); }
                return Err(Error::parser_unknown_str(expected_str, received_str, self.current_line, self.current_column));
            }
        }
        self.clone_from(&temp_parser);
        Ok(())
    }
    pub fn require_word(&mut self, word: &str) -> Result<(), Error> {
        let mut temp_parser = self.clone();
        let received_word = temp_parser.next_word()?;
        if received_word == word {
            self.clone_from(&temp_parser);
            return Ok(())
        } else {
            return Err(Error::parser_unknown_word(
                    String::from(word),
                    String::from(received_word),
                    self.current_line,
                    self.current_column));
        }
    }
    pub fn require_words(&mut self, words: Vec<&str>) -> Result<(), Error> {
        let mut temp_parser = self.clone();
        for word in words {
            temp_parser.require_word(word)?;
        }
        self.clone_from(&temp_parser);
        Ok(())
    }
    pub fn require_line(&mut self, line: &str) -> Result<(), Error> {
        let mut temp_parser = self.clone();
        let received_line = temp_parser.next_line()?;
        if received_line == line {
            self.clone_from(&temp_parser);
            return Ok(())
        } else {
            return Err(Error::parser_unknown_line(
                    String::from(line),
                    String::from(received_line),
                    self.current_line));
        }
    }
    pub fn get_remaining_bytes(&self) -> usize {
        self.iterator.size_hint().0
    }
    pub fn print_remaining_bytes(&self) {
        println!("remaining size is {}", self.get_remaining_bytes());
    }
}

#[derive(Clone)]
pub struct RawParser<'a> {
    iterator: ParserIterator<'a>,
}
impl<'a> RawParser<'a> {
    pub fn new(iterator: ParserIterator<'a>) -> Self {
        Self {
            iterator,
        }
    }
    pub fn create_from_vec(value: &'a mut Vec<u8>) -> Self {
        Self {
            iterator: value.iter(),
        }
    }
    pub fn get_ptr(&self) -> *const u8 {
        self.iterator.as_ref().as_ptr()
    }
    pub fn next_raw_u8(&mut self) -> Result<u8, Error> {
        if let Some(value) = self.iterator.next() {
            return Ok(value.clone());
        } else {
            return Err(Error::parser_end_of_file());
        }
    }
    pub fn next_raw_u32(&mut self) -> Result<u32, Error> {
        let ptr = self.get_ptr();
        if let Some(_) = self.iterator.nth(3) {
            unsafe {
                let ptr = ptr as *const u32;
                Ok(*ptr)
            }
        } else {
            Err(Error::parser_end_of_file())
        }
    }
    pub fn next_slice<T>(&mut self, size: usize) -> Result<&'a [T], Error> {
        let ptr = self.get_ptr() as *const T;
        if self.iterator.nth(size * size_of::<T>() - 1).is_none() {
            return Err(Error::parser_end_of_file());
        }
        let result = unsafe { std::slice::from_raw_parts(ptr, size) };
        return Ok(result);
    }
    pub fn get_remaining_bytes(&self) -> usize {
        self.iterator.size_hint().0
    }
    pub fn print_remaining_bytes(&self) {
        println!("remaining size is {}", self.get_remaining_bytes());
    }
}
impl<'a> From<&'a TextParser<'a>> for RawParser<'a> {
    fn from(parser: &'a TextParser) -> Self {
        Self {
            iterator: parser.iterator.clone()
        }
    }
}

pub fn parse_mesh(dir: PathBuf, gl: &glow::Context, name: &str) -> Result<Mesh, Error> {
    let mut data = load_file_raw_vec(dir)?;
    let mut parser = TextParser::create_from_vec(&mut data);
    parser.print_remaining_bytes();

    parser.require_line("ply")?;
    parser.require_line("format binary_little_endian 1.0")?;
    parser.skip_single_line_comment("comment");
    parser.skip_single_line_comment("comment");

    parser.require_words(vec!["element", "vertex"])?;
    let vertex_count = parser.next_ascii_uint32()?;

    parser.require_line("property float x")?;
    parser.require_line("property float y")?;
    parser.require_line("property float z")?;
    parser.require_line("property float nx")?;
    parser.require_line("property float ny")?;
    parser.require_line("property float nz")?;

    parser.require_words(vec!["element", "face"])?;
    let face_count = parser.next_ascii_uint32()?;

    parser.require_line("property list uchar uint vertex_indices")?;
    parser.require_line("end_header")?;
    

    let mut parser = RawParser::from(&parser);


    let vertex_data_count = (vertex_count * 6) as usize;
    let vertex_data: &[f32] = parser.next_slice(vertex_data_count)?;

    for _ in 0..1000 {
        //println!("byte {}", parser.next_raw_u8().unwrap_or(255));
    }


    let face_data_count = (face_count * 3) as usize;
    let mut face_data: Vec<u32> = Vec::with_capacity(face_data_count);
    for _ in 0..face_count {
        let count = parser.next_raw_u8()?;
        if count != 3 {
            return Err(Error::ply_parser(format!("Vertex count {} for face is not implimented", count)));
        }
        for _ in 0..count {
            let next_face_thing = parser.next_raw_u32()?;
            face_data.push(next_face_thing);
        }
    }

    //println!("face indexes: {:?}", face_data);
    //println!("vertex data: {:?}", vertex_data);

    return Mesh::create_indexed(vertex_data.to_vec(), face_data, gl, super::mesh::VertexShader::default_simple_with_normal(), name);
}

#[cfg(test)]
mod tests {
    use super::TextParser;

    #[test]
    fn get_line() {
        let text = "this is line one\nthis is line two\n\nhi";
        let mut parser = TextParser::create_from_str(text);
        assert_eq!(parser.next_line(), Ok("this is line one"));
        assert_eq!(parser.next_line(), Ok("this is line two"));
        assert_eq!(parser.next_line(), Ok(""));
        assert_eq!(parser.next_line(), Ok("hi"));
        assert!(parser.next_line().is_err());
    }

    #[test]
    fn get_word() {
        let text = "word\ntwo\tis going";
        let mut parser = TextParser::create_from_str(text);
        assert_eq!(parser.next_word(), Ok("word"));
        assert_eq!(parser.next_word(), Ok("two"));
        assert_eq!(parser.peek_word(), Ok("is"));
        assert_eq!(parser.next_word(), Ok("is"));
        assert_eq!(parser.next_word(), Ok("going"));
    }

    #[test]
    fn get_words() {
        let empty: Vec<String> = Vec::new();
        let text = "this is line one\nthis is line\n\ntwo";
        let mut parser = TextParser::create_from_str(text);
        assert_eq!(parser.next_words(), vec![String::from("this"), String::from("is"), String::from("line"), String::from("one")]);
        assert_eq!(parser.next_words(), vec![String::from("this"), String::from("is"), String::from("line")]);
        assert_eq!(parser.next_words(), empty);
        assert_eq!(parser.next_words(), vec![String::from("two")]);
    }

    #[test]
    fn require_word() {
        let text = "this is\na collection of\twords";
        let mut parser = TextParser::create_from_str(text);
        assert!(parser.require_word("incorrect").is_err());
        assert_eq!(parser.require_word("this"), Ok(()));
        assert!(parser.require_word("this").is_err());
    }

    #[test]
    fn require_words() {
        let text = "one two three four five size";
        let mut parser = TextParser::create_from_str(text);
        assert!(parser.require_words(vec!["one", "twoo"]).is_err());
        assert!(parser.require_words(vec!["one", "two", "three"]).is_ok());
        assert!(parser.require_words(vec!["one"]).is_err());
        assert!(parser.require_words(vec!["one", "two", "three", "four"]).is_err());
        assert!(parser.require_words(vec!["three", "four"]).is_err());
        assert!(parser.require_words(vec!["four", "five"]).is_ok());


    }

    #[test]
    fn next_ascii_uint32() {
        let text = "892 word 4";
        let mut parser = TextParser::create_from_str(text);
        assert_eq!(parser.next_ascii_uint32(), Ok(892));
        assert!(parser.next_ascii_uint32().is_err());
        assert_eq!(parser.next_ascii_uint32(), Ok(4));
        assert!(parser.next_ascii_uint32().is_err());

    }

    #[test]
    fn require_strs() {
        let text = "testing 1 2 3";
        let mut parser = TextParser::create_from_str(text);
        assert!(parser.require_str("tester").is_err());
        assert!(parser.require_str("test").is_ok());
        assert!(parser.require_str("ing").is_ok());
        assert!(parser.require_str("ing").is_err());
        assert!(parser.require_str(" 1 2 3").is_ok());
    }

    #[test]
    fn skip_comments() {
        let text = "comment ply version\n#python version\n//rust version";
        let mut parser = TextParser::create_from_str(text);

        parser.skip_single_line_comment("#");
        assert_eq!(parser.peek_line(), Ok("comment ply version"));
        parser.skip_single_line_comment("comment");
        assert_eq!(parser.peek_line(), Ok("#python version"));

        parser.skip_single_line_comment("#");
        assert_eq!(parser.peek_line(), Ok("//rust version"));
    }
}
