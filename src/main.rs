use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
#[derive(Debug)]
enum ProtoError {
    BindError,
}

type ProtoResult = Result<(), std::io::Error>;
const HOST_NAME: &str = "http://mytest.com";
const GET_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
fn main() -> ProtoResult {
    let mut pages = HashMap::new();
    pages.insert("/", "<html><main><p>This is a paragraph</p></main></html>");
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let (mut stream, socket) = listener.accept()?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    println!("Hello, world!");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    stream.write_all(GET_HEADER.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request);
}
