use std::net::{TcpStream, SocketAddr, Shutdown};
use red::{eventoconexion::EventoConexion, util};

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
        util::mandar_evento(&self.socket, EventoConexion::TerminaConexion);
        self.socket.shutdown(Shutdown::Both).expect("Error al cerrar el socket");
    }

    pub fn nombre(&self) -> &str {
        &self.nombre[..]
    }

    pub fn direccion_socket(&self) -> SocketAddr {
        self.direccion_socket
    }

    pub fn socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn clonar_socket(&self) -> TcpStream {
        self.socket.try_clone().expect("Error al clonar socket")
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

 impl PartialEq for Cliente {
     fn eq(&self, other: &Cliente) -> bool {
         self.direccion_socket == other.direccion_socket || self.nombre == other.nombre
     }

     fn ne(&self, other: &Cliente) -> bool {
         self.direccion_socket != other.direccion_socket && self.nombre != other.nombre
     }
 }
