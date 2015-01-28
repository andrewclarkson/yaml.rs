//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

use std::iter::Iterator;
use std::option::Option;
use std::io::IoResult;
use self::Token::*;

/// A Yaml Tokenizer
///
/// A tokenizer takes a byte stream and produces a stream of Tokens
/// This shelters the parser from the complexity of dealing with raw bytes.
///
pub struct Tokenizer<'a> {
    reader: Box<Reader + 'a>,
    buffer: Vec<u8>,
    position: usize,
    peek: usize,
    done: bool,
}

impl <'a>Tokenizer<'a> {
    
    /// Creates a new Tokenizer from a Reader
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Tokenizer<'a> {
        Tokenizer { 
            reader: reader, 
            done: false, 
            buffer: Vec::new(), 
            position: 0,
            peek: 0,
        } 
    }

    /// Gets the next byte
    pub fn get_byte(&mut self) -> IoResult<u8> {
        
        // If the buffer is empty, fill it
        if self.peek == self.buffer.len() {
            match self.reader.read_byte() {
                Ok(byte) => {
                    self.buffer.push(byte);
                    Ok(byte)
                },
                Err(error) => {
                    Err(error)
                }
            }
        } else {
            Ok(self.buffer[self.peek])
        }
    }

    pub fn peek(&mut self) -> IoResult<u8> {
        let byte = self.get_byte();
        match byte {
            Ok(_) => { self.peek += 1 },
            _ => {},
        }
        byte
    }

    pub fn advance(&mut self, number: usize) {
        self.position += number;
        self.peek = self.position;
    }

    /// Gets the next token in the stream
    pub fn get_next_token(&mut self) -> Option<Token> {
        if self.done {
            return None;
        }
        
        let token = match self.peek() {
            
            // TODO: Check error value
            Err(_) => {
                self.done = true;
                Token::Eof
            },
            Ok(byte) => {
                match byte as char {
                    
                    // Indicators:
                    '[' => {
                        self.advance(1);
                        SequenceStart
                    },
                    ']' => {
                        self.advance(1);
                        SequenceEnd
                    },
                    '{' => {
                        self.advance(1);
                        MappingStart
                    },
                    '}' =>  {
                        self.advance(1);
                        MappingEnd
                    },
                    '-' => {
                        if self.check_document_start() {
                            self.advance(3);
                            DocumentStart
                        } else {
                            self.advance(1);
                            SequenceEntry
                        }
                    },
                    '.' => {
                        if self.check_document_end() {
                            self.advance(3);
                            DocumentEnd
                        } else {
                            Other
                        }
                    },
                    ':' => MappingSeparator,
                    ',' => CollectionSeparator,
                    '?' => ComplexKey,
                    '!' => Tag,
                    '&' => Anchor,
                    '*' => Alias,
                    '|' => Literal,
                    '>' => Folded,
                    '\'' => SingleQuote,
                    '"' => DoubleQuote,
                    '#' => Comment,
                    '%' => Directive,
                    '@' | '`' => Reserved,
                    _ => Other,
                }
            }
        };

        Some(token)
    }

    fn check_document_start(&mut self) -> bool {
        match (self.peek(), self.peek()) {
            (Ok(byte1), Ok(byte2)) => {
                byte1 as char == '-' && byte2 as char == '-'
            },
            _ => false
        }
    } 
    
    fn check_document_end(&mut self) -> bool {
        match (self.peek(), self.peek()) {
            (Ok(byte1), Ok(byte2)) => {
                byte1 as char == '.' && byte2 as char == '.'
            },
            _ => false
        }
    } 
}


/// All the types of tokens
///
/// Describes all of the token types described by the [Yaml 1.0 Spec](http://yaml.org/spec/1.0/)
///
#[derive(PartialEq, Copy, Debug)]
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
    Anchor,

    /// `*`: indicates an anchor property (e.g using a variable)
    /// `c-alias` in the spec
    Alias,

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

    /// `%`: a directive (tags for the YAML parser)
    /// `c-directive` in the spec
    Directive,

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
    Eof,
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
        self.get_next_token()
    }
}


mod test {
    
    use std::io::MemReader;
    use std::vec;

    use super::{Tokenizer, Token};
    use super::Token::*;

    #[test]
    fn test_scanner() {
        let stream = "---[]-...";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut tokenizer = Tokenizer::new(Box::new(reader));
        let tokens: Vec<Token> = tokenizer.collect();

        assert!(tokens == vec![DocumentStart, SequenceStart, SequenceEnd, SequenceEntry, DocumentEnd, Eof]);
    }
}
