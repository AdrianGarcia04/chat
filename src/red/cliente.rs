use std::net::{TcpStream, SocketAddr, Shutdown};
use red::estadocliente::EstadoCliente;

pub struct Cliente {
    nombre: Option<String>,
    socket: TcpStream,
    direccion: SocketAddr,
    estado: EstadoCliente,
}

impl Cliente {

    pub fn new(nombre: Option<String>, socket: TcpStream, direccion: SocketAddr) -> Cliente {
        Cliente {
            nombre: nombre,
            socket: socket,
            direccion: direccion,
            estado: EstadoCliente::ACTIVE
        }
    }

    pub fn get_nombre(&self) -> &Option<String> {
        &self.nombre
    }

    pub fn set_nombre(&mut self, nuevo_nombre: &str) {
        self.nombre = Some(nuevo_nombre.to_owned());
    }

    pub fn get_socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn set_socket(&mut self, socket: TcpStream) {
        self.socket = socket;
    }

    pub fn get_direccion(&self) -> SocketAddr {
        self.direccion
    }

    pub fn set_direccion(&mut self, direccion: SocketAddr) {
        self.direccion = direccion
    }

    pub fn get_estado(&self) -> &EstadoCliente {
        &self.estado
    }

    pub fn set_estado(&mut self, estado: EstadoCliente) {
        self.estado = estado;
    }

    pub fn detener(&mut self) {
        self.socket.shutdown(Shutdown::Both).expect("Error al cerrar el socket");
    }
}

impl Clone for Cliente {
     fn clone(&self) -> Self {
        Cliente {
            nombre: self.nombre.clone(),
            socket: self.socket.try_clone().expect("Error al clonar"),
            direccion: self.direccion.clone(),
            estado: self.estado.clone()
        }
    }
 }
