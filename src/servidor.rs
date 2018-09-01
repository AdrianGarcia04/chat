extern crate chat;

use chat::red;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ip = &args[1];
    let puerto = &args[2];

    let mut servidor = red::servidor::Servidor::new(ip.to_string(), puerto.to_string());
    servidor.comenzar();
}
