use super::{eventoconexion::EventoConexion};
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io::{Error, ErrorKind};

/// Constante que representa al carácter nulo, presente cuando el buffer lee un mensaje
/// por completo sin llenarse.
pub const CHAR_NULL: u8 = 0;

/// Constante que representa el carácter de salto de línea "\n".
pub const SALTO_DE_LINEA: u8 = 10;

/// Regresa una cadena extraída de un buffer de carácteres en UTF-8
pub fn mensaje_de_buffer(buffer: &[u8; 180]) -> String {
    let mut mensaje: Vec<u8> = buffer.to_vec().into_iter()
        .filter(|&x| x != CHAR_NULL).collect();
    if mensaje.len() > 1 {
        if mensaje[mensaje.len() - 1] == SALTO_DE_LINEA {
            mensaje.pop();
        }
    }
    let mensaje = String::from_utf8(mensaje).unwrap();
    mensaje
}

/// Dado un socket de comunicación, regresa una cadena con el mensaje leído.
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

/// Dado un socket de comunicación de un cliente, regresa una tupla que contiene el
/// evento del protocolo que especificó el cliente y un vector con los argumentos de dicho evento.
pub fn obtener_mensaje_cliente(mut socket: &TcpStream)
    -> Result<(EventoConexion, Vec<String>), Error> {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                let mensaje = mensaje_de_buffer(&buffer);
                let mut argumentos: Vec<String> = mensaje.split(" ").map(|s| s.to_string()).collect();
                let evento = argumentos.remove(0).parse::<EventoConexion>();
                match evento {
                    Ok(evento) => {
                        Ok((evento, argumentos))
                    },
                    Err(_) => {
                        Ok((EventoConexion::INVALID, Vec::new()))
                    }
                }
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

/// Envía un mensaje por un socket de comunicación.
pub fn enviar_mensaje(mut socket: &TcpStream, mensaje: String) -> Result<(), Error>{
    let mensaje = mensaje.into_bytes();
    socket.write(&mensaje[..])?;
    socket.flush()?;
    Ok(())
}
