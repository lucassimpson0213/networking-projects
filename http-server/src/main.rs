#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::dbg_macro)]
#![deny(unsafe_code)]
use core::str::Utf8Error;

use crate::handler::Response;
mod errors;
mod handler;
mod request;



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

// Request body (empty)
//TODO make a test script using curl localhost:4221/abcsad
fn main() -> Result<(), std::io::Error>{
    //println!("hello");

    // let _ = bind_port();
    // let app = App::new()

    // app.get("/");
    // app.get("/echo/hello");
    // app.get("/user-agent")
    // let _ = bind_port();
    //

    let mut server = handler::Server::new();
    let mut port = String::new();
    port.push_str("127.0.0.1:8081");
    server.register_route("/".to_string(), response_handler);
    let _started = handler::Server::start(&mut server, port)?;
   
    
    Ok(())
}

fn response_handler(request: &Request) -> Response {
    return Response:
}


enum ServerError {
    FailedToStart
}


#[cfg(test)]
pub mod tests {
    
    use crate::request::parse_content_len_and_string;
 

    pub struct TestingBuf {
        request_line: &'static str,
    }

    #[test]
    pub fn return_request_line() -> Result<(), std::str::Utf8Error> {
        let test_struct = crate::tests::TestingBuf {
            request_line: "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n",
        };

        // let target = parse_request_target(test_struct.request_line.as_bytes())?;

        // assert_eq!(std::str::from_utf8(&target), Ok("/index.html"));

        Ok(())
    }

    #[test]
    pub fn parse_content_unit_test() {
        let path_vec = [
            ("/echo/abc", Some((3_usize, "abc"))),
            ("/echo/", Some((0_usize, ""))),
            ("/echo/hello-world", Some((11_usize, "hello-world"))),
            ("/echo/with spaces", Some((11_usize, "with spaces"))),
            ("/echo/12345", Some((5_usize, "12345"))),
            ("/echo/a", Some((1_usize, "a"))),
            ("/echo/!", Some((1_usize, "!"))),
            ("/echo/hello_world", Some((11_usize, "hello_world"))),
            ("/echo/hello%20world", Some((13_usize, "hello%20world"))),
            ("/echo/abc/def", Some((7_usize, "abc/def"))),
            ("/echo//", Some((1_usize, "/"))),
            ("/echo/🦀", Some((4_usize, "🦀"))),
            ("/", None),
            ("/index.html", None),
            ("/echo", None),
            ("/echoe/abc", None),
            ("/Echo/abc", None),
            ("/api/echo/abc", None),
            ("echo/abc", None),
            ("", None),
        ];

        for (path, expected) in path_vec {
            let result = parse_content_len_and_string(path.as_bytes()).ok();

            // assert_eq!(Some(result), Some(expected));
        }
    }
}
