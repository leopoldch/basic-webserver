use std::{
        io::{prelude::*, 
        BufReader}, 
        net::{TcpListener, TcpStream},  
        thread, 
        time::Duration
};

use typed_html::{
    html,
    dom::DOMTree,
    types::Metadata,
};

    

fn main() {
        
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("Server started on addr 0.0.0.0:3000");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {

    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let mut http_request: Vec<String> = vec![];
    
    for line in buf_reader.lines() {
        let val = line.unwrap();
    
        if val.contains("POST") {
            println!("Reading a POST request\n");
        }
        if !val.is_empty() {
            http_request.push(val)
        }else{
            break;
        }
        
    }
    let request_line = &http_request[0];

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

        _ => {

            /* 
            let name : &String = &request_line;
            let name = &name.replace("GET", "");
            let name = &name.replace("HTTP/1.1","");
            let name = &name.replace("/","");
            */
            
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

