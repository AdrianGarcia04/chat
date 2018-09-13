use std::net::{TcpStream, SocketAddr};

pub struct Sala {
    nombre: String,
    miembros: Vec<TcpStream>,
    direccion_propietario: SocketAddr,
    invitados: Vec<SocketAddr>
}

impl Sala {

    pub fn new(nombre: String, direccion_propietario: SocketAddr) -> Sala {
        Sala {
            nombre: nombre,
            miembros: Vec::new(),
            direccion_propietario: direccion_propietario,
            invitados: Vec::new(),
        }
    }

    pub fn get_nombre(&self) -> &str {
        &self.nombre[..]
    }

    pub fn agregar_miembro(&mut self, socket: &TcpStream) {
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.miembros.push(socket_clon);
    }

    pub fn es_propietario(&self, direccion: SocketAddr) -> bool {
        self.direccion_propietario.eq(&direccion)
    }

    pub fn invitar_miembro(&mut self, direccion_invitado: SocketAddr) {
        self.invitados.push(direccion_invitado);
    }
}

impl Clone for Sala {
     fn clone(&self) -> Self {
        let mut miembros = Vec::new();
        for miembro in self.miembros.iter() {
            let copia = miembro.try_clone().expect("Error al clonar");
            miembros.push(copia);
        }
        let mut invitados = Vec::new();
        for invitado in self.invitados.iter() {
            let copia = invitado.clone();
            invitados.push(copia);
        }
        Sala {
            nombre: self.nombre.clone(),
            miembros: miembros,
            direccion_propietario: self.direccion_propietario.clone(),
            invitados: invitados,
        }
    }
 }
