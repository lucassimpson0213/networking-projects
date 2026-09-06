use crate::TcpListener;
use crate::errors::BufferError;
use crate::errors::ClientError;
use crate::request::is_get_request;
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

mod handler {
    use std::io::Read;
    use std::net::TcpStream;
    use crate::request::parse_request_target;

    use crate::errors::ClientError;
    // why doesn't the request object own the stream and derive all the extra info like request line
    // and stuff like that
    struct Request<'a> {
        method: &'a str , 
        request_target: &'a str,
        headers: &'a str,
        buffer: Vec<u8>,
    }

    impl Request<'_> {
        pub fn new(&self, stream: TcpStream) -> Self {
            let mut owned_stream = stream;
            let mut  buffer = Vec::new();

            

            let _amtbytes = owned_stream.read_to_end(&mut buffer);
            // possibly move parse request target to request object
            let target = parse_request_target(&buffer);

            return Self{method: "GET", buffer: buffer, request_target: target.unwrap(), headers: "" };
        }
        // I think it makes a lot of sense to put these methods in here as they pertain to
        // information about the request
        fn parse_request_target() {

        }

        fn parse_content_len_and_string() {

        }

        
    }

    struct Response {}
}
pub fn handle_client(stream: TcpStream, _listener: &TcpListener) -> Result<(), ClientError> {
    let mut owned_stream = stream;
    let mut buf: Vec<u8> = Vec::new();

    let amt_bytes = owned_stream.read_to_end(&mut buf)?;

    debug_assert!(amt_bytes <= 16834);

    if amt_bytes > 16834 {
        let too_many_bytes_read = BufferError::TooManyBytesRead;
        return Err(ClientError::TcpStreamRead(too_many_bytes_read));
    }

    if !is_get_request(&buf)? {
        return Err(ClientError::UnsupportedRequest);
    }

    let target = parse_request_target(&buf)?;
    // this may be the boundary where we start making a request object
    let (len, str) = parse_content_len_and_string(&target)?;
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
