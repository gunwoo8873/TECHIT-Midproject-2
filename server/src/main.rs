use std::{
    fs, thread, time::Duration,
    hello::ThreadPool,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:30000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //// BufReader to std::io::Read Trait method Call
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //// Path Immutable
    let index_path = "../public/index.html";
    let notfound_path = "../public/404.html";

    //// HTTP Web Response Status Check [Single Thread]
    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", index_path)
    // }
    // else {
    //     ("HTTP/1.1 404 NOT FOUND", notfound_path)
    // };

    //// HTTP Web Response Status Check [Multi Thread]
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", index_path),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", index_path)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", notfound_path),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}