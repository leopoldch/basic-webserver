use std::{
    io::{prelude::*, BufReader,ErrorKind}, 
    net::{TcpListener, TcpStream}, 
    process::Command, 
    thread, 
    process,
    time::Duration
};

use typed_html::{
    html,
    dom::DOMTree,
    types::Metadata,
};


fn check_php(){
    match Command::new("php").spawn() {
        Ok(_) => println!("Php is installed \n"),
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                println!("Php was not found!")
            } else {
                println!("Unknown Error");
            }
            // ferme le programme 
            process::exit(0x0100);
        }, 
    }
}
    

fn main() {
    
    check_php();
    
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let doc: DOMTree<String> = html!(
                <html>
                    <head>
                        <title>"Hello!"</title>
                        <meta name=Metadata::Author />
                    </head>
                    <body>
                        <h1>"Hello from Rust !"</h1>
                        <p >
                            "Page served using Rust"
                        </p>
                    </body>
                </html>
            );
            let hello = doc.to_string();
            ("HTTP/1.1 200 OK", hello)
        },
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            let doc: DOMTree<String> = html!(
                <html>
                    <head>
                        <title>"Sleep"</title>
                        <meta name=Metadata::Author />
                    </head>
                    <body>
                        <h1>"Sleeping"</h1>
                        <p >
                            "Sleep !"
                        </p>
                    </body>
                </html>
            );
            let sleep = doc.to_string();
            ("HTTP/1.1 200 OK", sleep)
        }

        "GET /v1 HTTP/1.1" => {
            
            // demande à php de éxécuter la page et prendre ce qui est interprété

            let output = Command::new("php")
                .arg("index.php")
                .output()
                .expect("Failed to execute PHP script");

            let php_response = String::from_utf8_lossy(&output.stdout).to_string();
            ("HTTP/1.1 200 OK", php_response)
        }

        _ => {

            let doc: DOMTree<String> = html!(
                <html>
                    <head>
                        <title>"NOT FOUND"</title>
                        <meta name=Metadata::Author/>
                    </head>
                    <body>
                        <h1>"NOT FOUND"</h1>
                        <p>
                            "404 - NOT FOUND"
                        </p>
                    </body>
                </html>
            );
            let response = doc.to_string();
            ("HTTP/1.1 404 NOT FOUND", response)
        
        },
    };


    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

