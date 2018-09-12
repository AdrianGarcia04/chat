use super::{eventoconexion::EventoConexion};
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io::{Error, ErrorKind};

pub const CHAR_NULL: u8 = 00000000;

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
                EventoConexion::Desconexion
            }
        },
        _ => {
            EventoConexion::EventoInvalido
        }
    }
}

pub fn obtener_mensaje_conexion(mut socket: &TcpStream) -> Result<String, Error> {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                Ok(mensaje_de_buffer(&buffer))
            }
            else {
                Err(Error::new(ErrorKind::ConnectionAborted, "El cliente terminó la conexión"))
            }
        },
        _ => {
            Err(Error::new(ErrorKind::ConnectionAborted, "El cliente terminó la conexión"))
        }
    }
}

pub fn mandar_evento(mut socket: &TcpStream, evento: EventoConexion) -> Result<(), Error>{
    let evento = evento.to_string().into_bytes();
    socket.write(&evento[..])?;
    socket.flush()?;
    Ok(())
}

pub fn mandar_mensaje(mut socket: &TcpStream, mensaje: String) -> Result<(), Error>{
    let mensaje = mensaje.into_bytes();
    socket.write(&mensaje[..])?;
    socket.flush()?;
    Ok(())
}
