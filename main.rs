use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::io::BufReader;

mod logger{
    use std::process;
    pub fn error(text: &str){
        println!("{}", format!("{} {}", "Error:", text));
        process::exit(0);
    }
}
mod response{
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;
    use std::path::Path;
    use std::io::prelude::*;
    use logger;

    pub fn build(header: String) -> String {
        let filename = self::get_filename_from_header(header);

        let path = self::file_path(filename);
        println!("{:?}", path.is_file());
        
        if path.is_file() {
            self::final_response(200, self::read_file(path))
        }else{
            // read 404
            let path_404 = self::file_path(String::from("/404.html"));
            if path_404.is_file() {
                self::final_response(404, self::read_file(path_404))
            }else{
                self::final_response(404, String::from("404 page not found"))
            }
        }
    }

    pub fn get_filename_from_header(header: String) -> String {
        let first = header.split("\r\n").nth(0).expect("unable to split header");
        // let request_type = first.split(" ").nth(0).unwrap();
        let request_path = first.split(" ").nth(1).expect("unable to find path");
        if request_path == "/" {
            String::from("/index.html")
        }else{
            String::from(request_path)    
        }
    }

    // TODO: add option to send path as argument
   
    pub fn file_path(filename: String) -> PathBuf {
         if let Some(path) = env::args().nth(1) {
            Path::new(&path).join(format!(".{}", filename)).to_path_buf()
            // Path::new(&path).to_path_buf()
        }else {
            logger::error("Please specify the path of root directory.");            
            Path::new("./").to_path_buf() // code will never reach that point !!
        }
    }

    fn final_response(status: u32, content: String) -> String{
        let header = match status {
            200 => "HTTP/1.1 200 OK\r\n\r\n",
            _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
        };
        format!("{} {}", header, content)
    }

    pub fn read_file(filename: PathBuf) -> String{

        let mut contents = String::new();
        match File::open(&filename) {
            Ok(mut file) => {
                file.read_to_string(&mut contents).expect("unable to read file");
                contents
            },
            Err(_) => String::from(" "),
        }
    }
}


fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {  
                thread::spawn(move || { 
                    handle_connection(stream)
                });
            },
            Err(e) => {  
                thread::spawn(move || { 
                    println!("Connection failed: {:?}", e); 
                });
            },
        };
    };
}

/* from https://github.com/PritiKumr/rust-httpd 
*/
fn read_request_head(stream: &TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let mut buff = Vec::new();
    let mut read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
    while read_bytes > 0 {
        read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
        if read_bytes == 2 && &buff[(buff.len()-2)..] == b"\r\n" {
            break;
        }
    }
    return buff;
}


fn handle_connection(mut stream: TcpStream) {
    let buff = read_request_head(&stream);
   
    let header = String::from_utf8(buff).expect("failed: from_utf8 vector");

    // custom press
    let resp = response::build(header);
    let buff = resp.as_bytes();

    match stream.write(&buff) {
        Ok(size) => println!("{:?}", size),
        Err(e) => println!("{:?}", e),
    };

    stream.flush().expect("flush failed");
}

