use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  // Create the listener of the Server by setting the IP and Port
  let listener = TcpListener::bind("127.0.0.1:7878");
  // Check if the listener was created successfully
  if let Ok(listener) = listener {
    // For each call
    for stream in listener.incoming() {
      // handle the connection of each stream
      if let Ok(stream) = stream { handle_connection(stream); }
    }
  } else { panic!("The server is not allowed to use this IP and Port."); }
}

/// Handles the server connection.
fn handle_connection(mut stream: TcpStream) {
  // Read request
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  // Read HTML page
  let contents = fs::read_to_string("hello.html").unwrap();
  // Prepare response
  let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
  // Return HTML response
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
