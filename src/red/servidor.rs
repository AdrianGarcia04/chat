use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::mpsc;
use std::time::Duration;
use std::io::Read;

const CHAR_NULL: u8 = 00000000;

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
                if self.aceptar_cliente(&mut socket, direccion_socket) {
                    self.anunciar_escuchas(EventoServidor::NuevoCliente);
                    println!("SERVIDOR: Nuevo cliente: {:?}", direccion_socket);
                }
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

    fn aceptar_cliente(&mut self, mut socket: &TcpStream, direccion_socket: SocketAddr) -> bool {
        let mut nombre = String::new();

        socket.set_read_timeout(Some(Duration::from_millis(100)))
            .expect("Error al dar un limite de tiempo al socket");

        let evento = obtener_evento_conexion(&mut socket);
        match evento {
            EventoConexion::EmpiezaConexion => {
                nombre = obtener_mensaje_conexion(&mut socket);
            },
            _ => {
                return false;
            }
        };

        let _socket = socket.try_clone().expect("Error al clonar socket");
        let cliente = Cliente::new(nombre, _socket, direccion_socket);
        self.clientes.push(cliente);

        socket.set_read_timeout(None)
            .expect("Error al dar un limite de tiempo al socket");
        true
    }
}

pub fn obtener_evento_conexion(mut socket: &TcpStream) -> EventoConexion {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                let mensaje = mensaje_de_buffer(&buffer);
                if let Ok(evento) = mensaje.parse::<EventoConexion>() {
                    evento
                }
                else {
                    EventoConexion::EventoInvalido
                }
            }
            else {
                EventoConexion::EventoInvalido
            }
        },
        _ => {
            EventoConexion::EventoInvalido
        }
    }
}

pub fn obtener_mensaje_conexion(mut socket: &TcpStream) -> String {
    let mut buffer = [0; 180];
    match socket.read(&mut buffer) {
        Ok(count) => {
            if count > 0 {
                mensaje_de_buffer(&buffer)
            }
            else {
                String::new()
            }
        },
        _ => {
            String::new()
        }
    }
}

pub fn mensaje_de_buffer(buffer: &[u8; 180]) -> String {
    let mensaje: Vec<u8> = buffer.to_vec().into_iter()
        .filter(|&x| x != CHAR_NULL).collect();
    let mensaje = String::from_utf8(mensaje).unwrap();
    mensaje
}
