use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;

/// Estructura de las salas de chat en el servidor.
/// Las salas tienen un nombre asociado único, la dirección IP del cliente propietario
/// (quien creó la sala), un diccionario de invitados y un diccionario de miembros.
/// Solo los clientes invitados por el propietario pueden unirse como miembros a la sala.
pub struct Sala {
    nombre: String,
    direccion_propietario: SocketAddr,
    invitados: HashMap<SocketAddr, TcpStream>,
    miembros: HashMap<SocketAddr, TcpStream>,
}

impl Sala {

    /// Crea una nueva instancia de una sala, con un nombre y la dirección IP del propietario.
    pub fn new(nombre: &str, direccion_propietario: SocketAddr) -> Sala {
        Sala {
            nombre: nombre.to_owned(),
            direccion_propietario: direccion_propietario,
            invitados: HashMap::new(),
            miembros: HashMap::new(),
        }
    }

    /// Regresa el nombre de la sala.
    pub fn get_nombre(&self) -> &str {
        &self.nombre[..]
    }

    /// Define el nombre de la sala.
    pub fn set_nombre(&mut self, nombre: &str) {
        self.nombre = nombre.to_owned()
    }

    /// Regresa la dirección IP del propietario.
    pub fn get_direccion_propietario(&self) -> SocketAddr {
        self.direccion_propietario
    }

    /// Define la dirección IP del propietario.
    pub fn set_direccion_propietario(&mut self, direccion_propietario: SocketAddr) {
        self.direccion_propietario = direccion_propietario;
    }

    /// Regresa el diccionario de invitados a la sala.
    pub fn get_invitados(&mut self) -> &mut HashMap<SocketAddr, TcpStream> {
        &mut self.invitados
    }

    /// Regresa el diccionario de miembros de la sala.
    pub fn get_miembros(&mut self) -> &mut HashMap<SocketAddr, TcpStream> {
        &mut self.miembros
    }

    /// Determina si el cliente al que pertenece la dirección IP es el propietario de la sala.
    pub fn es_propietario(&self, direccion: SocketAddr) -> bool {
        self.direccion_propietario.eq(&direccion)
    }

    /// Determina si el cliente al que pertenece la dirección IP está invitado a la sala.
    pub fn cliente_es_invitado(&self, direccion: SocketAddr) -> bool {
        match self.get_socket_invitado(direccion) {
            Some(_) => true,
            None => false,
        }
    }

    /// Añade al cliente al que pertenecen el socket y la dirección IP a la lista de invitados.
    pub fn invitar_miembro(&mut self, direccion: SocketAddr, socket: &TcpStream) {
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.invitados.entry(direccion).or_insert(socket_clon);
    }

    /// Regresa el socket de comunicación del cliente invitado al que pertenece la dirección IP.
    pub fn get_socket_invitado(&self, direccion: SocketAddr) -> Option<&TcpStream> {
        self.invitados.get(&direccion)
    }

    /// Elimina de la lista de invitados al cliente al que pertenece la dirección IP.
    pub fn elimina_invitado(&mut self, direccion: SocketAddr) {
        self.invitados.remove(&direccion);
    }

    /// Determina si el cliente al que pertenece la dirección IP es miembro de la sala.
    pub fn cliente_es_miembro(&self, direccion: SocketAddr) -> bool {
        match self.get_socket_miembro(direccion) {
            Some(_) => true,
            None => false,
        }
    }

    /// Añade al cliente al que pertenecen el socket y la dirección IP a la lista de miembros.
    pub fn agregar_miembro(&mut self, direccion: SocketAddr, socket: &TcpStream) {
        self.elimina_invitado(direccion);
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.miembros.entry(direccion).or_insert(socket_clon);
    }

    /// Regresa el socket de comunicación del cliente miembro al que pertenece la dirección IP.
    pub fn get_socket_miembro(&self, direccion: SocketAddr) -> Option<&TcpStream> {
        self.miembros.get(&direccion)
    }

    /// Elimina de la lista de miembros al cliente al que pertenece la dirección IP.
    pub fn elimina_miembro(&mut self, direccion: SocketAddr) {
        self.miembros.remove(&direccion);
    }
}
