use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use futures::executor::ThreadPoolBuilder;
use futures::task::SpawnExt;

///
pub fn handle_http_stream(mut stream: TcpStream) {
    // let mut buffer = String::new();
    // let read = stream.read_to_string(&mut buffer);

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);

    for line in http_request {
        match &line[..] {
            "GET /sleep HTTP/1.1" => {
                sleep(Duration::from_secs(3)); // Mock IO operate
            }
            _ => {
                // TODO CODE HERE
            }
        }
    }

    let status_line = "HTTP/1.1 200 OK";
    let contents = r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <title>Hello, Eric!</title>
                </head>
                <body>
                    <h1>Hello, Eric!</h1>
                    <h2 style="color:red">平安喜乐！工作顺利！</h2>
                </body>
            </html>
        "#;
    let length = contents.len();
    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[allow(dead_code, unused)]
/// Demo 1. WebServer (single thread)
pub fn start_webserver_single_thread() {
    let host = "127.0.0.1:8080";
    // WARN: it's not tokio::net::TcpListener
    let listener = TcpListener::bind(host).unwrap();
    println!("Listen on port: {:?}", host);
    for result in listener.incoming() {
        println!("Connection established!!!");
        let stream = result.unwrap();
        handle_http_stream(stream);
    }
}

/// Demo 2. WebServer (multi threads)
pub fn start_webserver_multi_threads() {
    let host = "127.0.0.1:8080";
    // WARN: it's not tokio::net::TcpListener
    let listener = TcpListener::bind(host).unwrap();
    println!("Listen on port: {:?}", host);
    for result in listener.incoming() {
        println!("Connection established!!!");
        let stream = result.unwrap();
        thread::spawn(|| {
            handle_http_stream(stream);
        });
    }
}

/// Demo 3. WebServer (thread pool)
pub fn start_webserver_thread_pool() {
    let host = "127.0.0.1:8080";
    let mut builder = ThreadPoolBuilder::new();
    let pool = builder.pool_size(3).create().unwrap();
    // WARN: it's not tokio::net::TcpListener
    let listener = TcpListener::bind(host).unwrap();
    println!("Listen on port: {:?}", host);
    for result in listener.incoming() {
        println!("Connection established!!!");
        let stream = result.unwrap();
        // TODO
    }
}

#[allow(dead_code, unused)]
fn main() {
    // start_webserver_single_thread();
    // start_webserver_multi_threads();
    start_webserver_thread_pool();
}

