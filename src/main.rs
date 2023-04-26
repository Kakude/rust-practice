use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use httparse;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();

    let url = env::var("URL").expect("URL must be set");

    let listener = TcpListener::bind(&url).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        router(stream);
        println!("Connection established!");
    }
}

fn router(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    stream.read(&mut buf).unwrap();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buf).unwrap();
    let path = req.path.expect("fail");
    match path {
        "/healthcheck" => {
            establish_connection();
            let body =
            "<html><head><title>rust web</title></head><body>hello world!!!!!</body></html>";
            let status = "HTTP/1.1 200 OK\r\n".to_string();
            let header = status + "Content-Type: text/html; charset=UTF-8\r\n\r\n";
            let res = header + &body + "\r\n";
            let data = res.as_bytes();
            stream.write(data).unwrap();
        }
        &_ => todo!()
    }
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    println!("mysql connected");

    connection
 }
