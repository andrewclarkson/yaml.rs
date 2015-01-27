//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

use std::iter::Iterator;
use std::option::Option;


pub struct Scanner<'a> {
    reader: Box<Reader + 'a>,
}

impl <'a>Scanner<'a> {
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Scanner<'a> {
        Scanner { reader: reader } 
    }
}

#[derive(PartialEq, Copy, Show)]
pub enum Token {
    Character,
    Other,
}

impl <'a>Iterator for Scanner<'a> {
    
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        Option::Some(Token::Character)
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
        for token in scanner.take(5) {
            assert!(token == Token::Character);           
        }
    }
}
