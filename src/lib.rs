//
// yaml.rs
// Nothing to see here yet
//

#![crate_id="yaml"]

struct Scanner<'a> {
    reader: Box<Reader + 'a>,
}

impl <'a>Scanner<'a> {
    pub fn new<T: Reader + 'a>(reader: Box<T>) -> Scanner<'a> {
        Scanner { reader: reader } 
    }

    pub fn scan(&mut self) {

        println!("{:?}", self.reader.read_byte().unwrap());
    }

}




mod test {
    
    use std::io::MemReader;
    use std::vec;

    use super::Scanner;

    #[test]
    fn test_scanner() {
        let stream = "---";
        let mut reader = MemReader::new(stream.bytes().collect());
        let mut scanner = Scanner::new(Box::new(reader));
        scanner.scan();
    }
}
