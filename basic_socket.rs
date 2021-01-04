use std::net::{TcpListener, TcpStream};
use std::io::{ Write, BufReader, BufRead};

fn handle_connection(mut stream: TcpStream){
    let mut s = String::new();
    println!("[*] Connection received: {:?}", stream);
    let stream_clone = match stream.try_clone(){ 
        Ok(s) => s,
        Err(_) => return
    };
    let mut reader = BufReader::new(stream_clone);
    loop {
        s.clear();
        let _ = stream.write_all(b"> ");
        let len_recv = reader.read_line(&mut s);
        match len_recv {
            Ok(l) => if l <= 0 { break } ,
            Err(_) => break
        }
        println!("[*] Received: {}", s.trim()); 
        if s.trim() == "exit"{
            break;
        }
    }
    let _ = stream.write_all(b"Goodbye!");
    println!("[*] Ending connection");
}

fn main() {
    let listen = TcpListener::bind("0.0.0.0:9999");
    let listen = match listen {
        Ok(listen) => listen,
        Err(_) => panic!("Error establishing listener")
    };
    for stream in listen.incoming(){
        if let Ok(s) = stream {
            handle_connection(s);
        }
    }
}
