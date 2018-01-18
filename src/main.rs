use std::net::TcpListener;
use std::thread::spawn;
use std::env;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str) -> Self {
        Server { listener: TcpListener::bind(host).unwrap() }
    }

    pub fn listen(&mut self) {
        println!("[SERVER]: Waiting for connection from client...");

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    spawn(move || {
                        println!("[SERVER]: Recieve connection from client. {:?}", stream);
                    });
                }
                Err(e) => println!("[SERVER]: Error, {:?}", e),
            }
        }
    }
}


fn main() {
    let port = match env::var("_LAMBDA_SERVER_PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };
    let url = format!("127.0.0.1:{}", port);
    let mut server = Server::new(&url);
    println!("Hello, world! {}", url);

    server.listen();
}
