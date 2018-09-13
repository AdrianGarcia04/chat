use std::net::{TcpStream, SocketAddr, Shutdown};
use red::{eventoconexion::EventoConexion, util};
use std::io::Error;

#[derive(Debug)]
pub struct Cliente {
    nombre: Option<String>,
    socket: TcpStream,
    direccion_socket: SocketAddr,
}

impl Cliente {

    pub fn new(nombre: Option<String>, socket: TcpStream, direccion_socket: SocketAddr) -> Cliente {
        Cliente {
            nombre: nombre,
            socket: socket,
            direccion_socket: direccion_socket,
        }
    }

    pub fn detener(&mut self) -> Result<(), Error> {
        util::mandar_evento(&self.socket, EventoConexion::TerminaConexion)?;
        self.socket.shutdown(Shutdown::Both).expect("Error al cerrar el socket");
        Ok(())
    }

    pub fn get_nombre(&self) -> &Option<String> {
        &self.nombre
    }

    pub fn set_nombre(&mut self, nuevo_nombre: &str) {
        self.nombre = Some(nuevo_nombre.to_string());
    }

    pub fn get_direccion_socket(&self) -> SocketAddr {
        self.direccion_socket
    }

    pub fn get_socket(&self) -> &TcpStream {
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
