use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::* ;

fn main() {
    const HOST : &str = "127.0.0.1";  
    const PORT : &str = "8477"; // port number

    let end_point: String = HOST.to_owned() + ":" + PORT ; // creating whole url

    let listener = TcpListener::bind(end_point).unwrap(); // Creating TCS Listener at our port
    
    println!("Web Server is listening on port {}", PORT);

    for stream in listener.incoming() { // connecting to any incoming connections 
        let _stream = stream.unwrap();
        
        handle_connection(_stream);
    }
} 


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024] ; 
    stream.read(&mut buffer).unwrap();

    let response_contents = fs::read_to_string("index.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",response_contents.len(), response_contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
// reference : https://medium.com/@rameshovyas/a-step-by-step-guide-to-build-a-custom-http-server-of-own-in-rust-7308cead63a2
