use std::net::{TcpStream, SocketAddr, Shutdown};
use red::{eventoconexion::EventoConexion, estadocliente::EstadoCliente, util};
use std::io::Error;
use serde::{Serialize, Serializer, ser::SerializeStruct};

pub struct Cliente {
    nombre: Option<String>,
    socket: TcpStream,
    direccion_socket: SocketAddr,
    estado: EstadoCliente,
}

impl Cliente {

    pub fn new(nombre: Option<String>, socket: TcpStream, direccion_socket: SocketAddr) -> Cliente {
        Cliente {
            nombre: nombre,
            socket: socket,
            direccion_socket: direccion_socket,
            estado: EstadoCliente::ACTIVE
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

    pub fn get_estado(&self) -> &EstadoCliente {
        &self.estado
    }

    pub fn set_estado(&mut self, estado: EstadoCliente) {
        self.estado = estado;
    }

}

impl Clone for Cliente {

     fn clone(&self) -> Self {
        Cliente {
            nombre: self.nombre.clone(),
            socket: self.socket.try_clone().expect("Error al clonar"),
            direccion_socket: self.direccion_socket.clone(),
            estado: self.estado.clone()
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

impl Serialize for Cliente {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,{
        let mut serializable =  serializer.serialize_struct("Cliente", 3)?;
        serializable.serialize_field("nombre", &self.nombre.clone())?;
        serializable.serialize_field("estado", &self.estado.clone())?;
        serializable.serialize_field("direccion", &self.direccion_socket.clone())?;
        serializable.end()
    }
}
