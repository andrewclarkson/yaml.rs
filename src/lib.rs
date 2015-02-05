//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

use std::iter::Iterator;
use std::option::Option;
use std::io::{IoResult, MemReader};
use std::borrow::Cow;
use self::Token::*;

/// A Yaml Tokenizer
///
/// A tokenizer takes a byte stream and produces a stream of Tokens
/// This shelters the parser from the complexity of dealing with raw bytes.
///
pub struct Tokenizer<'a> {
    reader: Box<Reader + 'a>,
    stack: Vec<u8>,
}

impl <'a>Tokenizer<'a> {
    
    /// Creates a new Tokenizer from a Reader
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Tokenizer<'a> {
        Tokenizer { 
            reader: reader, 
            stack: Vec::new(), 
        } 
    }

    /// Reads a byte from the reader an returns it as a char
    /// TODO: Error Handling
    fn read_character(&mut self) -> Option<char> {
        match self.reader.read_byte() {
            Ok(byte) => Some(byte as char),
            Err(_) => None,
        }
    }

    /// Pops a character off of the internal buffer (stack)
    /// or if the stack is empty it reads a new character
    fn pop(&mut self) -> Option<char> {
        // if we have consumed all of the characters
        // in the buffer, just read one
        if(self.stack.len() == 0) {
            self.read_character()

        // if we have characters that something has puked back,
        // use those.
        } else {
            match self.stack.pop() {
                Some(byte) => Some(byte as char),
                None => None,
            }
        }
    }

    /// Gets the next token, useful for iterating
    /// TODO: Error handling
    pub fn next_token(&mut self) -> Option<Token> {
        
        match self.pop() {
            Some(character) => match character {
                // TODO: factor out Some(Token)
                '[' => Some(SequenceStart),
                ']' => Some(SequenceEnd),
                '{' => Some(MappingStart),
                '}' => Some(MappingEnd),
                '-' => {
                    if self.consume("--") {
                        Some(DocumentStart)
                    } else {
                        Some(SequenceEntry)  
                    }
                },
                ':' => Some(MappingSeparator),
                ',' => Some(CollectionSeparator),
                '?' => Some(ComplexKey),
                '!' => Some(Tag),
                '&' => {
                    match self.consume_anchor() {
                        Some(string) => Some(Anchor(string)),
                        //TODO: should this be an error?
                        None => Anchor(String::new()),
                    }
                },
                '*' => {
                    match self.consume_anchor() {
                        Some(string) => Some(Alias(string)),
                        //TODO: should this be an error?
                        None => Alias(String::new()),
                    }
                },
                '#' => {
                    self.throw_away_comment();
                    self.next_token()
                },
                '\'' => Some(SingleQuote),
                '"' => Some(DoubleQuote),
                '|' => Some(Literal),
                '>' => Some(Folded),
                '.' => {
                    // Attempt to consume '..'
                    if self.consume("..") {
                        Some(DocumentEnd)
                    } else {
                        // Puke the '.' and try again as a scalar
                        self.stack.push(character as u8);
                        self.consume_scalar()
                    }
                },
                '%' => self.consume_directive(),
                // TODO: do more with whitespace than just
                // throw it out!
                '\n' | ' ' | '\t' => {
                    self.throw_away_whitespace();
                    self.next_token()
                }
                _ => {
                    self.stack.push(character as u8);
                    self.consume_scalar()
                },
            },
            None => None,
        }
    }

    fn throw_away_whitespace(&mut self) {
        loop {
            match self.pop() {
                Some(character) => match character {
                    '\n' | ' ' | '\t' => { continue },
                    // If we found a non-whitespace character,
                    // put it back on the stack
                    _ => { 
                        //TODO: factor out "as u8" on the push
                        self.stack.push(character as u8);
                        break;
                    },
                },
                None => { 
                    break;
                }
            }
        }
        
    }

    fn throw_away_comment(&mut self) {
        loop {
            match self.pop() {
                Some(character) => match character {
                    '\n' => { break },
                    _ => { continue },
                },
                None => { 
                    break;
                }
            }
        }
    }

    fn consume_directive(&mut self) -> Option<Token> {
        if self.consume("YAML ") {
            match self.consume_yaml_version() {
                Some(version) => Some(YamlDirective(version)),
                None => None,
            }

        // TODO: Other types of directives
        } else {
            None
        }
    }

    fn consume_yaml_version(&mut self) -> Option<YamlVersion> {
        let major = match self.consume_number() {
            Some(number) => number,
            None => { return None },
        };
        if !self.consume(".") {
            return None;        
        }
        let minor = match self.consume_number() {
            Some(number) => number,
            None => { return None },
        };
        Some(YamlVersion {major: major, minor: minor})
    }

    fn consume_number(&mut self) -> Option<usize> {
        let mut accumulator: Option<usize> = None;
        loop {
            match self.pop() {
                Some(character) => match character {
                    '0' ... '9' => {
                        let digit: usize = character as usize - '0' as usize;
                        match accumulator {
                            Some(mut a) => {
                                a *= 10;
                                a += digit;
                            }
                            None => {
                                accumulator = Some(digit);
                            }
                        }
                    },
                    _ => {
                        self.stack.push(character as u8);
                        break;
                    }
                },
                None => { return None },
            }
        }
        accumulator
    }

    fn consume_anchor(&mut self) -> Option<String> {
        let mut eaten = Vec::new();
        loop {
            match self.pop() {
                Some(character) => match character {
                    'A'...'Z'|'a'...'z'|'0'...'9'|'-'|'_' => {
                        eaten.push(character as u8);
                    },
                    _ => {
                        self.stack.push(character as u8);
                        break;
                    },
                },
                None => { 
                    break;
                }
            }
        }
        
        match String::from_utf8(eaten) {
            Ok(string) => Some(string),
            Err(_) => None,
        }
           
    }

    fn consume_scalar(&mut self) -> Option<Token> {
        let mut eaten = Vec::new();
        loop {
            match self.pop() {
                Some(character) => match character {
                    'A'...'Z'|'a'...'z'|'0'...'9' => {
                        eaten.push(character as u8);
                    },
                    _ => {
                        self.stack.push(character as u8);
                        break;
                    },
                },
                None => { 
                    break;
                }
            }
        }

        match String::from_utf8(eaten) {
            Ok(string) => Some(Scalar(string)),
            Err(_) => None,
        }
    }

    fn consume(&mut self, edible: &'static str) -> bool {
        let mut eaten = Vec::new();
        for expected in edible.chars() {
            let character = self.pop();
            if character.is_some() {
                let c = character.unwrap();
                eaten.push(c as u8);
                if c == expected {
                    continue;
                }
            }
            for ate in eaten.drain().rev() {
                self.stack.push(ate);
            } 
            return false;
        }
        true
    }
}


