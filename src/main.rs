use tokio::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     let pool = ThreadPool::new(4);

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             handle_connection(stream);
//         });
//     }
// }

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {
    stream.readable().await.unwrap();
    
    let mut buffer = [0; 4096];

    // TODO: better error handling, especially on would_block errors
    // should retry if try_read or write results in that error
    stream.try_read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "test.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.writable().await.unwrap();

    stream.try_write(response.as_bytes()).unwrap();
}
