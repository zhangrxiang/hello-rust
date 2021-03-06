use mini_redis::{Connection, Frame};
use std::option::Option::Some;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        //process(socket).await;

        tokio::spawn(async move {
            // process(socket).await;
            process2(socket).await;
        });
    }
}

#[allow(dead_code)]
async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("err".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

#[allow(dead_code)]
async fn process2(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Set...");
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("OK");
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("Get...");
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
