use crate::TcpListener;
use crate::errors::BufferError;
use crate::errors::ClientError;
use crate::request::parse_content_len_and_string;
use crate::request::parse_headers;
use crate::request::parse_request_target;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

// Request line
// GET                          // HTTP method
// /index.html                  // Request target
// HTTP/1.1                     // HTTP version
// \r\n                         // CRLF that marks the end of the request line
//
// // Headers
// Host: localhost:4221\r\n     // Header that specifies the server's host and port
// User-Agent: curl/7.64.1\r\n  // Header that describes the client's user agent
// Accept: */*\r\n              // Header that specifies which media types the client can accept
// \r\n                         // CRLF that marks the end of the headers


struct Handler {
    stream: TcpStream
}


impl Handler {
    pub fn handle_client(self) -> Result<(), ClientError> {
        let mut owned_stream = self.stream;
        
        
        let req = Request::new(owned_stream)?;
        let request_target = 

        let (len, str) = parse_content_len_and_string()?;
        let (len, header) = parse_headers(&buf);

        let response200 = "HTTP/1.1 200 OK\r\n\r\n";
        let response404 = "HTTP/1.1 404 Not Found\r\n\r\n";
        let response_echo = format!(
            "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {:?}\r\n\r{:?}",
            len, str
        );

        if str::from_utf8(&target.clone())? == "/" {
            owned_stream.write_all(response200.as_bytes())?;
        } else if str::from_utf8(&target)?.starts_with("/echo/") {
            let _response = owned_stream.write_all(&response_echo.into_bytes());
        } else if str::from_utf8(&target)?.starts_with("/user-agent") {
        } else {
            owned_stream.write_all(response404.as_bytes())?;
        }

        Ok(())
    }



}

// honestly method should be an enum with several different options
// headers could also be modeled a little differently
struct Request {
    method: String,
    request_target: String,
    headers: String,
    buffer: String,
}

impl Request {
    pub fn new(stream: TcpStream) -> Result<Request, ClientError> {
        let mut owned_stream = stream;
        let mut buffer = Vec::new();

        let _amtbytes = owned_stream.read_to_end(&mut buffer);
        let buffer_as_utf8 = String::from_utf8(buffer.clone())?;
        // possibly move parse request target to request object
        let request_target = parse_request_target(&buffer)?.clone();

        return Ok(Request { method: String::new(), request_target, headers: String::new(), buffer:buffer_as_utf8 })
    }
    // I think it makes a lot of sense to put these methods in here as they pertain to
    // information about the request

    fn parse_content_len_and_string() {}


}
struct Response {}
