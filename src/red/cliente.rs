use std::net::{TcpStream, SocketAddr};

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
