use super::{eventoconexion::EventoConexion};
use std::net::{TcpStream};
use std::io::{Read, Write};

const CHAR_NULL: u8 = 00000000;

pub fn mensaje_de_buffer(buffer: &[u8; 180]) -> String {
    let mensaje: Vec<u8> = buffer.to_vec().into_iter()
        .filter(|&x| x != CHAR_NULL).collect();
    let mensaje = String::from_utf8(mensaje).unwrap();
    mensaje
}

pub fn obtener_evento_conexion(mut socket: &TcpStream) -> EventoConexion {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                let mensaje = mensaje_de_buffer(&buffer);
                if let Ok(evento) = mensaje.parse::<EventoConexion>() {
                    evento
                }
                else {
                    EventoConexion::EventoInvalido
                }
            }
            else {
                EventoConexion::EventoInvalido
            }
        },
        _ => {
            EventoConexion::EventoInvalido
        }
    }
}

pub fn obtener_mensaje_conexion(mut socket: &TcpStream) -> String {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                mensaje_de_buffer(&buffer)
            }
            else {
                String::new()
            }
        },
        _ => {
            String::new()
        }
    }
}

pub fn mandar_evento(mut socket: &TcpStream, evento: EventoConexion) {
    let evento = evento.to_string().into_bytes();
    socket.write(&evento[..]).unwrap();
    socket.flush().unwrap();
}

pub fn mandar_mensaje(mut socket: &TcpStream, mensaje: String) {
    let mensaje = mensaje.into_bytes();
    socket.write(&mensaje[..]).unwrap();
    socket.flush().unwrap();
}
