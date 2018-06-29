mod parser;

use std::io;
use std::io::{Read, Write};
use std::thread;
use std::net::TcpListener;

fn server () -> io::Result<()> {
    // Listen on localhost, port 8080
    let lis = TcpListener::bind("127.0.0.1:8080")?;

    for stream in lis.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Spawn a new thread not to block, but to accept a new connection
        let _ = thread::spawn(
            move || -> io::Result<()> {
                use parser::ParseResult::*;

                let mut buf_w = Vec::new();     // Buffer that stores the whole request
                loop {
                    let mut buf_p = [0; 1024];     // Partial buffer (buffer for one read)
                    let n = stream.read(&mut buf_p)?;
                    if n == 0 {
                        return Ok(());
                    }
                    // Add that to the whole buffer
                    buf_w.extend_from_slice(&buf_p[0..n]);
                    match parser::parse(buf_w.as_slice()) {
                        Partial => continue,
                        Error => {
                            return Ok(());
                        },
                        Complete(req) => {
                            write!(stream, "OK {}\r\n", req.0)?;
                            return Ok(());
                        },
                    };
                }
            });
    }
    Ok(())
}

fn main() {
    match server() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}