use super::{cliente::Cliente, eventoservidor::EventoServidor};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::mpsc;

#[derive(Clone)]
pub struct Servidor {
    direccion: String,
    clientes: Vec<Cliente>,
    escuchas: Vec<mpsc::Sender<EventoServidor>>,
}

impl Servidor {

    pub fn new(puerto: &str) -> Servidor {
        let direccion = format!("127.0.0.1:{}", puerto);
        Servidor {
            direccion: direccion,
            clientes: Vec::new(),
            escuchas: Vec::new(),
        }
    }

    pub fn direccion(&self) -> &str {
        &self.direccion
    }

    pub fn clientes(&self) -> &[Cliente] {
        self.clientes.as_slice()
    }

    pub fn comenzar(&mut self) {
        let escucha_tcp = match TcpListener::bind(&self.direccion) {
            Ok(escucha) => escucha,
            Err(error) => {
                panic!("Ocurrió un problema al iniciar el servidor: {:?}", error);
            },
        };

        self.anunciar_escuchas(EventoServidor::ServidorArriba);
        escucha_tcp.set_nonblocking(true).expect("Error al inicializar el non-blocking");
        loop {
            if let Ok((mut socket, direccion_socket)) = escucha_tcp.accept() {
                self.aceptar_cliente(&socket, direccion_socket);
                self.anunciar_escuchas(EventoServidor::NuevoCliente);
                println!("SERVIDOR: Nuevo cliente: {:?}", direccion_socket);
            }
        }
    }

    pub fn detener(&mut self) {

    }

    pub fn nuevo_escucha(&mut self) -> mpsc::Receiver<EventoServidor> {
        let (tx, rx) = mpsc::channel::<EventoServidor>();
        self.escuchas.push(tx);
        rx
    }

    fn anunciar_escuchas(&mut self, evento: EventoServidor) {
        for escucha in &self.escuchas {
            &escucha.send(evento.clone());
        }
    }

    fn aceptar_cliente(&mut self, socket: &TcpStream, address: SocketAddr) {
        let _socket = socket.try_clone().expect("Error al clonar socket");
        let cliente = Cliente::new(None, None, Some(_socket), Some(address));
        self.clientes.push(cliente);
    }
}
