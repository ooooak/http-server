use std::env;
use std::path::PathBuf;
use std::path::Path;
use file_io;
use headers;

pub fn create(header_buffer: Vec<u8>) -> Vec<u8> {

    let header = headers::parse(header_buffer);
    match header.path {
        Ok(request_path) => {
            if request_path == "/"{
                    build_content(String::from("/index.html"))
            }else {
                build_content(request_path)
            }
        },
        Err(_) => serv_404(), // handle error wisely
    }
}

fn build_content(filename: String) -> Vec<u8> {
    if let Some(path) = file_path(filename) {
        if path.is_file(){
            final_response(200, file_io::read(path))
        }else{
            serv_404()        
        }
    }else{
        serv_404()
    }
}

// pub fn get_filename_from_header(header: String) -> String {
//     if request_path == "/" {
//         String::from("/index.html")
//     }else{
//         String::from(request_path)    
//     }
// }

// TODO: add option to send path as argument
pub fn file_path(filename: String) -> Option<PathBuf> {
     if let Some(path) = env::args().nth(1) {
        let path = Path::new(&path).join(format!(".{}", filename)).to_path_buf();
        Some(path)
    }else {
        None
    }
}

fn final_response(status: u32, content: Option<Vec<u8>>) -> Vec<u8> {
    let header = match status {
        200 => String::from("HTTP/1.1 200 OK\r\n\r\n").as_bytes().to_vec(),
        _ => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n").as_bytes().to_vec(),
    };
    let data = content.expect("unable to decode content");
    [&header[..], &data[..]].concat()
}

fn serv_404() -> Vec<u8> {
    if let Some(path) = file_path(String::from("/404.html")) {
        if path.is_file(){
            final_response(404, file_io::read(path))
        }else{
            let resp = String::from("404 page not found").as_bytes().to_vec();
            final_response(404, Some(resp))
        }
    }else {
        let resp = String::from("404 page not found").as_bytes().to_vec();
        final_response(404, Some(resp))
    }
}