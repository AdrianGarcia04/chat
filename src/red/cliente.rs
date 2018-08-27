use std::net::{TcpStream, SocketAddr, Shutdown};
use red::{eventoconexion::EventoConexion};
use std::io::Write;

pub struct Cliente {
    nombre: String,
    socket: TcpStream,
    direccion_socket: SocketAddr,
}

impl Cliente {

    pub fn new(nombre: String, socket: TcpStream, direccion_socket: SocketAddr) -> Cliente {
        Cliente {
            nombre: nombre,
            socket: socket,
            direccion_socket: direccion_socket,
        }
    }

    pub fn detener(&mut self) {
        self.mandar_evento(EventoConexion::TerminaConexion);
        self.socket.shutdown(Shutdown::Both).expect("Error al cerrar el socket");
    }

    pub fn mandar_evento(&mut self, evento: EventoConexion) {
        let evento = evento.to_string().into_bytes();
        self.socket.write(&evento[..]).unwrap();
        self.socket.flush().unwrap();
    }

    pub fn mandar_mensaje(&mut self, mensaje: String) {
        let mensaje = mensaje.into_bytes();
        self.socket.write(&mensaje[..]).unwrap();
        self.socket.flush().unwrap();
    }
}

impl Clone for Cliente {
     fn clone(&self) -> Self {
        Cliente {
            nombre: self.nombre.clone(),
            socket: self.socket.try_clone().expect("Error al clonar"),
            direccion_socket: self.direccion_socket.clone(),
        }
    }
 }
