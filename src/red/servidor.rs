use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::mpsc;
use std::io::{Read, Write};
use std::thread;

const CHAR_NULL: u8 = 00000000;

#[derive(Clone)]
pub struct Servidor {
    direccion: String,
    clientes: Vec<Cliente>,
    escuchas: Vec<mpsc::Sender<EventoServidor>>,
    aceptando_conexiones: bool,
}

impl Servidor {

    pub fn new(puerto: &str) -> Servidor {
        let direccion = format!("127.0.0.1:{}", puerto);
        Servidor {
            direccion: direccion,
            clientes: Vec::new(),
            escuchas: Vec::new(),
            aceptando_conexiones: false,
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
        self.aceptando_conexiones = true;
        let (tx, rx) = mpsc::channel::<Box<Fn(&mut Servidor) + Send>>();
        escucha_tcp.set_nonblocking(true).expect("Error al inicializar el non-blocking");

        while self.aceptando_conexiones {
            if let Ok((socket, direccion_socket)) = escucha_tcp.accept() {
                let _tx = tx.clone();
                thread::spawn(move || loop {
                    let _socket = socket.try_clone().expect("Error al clonar el socket");
                    let reaccion = obtener_reaccion(_socket, direccion_socket);
                    _tx.send(reaccion).unwrap();
                });
            }

            if let Ok(reaccion) = rx.try_recv() {
                reaccion(self);
            }
        }
    }

    pub fn detener(&mut self) {
        self.matar_clientes();
        self.matar_escuchas();
        self.aceptando_conexiones = false;
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
        // TODO: Desarrollar "unwraps", para que en caso de error, regresar false
        let evento = EventoConexion::EmpiezaConexion.to_string();
        let bytes = evento.into_bytes();
        socket.write(&bytes[..]).unwrap();
        socket.flush().unwrap();

        let nombre = obtener_mensaje_conexion(&mut socket);

        let _socket = socket.try_clone().expect("Error al clonar socket");
        let cliente = Cliente::new(nombre, _socket, direccion_socket);
        self.clientes.push(cliente);

        true
    }

    fn matar_escuchas(&mut self) {
        self.anunciar_escuchas(EventoServidor::ServidorAbajo);
        for escucha in self.escuchas.iter() {
            drop(escucha);
        }
    }

    fn matar_clientes(&mut self) {
        for cliente in self.clientes.iter_mut() {
            cliente.detener();
            drop(cliente);
        }
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

pub fn obtener_reaccion(socket: TcpStream, direccion_socket: SocketAddr) -> Box<Fn(&mut Servidor) + Send> {
    match obtener_evento_conexion(&socket) {
        EventoConexion::EmpiezaConexion => {
            Box::new(move |servidor: &mut Servidor| {
                let aceptado = servidor.aceptar_cliente(&socket, direccion_socket);
                if aceptado {
                    servidor.anunciar_escuchas(EventoServidor::NuevoCliente);
                }
            })
        },
        EventoConexion::Mensaje => {
            Box::new(move |_servidor: &mut Servidor| {
                // TODO: Mandar mensajes a los clientes
                ()
            })
        },
        EventoConexion::TerminaConexion => {
            Box::new(move |servidor: &mut Servidor| servidor.detener())
        },
        _ => {
            Box::new(move |_servidor: &mut Servidor| () )
        }
    }
}
