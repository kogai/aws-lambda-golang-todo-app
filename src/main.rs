use std::io::Write;
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
                Ok(mut stream) => {
                    spawn(move || {
                        println!("[SERVER]: Recieve connection from client. {:?}", stream);
                        let response = create_response();
                        println!("[SERVER]: Send {:?}", response);
                        stream.write_all(response.as_slice()).unwrap();

                    });
                }
                Err(e) => println!("[SERVER]: Error, {:?}", e),
            }
        }
    }
}

fn create_response() -> Vec<u8> {
    let mut body = Vec::new();
    // let file = File::open(format!("{}{}", ROOT_DIR, &request_header.uri));
    // let (mut body, status) = match file {
    //     Ok(mut f) => {
    //         let mut buffer = Vec::new();
    //         f.read_to_end(&mut buffer).unwrap();
    //         (buffer, HttpStatus::from_usize(200))
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //         let mut f = File::open(format!("{}{}", ROOT_DIR, "/404.html")).unwrap();
    //         let mut buffer = Vec::new();
    //         f.read_to_end(&mut buffer).unwrap();
    //         (buffer, HttpStatus::from_usize(404))
    //     }
    // };

    let mut send_buffer = [
        format!("HTTP/1.1 200"),
        // format!("Date:  "),
        format!("Server: Modoki/0.1"),
        format!("Connection: close"),
        // format!("{}", request_header.content_type.to_string()),
        format!("application/json"),
        format!(""),
        format!(""),
    ].join("\r\n")
        .as_bytes()
        .to_vec();

    send_buffer.append(&mut body);
    send_buffer
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
