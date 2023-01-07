use core::time;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, meta) = listener.accept().await.unwrap();

        println!("\n== ACCEPTED: {:?} ==", meta);
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, meta, db).await;
        });
    }
}

async fn process(socket: TcpStream, meta: SocketAddr, db: Db) {
    let mut connection = Connection::new(socket);

    // while to accept multiple commands from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("[{meta}] PROCESSING: GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // println!("[{meta}] SLEEP: 5 seconds");
        // std::thread::sleep(time::Duration::from_secs(5));
        // println!("[{meta}] SLEEP: OVER");

        connection.write_frame(&response).await.unwrap();
    }
}
