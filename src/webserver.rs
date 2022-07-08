use std::env;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
  let response = b"HTTP/1.0 404 Not Found\r\nContent-type: text/html\r\n\r\n
  <html>
  <head>
  <title>Unimplemented</title>
  </head>
  <body>
  <h1>Unimplemented</h1>
  </body>
  </html>";
  stream.write(response);
}

struct ServerConfig {
  port: u16,
  address: String,
}

struct ServerInstance {
  clients: Vec<TcpStream>,
  config: ServerConfig,
}

struct HTTPResponse {
  status_code: u8,
}

struct ArgumentParser {}

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  println!("{:#?}", args);

  let listener = TcpListener::bind("127.0.0.1:34254")?;

  for stream in listener.incoming() {
    handle_client(stream?);
  }
  
  Ok(())
}
