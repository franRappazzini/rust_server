use server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // let port = std::env::var("PORT").unwrap_or_else(|_| "7676".to_string());
    let port = 7676;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::build(4).unwrap();

    println!("Server is running on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        "GET /api HTTP/1.1" => ("HTTP/1.1 200 OK", "test.json"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let file = fs::read_to_string(filename).unwrap();
    let len = file.len();

    let res = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{file}");

    stream.write_all(res.as_bytes()).unwrap();
}
