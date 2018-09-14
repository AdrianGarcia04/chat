use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;

pub struct Sala {
    nombre: String,
    miembros: HashMap<SocketAddr, TcpStream>,
    direccion_propietario: SocketAddr,
    invitados: HashMap<SocketAddr, TcpStream>
}

impl Sala {

    pub fn new(nombre: String, direccion_propietario: SocketAddr) -> Sala {
        Sala {
            nombre: nombre,
            miembros: HashMap::new(),
            direccion_propietario: direccion_propietario,
            invitados: HashMap::new(),
        }
    }

    pub fn get_nombre(&self) -> &str {
        &self.nombre[..]
    }

    pub fn get_miembros(&mut self) -> &mut HashMap<SocketAddr, TcpStream> {
        &mut self.miembros
    }

    pub fn agregar_miembro(&mut self, direccion_miembro: SocketAddr, socket_miembro: &TcpStream) {
        let socket_clon = socket_miembro.try_clone().expect("Error al clonar socket");
        self.miembros.entry(direccion_miembro).or_insert(socket_clon);
    }

    pub fn es_propietario(&self, direccion: SocketAddr) -> bool {
        self.direccion_propietario.eq(&direccion)
    }

    pub fn invitar_miembro(&mut self, direccion_invitado: SocketAddr, socket_invitado: &TcpStream) {
        let socket_clon = socket_invitado.try_clone().expect("Error al clonar socket");
        self.invitados.entry(direccion_invitado).or_insert(socket_clon);
    }

    pub fn get_invitado(&self, direccion_invitado: SocketAddr) -> Option<&TcpStream> {
        self.invitados.get(&direccion_invitado)
    }

    pub fn get_miembro(&self, direccion_miembro: SocketAddr) -> Option<&TcpStream> {
        self.miembros.get(&direccion_miembro)
    }

    pub fn cliente_esta_invitado(&self, direccion_invitado: SocketAddr) -> bool {
        match self.get_invitado(direccion_invitado) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn cliente_es_miembro(&self, direccion_miembro: SocketAddr) -> bool {
        match self.get_miembro(direccion_miembro) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn elimina_miembro(&mut self, direccion_miembro: SocketAddr) {
        self.miembros.remove(&direccion_miembro);
    }

    pub fn elimina_invitado(&mut self, direccion_invitado: SocketAddr) {
        self.invitados.remove(&direccion_invitado);
    }
}

impl Clone for Sala {
     fn clone(&self) -> Self {
        let mut miembros = HashMap::new();
        for (direccion, socket) in self.miembros.iter() {
            let socket = socket.try_clone().expect("Error al clonar socket");
            miembros.insert(direccion.clone(), socket);
        }
        let mut invitados = HashMap::new();
        for (direccion, socket) in self.invitados.iter() {
            let socket = socket.try_clone().expect("Error al clonar socket");
            invitados.insert(direccion.clone(), socket);
        }
        Sala {
            nombre: self.nombre.clone(),
            miembros: miembros,
            direccion_propietario: self.direccion_propietario.clone(),
            invitados: invitados,
        }
    }
 }
