use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::prelude::*;
use std::process;
use std::option;


pub fn build(header: String) -> String {
    let filename = get_filename_from_header(header);

    if let Some(path) = file_path(filename) {
        if path.is_file(){
            final_response(200, read_file(path))
        }
    }

    serv_404()
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
pub fn file_path(filename: String) -> Option<PathBuf> {
     if let Some(path) = env::args().nth(1) {
        let path = Path::new(&path).join(format!(".{}", filename)).to_path_buf();
        Some(path)
    }else {
        println!("{}", "Please specify the path of root directory.");
        None
    }
}

fn final_response(status: u32, content: String) -> String{
    let header = match status {
        200 => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
    };

    format!("{} {}", header, content)
}

fn read_file(filename: PathBuf) -> String{

    let mut contents = String::new();
    match File::open(&filename) {
        Ok(mut file) => {
            file.read_to_string(&mut contents).expect("unable to read file");
            contents
        },
        Err(_) => String::from(" "),
    }
}

fn serv_404() -> String {
    if Some(path) = file_path(String::from("/404.html")) {
        if path.is_file(){
            final_response(404, read_file(path))
        }else{
            final_response(404, String::from("404 page not found"))
        }
    }else {
        final_response(404, String::from("404 page not found"))
    }
}