use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    HEAD, 
    PUT, 
    DELETE, 
    OPTIONS, 
    CONNECT
}

#[derive(Debug)]
pub struct Header {
    pub method: Option<RequestType>,
    pub path: Result<String, FromUtf8Error>,
    pub version: Option<f32>
}

#[derive(Debug)]
struct Source {
    header: Vec<u8>,
    pos: usize
}

impl Source {
    pub fn new(header: Vec<u8>) -> Source {
        Source {
            header: header,
            pos: 0
        }
    }

    pub fn bump(&mut self){
        self.pos += 1;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn get(&self) -> Option<u8> {
        self.header.get(self.pos).cloned()
    }

    // fn next(&mut self) -> Option<u8> {
    //     if self.header.len() > self.pos {
    //         self.pos += 1;
    //         let val = self.header.get(self.pos).unwrap();
    //         Some(val.clone())
    //     } else {
    //         None
    //     }
    // }

    // list of string 
    fn get_keyword(&mut self) -> Vec<u8> {
        let mut keyword = Vec::new();
        while let Some(token) = self.get() {
            if self.is_word(&token){
                self.bump();
                keyword.push(token);                   
            }else{
                break
            }
        }
        keyword
    }
    
    fn get_char_chain(&mut self) -> Vec<u8> {
        let mut keyword = Vec::new();
        while let Some(token) = self.get() {
            if self.is_word(&token) || self.is_num(&token) || self.is_valid_char(&token) {
                self.bump();
                keyword.push(token);                   
            }else{
                break
            }
        }

        keyword
    }

    fn is_word(&self, token: &u8) -> bool {
        match *token {
            b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'G' | b'H' | b'I' | b'J' | 
            b'K' | b'L' | b'M' | b'N' | b'O' | b'P' | b'Q' | b'R' | b'S' | b'T' | 
            b'U' | b'V' | b'W' | b'X' | b'Y' | b'Z' | 
            b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'g' | b'h' | b'i' | b'j' | 
            b'k' | b'l' | b'm' | b'n' | b'o' | b'p' | b'q' | b'r' | b's' | b't' | 
            b'u' | b'v' | b'w' | b'x' | b'y' | b'z' => true,
            _ => false,
        }
    }
    fn is_num(&self, token: &u8) -> bool {
        match *token {
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'  => true,
            _ => false,
        }
    }
    fn is_valid_char(&self, token: &u8) -> bool {
        match *token {
            b'-' | b'.' | b'_' | b'~' | b':' | b'/' | b'?' | b'#' | b'[' | b']'|
            b'@' | b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b','|
            b';' | b'=' 
              => true,
            _ => false,
        }
    }
}


pub fn parse(header_buffer: Vec<u8>) -> Header {

    let mut method = vec![];
    let mut path = vec![];
    let mut html_version = vec![];

    let mut parser = Source::new(header_buffer);
    while let Some(_) = parser.get() {
        if parser.pos() == 0 {
            method = parser.get_keyword();

            parser.bump(); // remove white space
            path = parser.get_char_chain();

            parser.bump(); // remove white space
            html_version = parser.get_char_chain();
        }else{
            // parse headers
            parser.bump();

            // TODO: remove \r\n  and then collect values
        }
    }

    let method_enum = match &method[..] {
        b"GET"       =>  Some(RequestType::GET),
        b"POST"      =>  Some(RequestType::POST),
        b"HEAD"      =>  Some(RequestType::HEAD), 
        b"PUT"       =>  Some(RequestType::PUT), 
        b"DELETE"    =>  Some(RequestType::DELETE), 
        b"OPTIONS"   =>  Some(RequestType::OPTIONS), 
        b"CONNECT"   =>  Some(RequestType::CONNECT),
        _            => None
    };


    Header{
        method: method_enum, 
        path: String::from_utf8(path),
        version: match &html_version[..] {
            b"HTTP/1.0" =>  Some(1.0),
            b"HTTP/2"   =>  Some(2.0),
            _           =>  Some(1.1),
        }
    }
}

