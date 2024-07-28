use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7676").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let steam = stream.unwrap();
        println!("new steam = {:?}", steam);
        let pool = ThreadPool::new(4);

        // aca estamos creando un nuevo thread por cada solicitud (posible DoS attacking). despues vamos a crear una pool de threads
        // thread::spawn(|| {
        //     handle_connection_4(steam);
        // });

        pool.execute(|| {
            handle_connection(steam);
        });
    }
}

// sleep server en un single thread => hace que si una solicitud tarda mucho, para a todas las solicitudes
fn handle_connection(mut steam: TcpStream) {
    let buf_reader = BufReader::new(&steam);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    println!("request line: {}", req_line);

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let html = fs::read_to_string(filename).unwrap();
    let len = html.len();

    let res = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, html);

    steam.write_all(res.as_bytes()).unwrap();
}
