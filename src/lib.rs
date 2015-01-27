//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

use std::iter::Iterator;
use std::option::Option;

/// A Yaml Tokenizer
///
/// A tokenizer takes a byte stream and produces a stream of Tokens
/// This shelters the parser from the complexity of dealing with raw bytes.
///
pub struct Tokenizer<'a> {
    reader: Box<Reader + 'a>,
    done: bool,
}

impl <'a>Tokenizer<'a> {
    
    /// Creates a new Tokenizer from a Reader
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Tokenizer<'a> {
        Tokenizer { reader: reader, done: false } 
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
                match byte as char {
                    
                    // Indicators:
                    '[' => Token::SequenceStart,
                    ']' => Token::SequenceEnd,
                    '{' => Token::MappingStart,
                    '}' => Token::MappingEnd,
                    // TODO: peek ahead to check for a document opening
                    '-' => Token::SequenceEntry,
                    ':' => Token::MappingSeparator,
                    ',' => Token::CollectionSeparator,
                    '?' => Token::ComplexKey,
                    '!' => Token::Tag,
                    '&' => Token::Anchor,
                    '*' => Token::Alias,
                    '|' => Token::Literal,
                    '>' => Token::Folded,
                    '\'' => Token::SingleQuote,
                    '"' => Token::DoubleQuote,
                    '#' => Token::Comment,
                    '%' => Token::Directive,
                    '@' | '`' => Token::Reserved,
                    _ => Token::Other,
                }
            }
        };

        Some(token)
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

    /// ``: a line break with no specific purpose
    LineBreakGeneric,

    /// ``
    LineBreak(LineBreak),

    Other, // A temporary addition for unimplemented tokens
    Eof,
}

#[derive(PartialEq, Copy, Debug)]
pub enum LineBreak {
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

    #[test]
    fn test_scanner() {
        let stream = "[";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut tokenizer = Tokenizer::new(Box::new(reader));
        let tokens: Vec<Token> = tokenizer.collect();
        let expected = vec!(Token::SequenceStart, Token::Eof);
        assert!(tokens == expected);
    }
}
