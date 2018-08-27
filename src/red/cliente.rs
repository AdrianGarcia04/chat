use std::net::{TcpStream, SocketAddr};

pub struct Cliente {
    nombre: Option<String>,
    direccion: Option<String>,
    socket: Option<TcpStream>,
    direccion_socket: Option<SocketAddr>,
}

impl Cliente {

    pub fn new(nombre: Option<String>, direccion: Option<String>, socket: Option<TcpStream>,
            direccion_socket: Option<SocketAddr>) -> Cliente {
        Cliente {
            nombre: nombre,
            direccion: direccion,
            socket: socket,
            direccion_socket: direccion_socket,
        }
    }

    pub fn conectar(&mut self) {
        let direccion = match self.direccion {
            Some(ref direccion) => direccion,
            None => panic!(),
        };
        let tcp_stream = TcpStream::connect(direccion)
            .expect("Error connecting client");
        self.socket = Some(tcp_stream);
    }

    pub fn detener(self) {

    }
}

impl Clone for Cliente {

    fn clone(&self) -> Self {
        let _socket = match self.socket {
            Some(ref sock) => sock.try_clone().unwrap(),
            None => panic!(),
        };
        Cliente {
            nombre: self.nombre.clone(),
            direccion: self.direccion.clone(),
            socket: Some(_socket),
            direccion_socket: self.direccion_socket.clone(),
        }
    }

}
