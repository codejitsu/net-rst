use std::net::TcpListener;

fn main() {
    let connection = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000");

    for stream in connection.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established");
    }
}
