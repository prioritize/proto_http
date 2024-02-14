use std::io::{Read, Write};
use std::net::TcpListener;
#[derive(Debug)]
enum ProtoError {
    BindError,
}
type ProtoResult = Result<(), std::io::Error>;
const HOST_NAME: &str = "http://mytest.com";
const GET_HEADER: &str = "GET / HTTP/1.1";
fn main() -> ProtoResult {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let (mut stream, socket) = listener.accept()?;
    let mut read_buffer = Vec::new();
    let mut header_buff = Vec::new();
    writeln!(&mut header_buff, "{}", GET_HEADER)?;
    writeln!(&mut header_buff, "{}", HOST_NAME)?;
    // let nice_message = r#"GET / HTTP/1.1\r\n"#;
    let req = stream.read(&mut read_buffer).expect("Unable to w");
    stream
        .write(&header_buff)
        .expect("Unable to write the message");
    println!("{:?}", read_buffer);

    println!("Hello, world!");
    Ok(())
}
