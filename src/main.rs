use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7676").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let steam = stream.unwrap();
        println!("new steam = {:?}", steam);
        // handle_connection(steam);
        handle_connection_2(steam);
    }
}

// devolviendo html
fn handle_connection(mut steam: TcpStream) {
    let buf_reader = BufReader::new(&steam);
    let http_req: Vec<String> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request = {:#?}", http_req);

    // las respuestas tienen el siguiente formato:
    /*
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
    */

    // let res = "HTTP/1.1 200 OK\r\n\r\n";

    // write_all => toma un &[u8] y envía esos bytes directamente por la conexión
    // steam.write_all(res.as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let html = fs::read_to_string("index.html").unwrap();
    let len = html.len();

    println!("html = {}", html);

    let res = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, html);

    steam.write_all(res.as_bytes()).unwrap();
}

fn handle_connection_2(mut steam: TcpStream) {
    let buf_reader = BufReader::new(&steam);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("request_line = {}", request_line);

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let html = fs::read_to_string("index.html").unwrap();
        let len = html.len();

        let res = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, html);

        steam.write_all(res.as_bytes()).unwrap();
    } else {
        println!("else");
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let html = fs::read_to_string("404.html").unwrap();
        let len = html.len();

        let res = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, html);

        steam.write_all(res.as_bytes()).unwrap();
    }
}
