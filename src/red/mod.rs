use std::net::{TcpStream, SocketAddr};

pub struct Servidor {
    direccion: String,
    conexiones: Vec<Conexion>,
}

impl Servidor {

    pub fn new(puerto: &str) -> Servidor {
        let direccion = format!("127.0.0.1:{}", puerto);

        Servidor {
            direccion: direccion,
            conexiones: Vec::new(),
        }
    }

    pub fn direccion(&self) -> &str {
        &self.direccion
    }

    pub fn comenzar(&mut self) {

    }

    pub fn detener(&mut self) {

    }

    pub fn aceptar_conexion(&mut self, conexion: Conexion) {
        &self.conexiones.push(conexion);
    }

}

pub struct Conexion {
    socket: TcpStream,
    direccion_socket: SocketAddr,
}

impl Conexion {

    pub fn new(socket: TcpStream, direccion_socket: SocketAddr) -> Conexion {
        Conexion {
            socket: socket,
            direccion_socket: direccion_socket,
        }
    }

    pub fn detener(self) {

    }
}
