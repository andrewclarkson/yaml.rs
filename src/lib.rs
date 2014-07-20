//
// yaml.rs
// Nothing to see here yet
//
#![feature(macro_rules)]

extern crate serialize;

use std::io::{IoError, IoResult, Writer, MemWriter};
use std::collections::{TreeMap};
use serialize::{Encodable, Encoder, Decodable, Decoder};


#[deriving(Clone, PartialEq, PartialOrd)]
pub enum Yaml {
        Integer(int),
        Float(f64),
        String(String),
        Boolean(bool),
        Sequence(Vec<Yaml>),
        Map(TreeMap<String, Yaml>),
        Null,
}


//
// Encode a Rust object into a yaml document 
// in typical yaml terms this would be 'dump'
//
//
#[macro_export]
macro_rules! encode {
    ($obj:ident) => {
        {
            let mut writer = MemWriter::new(); 
            {
                let mut encoder = YamlEncoder::new(&mut writer);
                $obj.encode(&mut encoder);
            }
            String::from_utf8(writer.unwrap()).unwrap()
        }
    }  
}


pub type EncodeResult = IoResult<()>;

struct YamlEncoder<'writer> {
    writer: &'writer mut Writer
}

impl<'writer> YamlEncoder<'writer> {
    
    pub fn new(writer: &'writer mut Writer) -> YamlEncoder<'writer> {
        YamlEncoder {
            writer: writer
        }
    }

    //pub fn buffer_encode<'writer, T:Encodable<YamlEncoder<'writer>, IoError>>(object: &T) -> Vec<u8> {
    //    let writer = MemWriter::new();
    //    {
    //        let mut encoder = YamlEncoder::new(&mut writer as &mut Writer);
    //        object.encode(&mut encoder);
    //    }
    //    writer.unwrap()
    //}
    

}

impl <'writer>Encoder<IoError> for YamlEncoder<'writer> {
        // Primitive types:
    fn emit_nil(&mut self) -> EncodeResult {
        write!(self.writer, "null")
    }

    // TODO
    fn emit_uint(&mut self, v: uint) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_u64(&mut self, v: u64) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_u32(&mut self, v: u32) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_u16(&mut self, v: u16) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    // TODO
    fn emit_u8(&mut self, v: u8) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    // TODO
    fn emit_int(&mut self, v: int) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_i64(&mut self, v: i64) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_i32(&mut self, v: i32) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_i16(&mut self, v: i16) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    // TODO
    fn emit_i8(&mut self, v: i8) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    fn emit_bool(&mut self, v: bool) -> EncodeResult {
        if v {
            write!(self.writer, "true")
        } else {
            write!(self.writer, "false")
        }
    }

    
    // TODO
    fn emit_f64(&mut self, v: f64) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_f32(&mut self, v: f32) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }
    
    // TODO
    fn emit_char(&mut self, v: char) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    // TODO
    fn emit_str(&mut self, v: &str) -> EncodeResult { 
        write!(self.writer, "Unsupported") 
    }

    fn emit_enum(&mut self, _name: &str, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_enum_variant(&mut self,
                         name: &str,
                         _id: uint,
                         cnt: uint,
                         f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_enum_variant_arg(&mut self,
                             idx: uint,
                             f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_enum_struct_variant(&mut self,
                                name: &str,
                                id: uint,
                                cnt: uint,
                                f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_enum_struct_variant_field(&mut self,
                                      _: &str,
                                      idx: uint,
                                      f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_struct(&mut self,
                   _: &str,
                   _: uint,
                   f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_struct_field(&mut self,
                         name: &str,
                         idx: uint,
                         f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_tuple(&mut self, len: uint, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }
    fn emit_tuple_arg(&mut self,
                      idx: uint,
                      f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_tuple_struct(&mut self,
                         _name: &str,
                         len: uint,
                         f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }
    fn emit_tuple_struct_arg(&mut self,
                             idx: uint,
                             f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_option(&mut self, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }
    fn emit_option_none(&mut self) -> EncodeResult { 
        self.emit_nil() 
    }
    
    fn emit_option_some(&mut self, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        f(self)
    }

    fn emit_seq(&mut self, _len: uint, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_seq_elt(&mut self, idx: uint, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_map(&mut self, _len: uint, f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_map_elt_key(&mut self,
                        idx: uint,
                        f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }

    fn emit_map_elt_val(&mut self,
                        _idx: uint,
                        f: |&mut YamlEncoder<'writer>| -> EncodeResult) -> EncodeResult {
        write!(self.writer, "Unsupported") 
    }
}

#[cfg(test)]
mod tests {
    
    #![feature(phase)]
    
    extern crate serialize;

    use std::string::String;
    use std::io::MemWriter;
    use serialize::{Encodable, Encoder, Decodable, Decoder};
    
    #[phase(syntax)]
    use super::{YamlEncoder};


    #[deriving(Encodable)]
    struct Person {
        name: String,
        age: int
    }

    #[test]
    fn test_encode_null() {
        let object: Option<String> = None;
        let document = {
            let mut writer = MemWriter::new();
            {
                let mut encoder = YamlEncoder::new(&mut writer as &mut Writer);
                object.encode(&mut encoder);
            }
            String::from_utf8(writer.unwrap()).unwrap()
        };
            
        let expected = "null".to_string();
        assert_eq!(document, expected);
    }


}
