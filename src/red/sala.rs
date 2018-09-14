use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;

pub struct Sala {
    nombre: String,
    direccion_propietario: SocketAddr,
    invitados: HashMap<SocketAddr, TcpStream>,
    miembros: HashMap<SocketAddr, TcpStream>,
}

impl Sala {

    pub fn new(nombre: &str, direccion_propietario: SocketAddr) -> Sala {
        Sala {
            nombre: nombre.to_owned(),
            direccion_propietario: direccion_propietario,
            invitados: HashMap::new(),
            miembros: HashMap::new(),
        }
    }

    pub fn get_nombre(&self) -> &str {
        &self.nombre[..]
    }

    pub fn set_nombre(&mut self, nombre: &str) {
        self.nombre = nombre.to_owned()
    }

    pub fn get_direccion_propietario(&self) -> SocketAddr {
        self.direccion_propietario
    }

    pub fn set_direccion_propietario(&mut self, direccion_propietario: SocketAddr) {
        self.direccion_propietario = direccion_propietario;
    }

    pub fn get_invitados(&mut self) -> &mut HashMap<SocketAddr, TcpStream> {
        &mut self.invitados
    }

    pub fn get_miembros(&mut self) -> &mut HashMap<SocketAddr, TcpStream> {
        &mut self.miembros
    }

    pub fn es_propietario(&self, direccion: SocketAddr) -> bool {
        self.direccion_propietario.eq(&direccion)
    }

    pub fn cliente_es_invitado(&self, direccion: SocketAddr) -> bool {
        match self.get_socket_invitado(direccion) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn invitar_miembro(&mut self, direccion: SocketAddr, socket: &TcpStream) {
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.invitados.entry(direccion).or_insert(socket_clon);
    }

    pub fn get_socket_invitado(&self, direccion: SocketAddr) -> Option<&TcpStream> {
        self.invitados.get(&direccion)
    }

    pub fn elimina_invitado(&mut self, direccion: SocketAddr) {
        self.invitados.remove(&direccion);
    }

    pub fn cliente_es_miembro(&self, direccion: SocketAddr) -> bool {
        match self.get_socket_miembro(direccion) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn agregar_miembro(&mut self, direccion: SocketAddr, socket: &TcpStream) {
        self.elimina_invitado(direccion);
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.miembros.entry(direccion).or_insert(socket_clon);
    }

    pub fn get_socket_miembro(&self, direccion: SocketAddr) -> Option<&TcpStream> {
        self.miembros.get(&direccion)
    }

    pub fn elimina_miembro(&mut self, direccion: SocketAddr) {
        self.miembros.remove(&direccion);
    }
}
