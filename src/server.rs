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
        let pool = ThreadPool::build(4).unwrap();

        // aca estamos creando un nuevo thread por cada solicitud (posible DoS attacking). despues vamos a crear una pool de threads
        // thread::spawn(|| {
        //     handle_connection_4(steam);
        // });

        pool.execute(|| {
            handle_connection_4(steam);
        });
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

fn handle_connection_3(mut steam: TcpStream) {
    let buf_reader = BufReader::new(&steam);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let html = fs::read_to_string(filename).unwrap();
    let len = html.len();

    let res = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, html);

    steam.write_all(res.as_bytes()).unwrap();
}

// sleep server en un single thread => hace que si una solicitud tarda mucho, para a todas las solicitudes
fn handle_connection_4(mut steam: TcpStream) {
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

//
// Mejorando el rendimiento con un pool de hilos
// Cuando el programa recibe una nueva tarea, asigna uno de los hilos del grupo a la tarea, y ese hilo procesará la tarea. Los hilos restantes en el grupo están disponibles para manejar cualquier otra tarea que llegue mientras el primer hilo está procesando. Cuando el primer hilo termina de procesar su tarea, se devuelve al grupo de hilos inactivos, listo para manejar una nueva tarea. Un pool de hilos le permite procesar conexiones de forma concurrente, aumentando el rendimiento de su servidor
// Limitaremos el número de hilos en el grupo a un número pequeño para protegernos de los ataques de denegación de servicio (DoS); si nuestro programa creara un nuevo hilo para cada solicitud que llegara, alguien que hiciera 10 millones de solicitudes a nuestro servidor podría crear el caos al agotar todos los recursos de nuestro servidor y detener el procesamiento de las solicitudes
// En lugar de crear un nuevo hilo para cada solicitud, crearemos un grupo de hilos que actuarán como un pool de hilos. Cuando llega una solicitud, el servidor enviará la solicitud al pool de hilos. El pool de hilos mantendrá una cola de solicitudes entrantes. Cada uno de los hilos en el pool sacará una solicitud de esta cola, manejará la solicitud y luego pedirá a la cola otra solicitud. Con este diseño, podemos procesar hasta N solicitudes simultáneamente, donde N es el número de hilos. Si cada hilo responde a una solicitud de larga duración, las solicitudes posteriores aún pueden acumularse en la cola, pero hemos aumentado el número de solicitudes de larga duración que podemos manejar antes de llegar a ese punto
fn handle_connection_5(mut steam: TcpStream) {}