/// All the types of tokens
///
/// Describes all of the token types described by the [Yaml 1.0 Spec](http://yaml.org/spec/1.0/)
///
#[derive(PartialEq, Debug)]
pub enum Token {

    /// `[`: the start of a _flow_ sequence
    /// `c-sequence-start` in the spec
    SequenceStart,

    /// `]`: the end of a _flow_ sequence
    /// `c-sequence-end` in the spec
    SequenceEnd,

    /// `{`: the end of a _flow_ mapping
    /// `c-mapping-start` in the spec
    MappingStart,

    /// `}`: the end of a _flow_ mapping
    /// `c-mapping-end` in the spec
    MappingEnd,

    /// `-`: indicates an entry in a sequence
    /// `c-sequence-entry` in the spec
    SequenceEntry,

    /// `:`: separates key from value in a mapping
    /// `c-mapping-entry` in the spec
    MappingSeparator,
    
    /// `,`: separates entries in _flow_ collections (mapping or sequence)
    /// `c-collect-entry` in the spec
    CollectionSeparator,

    /// `?`: indicates a complex key (e.g a non-scalar key)
    /// `c-complex-key` in the spec
    ComplexKey,

    /// `!`: indicates a tag property
    /// `c-tag` in the spec
    Tag,

    /// `&`: indicates an anchor property (e.g setting a variable)
    /// `c-anchor` in the spec
    Anchor(String),

    /// `*`: indicates an anchor property (e.g using a variable)
    /// `c-alias` in the spec
    Alias(String),

    /// `|`: indicates a literal value (not to be escaped)
    /// `c-literal` in the spec
    Literal,

    /// `>`: indicates a folded value (ignore line breaks)
    /// `c-folded` in the spec
    Folded,

    /// `'`: a single quoted scalar
    /// `c-single-quote` in the spec
    SingleQuote,
    
    /// `"`: a double quoted scalar
    /// `c-double-quote` in the spec
    DoubleQuote,

    /// `#`: a comment
    /// `c-throwaway`
    Comment,

    /// `%YAML <version>`: a yaml version directive (tags for the YAML parser)
    /// `c-directive` in the spec
    YamlDirective(YamlVersion),

    /// TODO: `%<NAME> <param>*`: a directive yet to be implemented
    /// ``
    /// ReservedDirective(String, Vec<String>)

    /// `@ | \``: reserved for future use?
    /// `c-reserved` in the spec
    Reserved,

    ///TODO Documentation
    ///
    DocumentStart,

    ///TODO: Documentation
    ///
    DocumentEnd,

    /// ``: a line break with no specific purpose
    LineBreakGeneric,

    /// ``
    LineBreak(LineBreakType),

    Other, // A temporary addition for unimplemented tokens
    Scalar(String),
    Eof,
}

#[derive(PartialEq, Eq, Debug)]
pub struct YamlVersion {
    major: usize,
    minor: usize,
}

#[derive(PartialEq, Copy, Debug)]
pub enum LineBreakType {
    Generic,
    LineSeparator,
    ParagraphSeparator
}

impl <'a>Iterator for Tokenizer<'a> {
    
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}


mod test {

    use std::io::MemReader;
    use super::{Tokenizer, Token};
    use super::Token::*;

    #[test]
    fn test_look_ahead() {
        let stream = "--hallo";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut tokenizer = Tokenizer::new(Box::new(reader));
        let tokens: Vec<Token> = tokenizer.collect();
        assert!(tokens == vec![SequenceEntry, SequenceEntry, Scalar("hallo".to_string())]);
    }

    #[test]
    fn test_document() {
        let stream = "---\n    - hallo\n    - yolo\n...";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut tokenizer = Tokenizer::new(Box::new(reader));
        let tokens: Vec<Token> = tokenizer.collect();
        assert!(tokens == vec![
                DocumentStart, 
                SequenceEntry, 
                Scalar("hallo".to_string()),
                SequenceEntry, 
                Scalar("yolo".to_string()),
                DocumentEnd
        ]);
        
    }
}
