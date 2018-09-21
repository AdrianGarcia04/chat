use std::net::{TcpStream, SocketAddr, Shutdown};
use red::estadocliente::EstadoCliente;
use std::io::{Error, Write};

/// Representación abstracta de los clientes conectados al servidor.
/// Los clientes tienen un nombre asociado único, un socket de comunicación
/// [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html),
/// una dirección IP [`SocketAddr`](https://doc.rust-lang.org/std/net/struct.SocketAddr.html)
/// y un estado [`EstadoCliente`](../estadocliente/enum.EstadoCliente.html).
pub struct Cliente {
    nombre: Option<String>,
    socket: TcpStream,
    direccion: SocketAddr,
    estado: EstadoCliente,
}

impl Cliente {

    /// Crea una nueva instancia de un cliente, con un socket y dirección IP.
    /// El estado por omisión de todos los clientes es
    /// [`ACTIVE`](../estadocliente/enum.EstadoCliente.html#variant.ACTIVE).
    pub fn new(nombre: Option<String>, socket: TcpStream, direccion: SocketAddr) -> Cliente {
        Cliente {
            nombre: nombre,
            socket: socket,
            direccion: direccion,
            estado: EstadoCliente::ACTIVE
        }
    }

    /// Regresa el nombre del cliente, el cual puede no estar definido.
    pub fn get_nombre(&self) -> &Option<String> {
        &self.nombre
    }

    /// Define el nombre del cliente.
    pub fn set_nombre(&mut self, nuevo_nombre: &str) {
        self.nombre = Some(nuevo_nombre.to_owned());
    }

    /// Regresa el socket de comunicación.
    pub fn get_socket(&self) -> &TcpStream {
        &self.socket
    }

    /// Define el socket de comunicación.
    pub fn set_socket(&mut self, socket: TcpStream) {
        self.socket = socket;
    }

    /// Regresa la dirección IP del cliente.
    pub fn get_direccion(&self) -> SocketAddr {
        self.direccion
    }

    /// Define la dirección IP del cliente.
    pub fn set_direccion(&mut self, direccion: SocketAddr) {
        self.direccion = direccion
    }

    /// Regresa el estado del cliente.
    pub fn get_estado(&self) -> &EstadoCliente {
        &self.estado
    }

    /// Define el estado del cliente.
    pub fn set_estado(&mut self, estado: EstadoCliente) {
        self.estado = estado;
    }

    /// Permite enviar un mensaje a través de la conexión.
    pub fn enviar_mensaje(&mut self, mensaje: &str) -> Result<(), Error> {
        let mensaje = mensaje.as_bytes();
        self.socket.write(&mensaje[..])?;
        self.socket.flush()?;
        Ok(())
    }

    /// Provoca que el socket de comunicación se cierre. Eso no implica que el
    /// cliente ya no esté en memoria.
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


impl PartialEq for Cliente {

    fn eq(&self, other: &Cliente) -> bool {
        self.direccion == other.direccion || self.nombre == other.nombre
    }

    fn ne(&self, other: &Cliente) -> bool {
        self.direccion != other.direccion && self.nombre != other.nombre
    }
}
