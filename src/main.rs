use std::{
    env, io::{prelude::*, BufReader,ErrorKind}, net::{TcpListener, TcpStream}, process::{self, Command}, thread, time::Duration
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

    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let mut http_request: Vec<String> = vec![];
    let mut post_request = false; 
    
    for line in buf_reader.lines() {
        let val = line.unwrap();
    
        if val.contains("POST") {
            println!("Reading a POST request\n");
            //post_request = true;
        }
        if !val.is_empty() {
            http_request.push(val)
        }else if post_request{
            if val.contains("nom") {
                http_request.push(val);
                break;
            }
        }else{
            break;
        }
        
    }
    let request_line = &http_request[0];
    /*let mut name: &String;
    if post_request {
        name = &http_request[http_request.len()-1]
    }else{
        name = &"None".to_owned();
    }*/

    println!("Request: {:#?}", http_request);


    
    

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
            env::set_var("REQUEST_METHOD", "GET");
            env::set_var("SCRIPT_FILENAME", "index.php");
            env::set_var("REDIRECT_STATUS", "CGI");
            env::set_var("CONTENT_TYPE", "application/www-form-urlencoded");

            let output = Command::new("php-cgi")
                .output()
                .expect("Failed to execute PHP script");

            let php_response = String::from_utf8_lossy(&output.stdout).to_string();
            let lines = php_response.lines();
            let mut response = String::new();
            let mut verif = 0;
            for line in lines {
                if verif == 1{
                    response.push_str(line);
                    response.push_str("\n");
                }
                if line.starts_with("Content-type: text/html; charset=UTF-8") {
                    verif = 1;
                }
            }
            ("HTTP/1.1 200 OK", response)
        }

        "POST /v1 HTTP/1.1" => {


            env::set_var("REQUEST_METHOD", "POST");
            env::set_var("SCRIPT_FILENAME", "index.php");
            env::set_var("REDIRECT_STATUS", "CGI");
            env::set_var("CONTENT_TYPE", "application/www-form-urlencoded");


            let mut command_str: String = "echo 'nom=".to_owned();
            command_str.push_str("julien"); // nom récupéré 
            command_str.push_str("' | php-cgi");
            println!("{:}", command_str);

            let output = Command::new("sh")
               .arg("-c")
               .arg(command_str)
               .output()
               .expect("Failed to execute command");

            let php_response = String::from_utf8_lossy(&output.stdout).to_string();
            let lines = php_response.lines();
            let mut response = String::new();
            let mut verif = 0;

            for line in lines {
                if verif == 1{
                    if !line.contains("Warning"){
                        response.push_str(line);
                        response.push_str("\n");
                    }
                }
                if line.starts_with("Content-type: text/html; charset=UTF-8") {
                    verif = 1;
                }
            }

            ("HTTP/1.1 200 OK", response)
        
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

