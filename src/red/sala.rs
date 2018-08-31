use std::net::{TcpStream};

pub struct Sala {
    nombre: String,
    miembros: Vec<TcpStream>,
}

impl Sala {

    pub fn new(nombre: String) -> Sala {
        Sala {
            nombre: nombre,
            miembros: Vec::new(),
        }
    }

    pub fn nombre(&self) -> &str {
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
        }
    }
 }
