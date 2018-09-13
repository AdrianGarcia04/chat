use std::net::{TcpStream, SocketAddr};

pub struct Sala {
    nombre: String,
    miembros: Vec<TcpStream>,
    direccion_propietario: SocketAddr,
}

impl Sala {

    pub fn new(nombre: String, direccion_propietario: SocketAddr) -> Sala {
        Sala {
            nombre: nombre,
            miembros: Vec::new(),
            direccion_propietario: direccion_propietario,
        }
    }

    pub fn get_nombre(&self) -> &str {
        &self.nombre[..]
    }

    pub fn agregar_miembro(&mut self, socket: &TcpStream) {
        let socket_clon = socket.try_clone().expect("Error al clonar socket");
        self.miembros.push(socket_clon);
    }
}

impl Clone for Sala {
     fn clone(&self) -> Self {
        let mut miembros = Vec::new();
        for miembro in self.miembros.iter() {
            let copia = miembro.try_clone().expect("Error al clonar");
            miembros.push(copia);
        }
        Sala {
            nombre: self.nombre.clone(),
            miembros: miembros,
            direccion_propietario: self.direccion_propietario.clone(),
        }
    }
 }
