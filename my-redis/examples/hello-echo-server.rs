use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    // Read it back
    let mut buf = vec![0; 1024];
    println!("echo from server:");

    match stream.read(&mut buf).await {
        // Return value of `Ok(0)` signifies that the remote has
        // closed
        Ok(0) => (),
        Ok(n) => {
            let result = str::from_utf8(&buf[..n]).unwrap();
            println!("{:?}", result);
        }
        Err(e) => {
            // Unexpected socket error. There isn't much we can do
            // here so just stop processing.
            println!("Something went wrong");
            return Err(e);
        }
    }

    Ok(())
}
