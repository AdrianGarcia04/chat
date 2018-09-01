extern crate chat;

use chat::red;
use std::env;
use std::thread;
use std::net::TcpStream;
use std::io;
use std::io::Write;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();

    let ip = &args[1];
    let puerto = &args[2];

    let mut cliente = TcpStream::connect(ip.to_string() + ":" + puerto)
                    .expect("Error al conectar cliente");

    let mut escucha = cliente.try_clone().unwrap();

    thread::spawn(move || loop {
        let mut buffer = [0; 180];
        match escucha.read(&mut buffer) {
            Ok(count) => {
                if count > 0 {
                    let mensaje = red::util::mensaje_de_buffer(&buffer);
                    println!("Mensaje del servidor: {:?}", mensaje);
                }
            }
            _ => {

            }
        };
    });

    loop {
        let mut mensaje = String::new();
        io::stdin().read_line(&mut mensaje).unwrap();
        let mut bytes = mensaje.into_bytes();
        bytes.pop(); // Quitando el caracter '\n'.
        cliente.write(&bytes[..]).unwrap();
        cliente.flush().unwrap();
    }
}
