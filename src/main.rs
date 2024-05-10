use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use typed_html::{
    html,
    dom::DOMTree,
    types::Metadata,
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = if request_line == "GET / HTTP/1.1" {
        let doc: DOMTree<String> = html!(
            <html>
                <head>
                    <title>"Hello"</title>
                    <meta name=Metadata::Author />
                </head>
                <body>
                    <h1>"Hello from Rust"</h1>
                    <p >
                        "Hello !"
                    </p>
                </body>
            </html>
        );
        let hello = doc.to_string();
            ("HTTP/1.1 200 OK", hello)
    } else {
        let doc: DOMTree<String> = html!(
            <html>
                <head>
                    <title>"Unauthorized"</title>
                    <meta name=Metadata::Author/>
                </head>
                <body>
                    <h1>"UNAUTHORIZED"</h1>
                    <p>
                        "401 - UNAUTHORIZED"
                    </p>
                </body>
            </html>
        );
        let response = doc.to_string();
        ("HTTP/1.1 401 UNAUTHORIZED", response)
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

