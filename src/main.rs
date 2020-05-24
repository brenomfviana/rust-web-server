use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use pool::ThreadPool;

fn main() {
  // Create the listener of the Server by setting the IP and Port
  let listener = TcpListener::bind("127.0.0.1:7878");
  // Create a thread pool to avoid DoS attacks
  let pool = ThreadPool::new(4);
  // Check if the listener was created successfully
  if let Ok(listener) = listener {
    // For each call
    for stream in listener.incoming().take(2) {
      // handle the connection of each stream
      if let Ok(stream) = stream {
        pool.execute(|| { handle_connection(stream); });
      } else { eprintln!("WARNNIG: The connection could not be established."); }
    }
  } else { eprintln!("The server is not allowed to use this IP and Port."); }
}

/// Handle the server connection.
fn handle_connection(mut stream: TcpStream) {
  // Read request
  let mut buffer = [0; 512];
  // Check if the request was successfully performed
  if let Ok(_) = stream.read(&mut buffer) {
    // Define the GET methods
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // Get the HTTP header and HTML filename
    let (status_line, filename) = if buffer.starts_with(get) {
      ("HTTP/1.1 200 OK\r\n\r\n", "pages/index.html")
    } else if buffer.starts_with(sleep) {
      thread::sleep(Duration::from_secs(5));
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
      eprintln!("The server could not write the HTML response.");
    } else {
      // Check if the server sent the response to the client
      if let Err(_) = stream.flush() {
        eprintln!("The server could not response the client.");
      } else { println!("The request was performed successfully!"); }
    }
  } else { eprintln!("Error: invalid request."); }
}
