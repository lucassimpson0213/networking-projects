use std::collections::HashMap;
use crate::errors::ClientError;
use crate::request::parse_content_len_and_string;
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



//  we gotta dispatch behavior so you can register a function and then dispath it 
pub struct Router{ 
    route_map:  HashMap<String, HashMap<String, HandlerFunc>>        
}


impl Router {
    pub fn new() -> {
        
    }
}


pub struct Handler<'conn> {
    stream: &'conn TcpStream,
    router: Router
}

 impl <'conn> Handler <'conn>{
    pub fn new(stream:  &'conn TcpStream) -> Self {
        //possibly provide some metadata from TcpStream
        return Self{stream: stream, Router{route_map: HashMap::new()}};
         
    }
    pub fn handle_client(self) -> Result<(), ClientError> {
        let mut owned_stream = self.stream;
        
        
        let req = Request::new(&owned_stream)?;
        let request_target = req.request_target;
        let headers = req.headers;
        let (len, body) = (req.body_len, req.body);
        
        // let (len, header) = parse_headers(&buf); 
        let response200 = "HTTP/1.1 200 OK\r\n\r\n";
        let response404 = "HTTP/1.1 404 Not Found\r\n\r\n";
        let response_echo = format!(
            "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {:?}\r\n\r{:?}",
            len, body
        );
        // it would be nice to match on a route pattern and then act on that and write a repsponse
        // from response writer
        
        let responses: HashMap<String, HashMap<String, HandlerFunc>> = HashMap::new();



        

        if request_target == "/" {
            owned_stream.write_all(response200.as_bytes())?;
        } else if request_target.starts_with("/echo/") {
            let _response = owned_stream.write_all(&response_echo.into_bytes());
        } else if request_target.starts_with("/user-agent") {
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
    body: String,
    body_len: usize,  
    headers: String,
    buffer: String,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Result<Request, ClientError> {
        let mut owned_stream = stream;
        let mut buffer = Vec::new();

        let _amtbytes = owned_stream.read_to_end(&mut buffer);
        let buffer_as_utf8 = String::from_utf8(buffer.clone())?;
        // possibly move parse request target to request object
        let request_target = parse_request_target(&buffer)?.clone();
        let (len, body) = parse_content_len_and_string(&buffer)?;

        return Ok(Request { method: String::new(),body: body, body_len: len, request_target, headers: String::new(), buffer:buffer_as_utf8 })
    }
    // I think it makes a lot of sense to put these methods in here as they pertain to
    // information about the request

    

    pub fn headers() {

    }


}
struct Response {
    
}



type HandlerFunc = fn(&Request) -> Response;

struct ResponseWriter {



}
