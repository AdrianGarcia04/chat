use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion,
    sala::Sala, util};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type MutexCliente = Arc<Mutex<Vec<Cliente>>>;
type MutexSala = Arc<Mutex<Vec<Sala>>>;
type CanalServidor = mpsc::Sender<EventoServidor>;

#[derive(Clone)]
pub struct Servidor {
    direccion: String,
    clientes: MutexCliente,
    escuchas: Vec<CanalServidor>,
    aceptando_conexiones: bool,
    salas: MutexSala
}

impl Servidor {


    pub fn new(ip: &str, puerto: &str) -> Servidor {
        let direccion = format!("{}:{}", ip, puerto);
        Servidor {
            direccion: direccion,
            clientes: Arc::new(Mutex::new(Vec::new())),
            escuchas: Vec::new(),
            aceptando_conexiones: false,
            salas: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn direccion(&self) -> &str {
        &self.direccion
    }

    pub fn clientes(&self) -> MutexCliente {
        Arc::clone(&self.clientes)
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
        escucha_tcp.set_nonblocking(true).expect("Error al inicializar el non-blocking");
        let (tx, rx) = mpsc::channel::<EventoServidor>();

        while self.aceptando_conexiones {
            if let Ok((socket, direccion_socket)) = escucha_tcp.accept() {
                let clientes = Arc::clone(&self.clientes);
                let salas = Arc::clone(&self.salas);
                let tx_hilo = tx.clone();
                thread::spawn(move || loop {
                    let _tx = tx_hilo.clone();
                    let _socket = socket.try_clone().expect("Error al clonar el socket");
                    let reaccion = Servidor::obtener_reaccion(_socket, direccion_socket);
                    reaccion(&clientes, &salas, _tx);
                });
            }

            if let Ok(evento) = rx.try_recv() {
                self.anunciar_escuchas(evento.clone());
                match evento {
                    EventoServidor::ServidorAbajo => {
                        self.detener();
                    },
                    _ => {

                    }
                }
            }
        }
    }

    pub fn detener(&mut self) {
        self.matar_clientes();
        self.matar_escuchas();
        self.aceptando_conexiones = false;
        self.anunciar_escuchas(EventoServidor::ServidorAbajo);
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
        println!("{:?}", evento);
    }

    fn matar_escuchas(&mut self) {
        self.anunciar_escuchas(EventoServidor::ServidorAbajo);
        for escucha in self.escuchas.iter() {
            drop(escucha);
        }
    }

    fn matar_clientes(&mut self) {
        let mut clientes = Arc::clone(&self.clientes);
        let mut clientes = clientes.lock().unwrap();
        let mut clientes = &mut *clientes;
        for cliente in clientes.iter_mut() {
            cliente.detener();
            drop(cliente);
        }
    }


    fn cambiar_sala(socket: &TcpStream, mutex_salas: &MutexSala, nombre_sala: String) {
        let mut salas = mutex_salas.lock().unwrap();
        let mut salas = &mut *salas;
        for sala in salas.iter_mut() {
            if sala.nombre() == nombre_sala {
                sala.agregar_miembro(&socket);
            }
        }
    }

    fn crear_sala(socket: &TcpStream, mutex_salas: &MutexSala, nombre_sala: String) {
        let mut salas = mutex_salas.lock().unwrap();
        let mut salas = &mut *salas;
        let mut sala = Sala::new(nombre_sala);
        sala.agregar_miembro(&socket);
        salas.push(sala);
    }

    fn aceptar_cliente(mutex_clientes: &MutexCliente, mut socket: &TcpStream, direccion_socket: SocketAddr) -> bool {
        let mut clientes = mutex_clientes.lock().unwrap();
        let mut clientes = &mut *clientes;
        util::mandar_evento(&socket, EventoConexion::EmpiezaConexion);
        let nombre = util::obtener_mensaje_conexion(&mut socket);

        let _socket = socket.try_clone().expect("Error al clonar socket");
        let cliente = Cliente::new(nombre, _socket, direccion_socket);
        clientes.push(cliente);
        drop(clientes);

        true
    }


    fn esparcir_mensaje_a_clientes(mutex_clientes: &MutexCliente, mensaje: String, direccion_socket: SocketAddr) {
        let mut clientes = mutex_clientes.lock().unwrap();
        let mut clientes = &mut *clientes;
        let mensaje = &mensaje[..];
        for cliente in clientes.iter_mut() {
            util::mandar_evento(&cliente.socket(), EventoConexion::Mensaje);
            util::mandar_mensaje(&cliente.socket(), mensaje.to_string());
        }
        drop(clientes);
    }

    fn obtener_reaccion(socket: TcpStream, direccion_socket: SocketAddr)
        -> Box<Fn(&MutexCliente, &MutexSala, CanalServidor) + Send> {
        match util::obtener_evento_conexion(&socket) {
            EventoConexion::EmpiezaConexion => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| {
                    let aceptado = Servidor::aceptar_cliente(mutex_clientes, &socket, direccion_socket);
                    if aceptado {
                        tx.send(EventoServidor::NuevoCliente).unwrap();
                    }
                })
            },
            EventoConexion::Mensaje => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| {
                    util::mandar_evento(&socket, EventoConexion::Mensaje);
                    let mensaje = util::obtener_mensaje_conexion(&socket);
                    Servidor::esparcir_mensaje_a_clientes(mutex_clientes, mensaje, direccion_socket);
                })
            },
            EventoConexion::TerminaConexion => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| {
                    tx.send(EventoServidor::ServidorAbajo).unwrap();
                })
            },
            EventoConexion::CambiarSala => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| {
                    util::mandar_evento(&socket, EventoConexion::CambiarSala);
                    let sala = util::obtener_mensaje_conexion(&socket);
                    Servidor::cambiar_sala(&socket, mutex_salas, sala);
                })
            },
            EventoConexion::NuevaSala => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| {
                    util::mandar_evento(&socket, EventoConexion::NuevaSala);
                    let nombre_sala = util::obtener_mensaje_conexion(&socket);
                    Servidor::crear_sala(&socket, mutex_salas, nombre_sala);
                    tx.send(EventoServidor::NuevaSala).unwrap();
                })
            },
            _ => {
                Box::new(move |mutex_clientes: &MutexCliente, mutex_salas: &MutexSala, tx: CanalServidor| ())
            }
        }
    }
}
