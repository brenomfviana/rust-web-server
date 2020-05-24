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
      else { println!("WARNNIG: The connection could not be established."); }
    }
  } else { panic!("The server is not allowed to use this IP and Port."); }
}

/// Handles the server connection.
fn handle_connection(mut stream: TcpStream) {
  // Read request
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  // Define the GET methods
  let get = b"GET / HTTP/1.1\r\n";
  // Get the HTTP header and HTML filename
  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "pages/index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "pages/404.html")
  };
  // Server response
  let response: String;
  // Read HTML content
  if let Ok(contents) = fs::read_to_string(filename) {
    // Prepare HTML response
    response = format!("{}{}", status_line, contents);
  } else {
    // Internal server error
    let status_line = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
    // Prepare HTML response
    response = format!("{}", status_line);
  }
  // Return HTML response
  if let Err(_) = stream.write(response.as_bytes()) {
    println!("The server could not write the HTML response.");
  } else {
    // Check if the server sent the response to the client
    if let Err(_) = stream.flush() {
      println!("The server could not response the client.");
    } else { println!("Everything it's OK!"); }
  }
}
