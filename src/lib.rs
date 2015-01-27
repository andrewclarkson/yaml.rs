//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

use std::iter::Iterator;
use std::option::Option;


pub struct Scanner<'a> {
    reader: Box<Reader + 'a>,
    done: bool,
}

impl <'a>Scanner<'a> {
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Scanner<'a> {
        Scanner { reader: reader, done: false } 
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        if self.done {
            return None;
        }
        
        let token = match self.reader.read_byte() {
            Err(error) => {
                self.done = true;
                Token::Eof
            },
            Ok(byte) => {
               Token::Character
            }
        };
        Some(token)
    }
}

#[derive(PartialEq, Copy, Show)]
pub enum Token {
    Character,
    Eof,
}

impl <'a>Iterator for Scanner<'a> {
    
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.get_next_token()
    }
}


mod test {
    
    use std::io::MemReader;
    use std::vec;

    use super::{Scanner, Token};

    #[test]
    fn test_scanner() {
        let stream = "---";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut scanner = Scanner::new(Box::new(reader));
        let tokens: Vec<Token> = scanner.collect();
        let expected = vec!(Token::Character, Token::Character, Token::Character, Token::Eof);
        assert!(tokens == expected);
    }
}
