use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion,
    sala::Sala, util};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use std::io::{Error, ErrorKind};

type MutexCliente = Arc<Mutex<Vec<Cliente>>>;
type MutexSala = Arc<Mutex<Vec<Sala>>>;
type CanalServidor = mpsc::Sender<EventoServidor>;

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
                self.maneja_conexion(socket, direccion_socket, tx.clone());
            }

            if let Ok(evento) = rx.try_recv() {
                self.maneja_evento_servidor(evento);
            }
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn maneja_evento_servidor(&mut self, evento: EventoServidor) {
        self.anunciar_escuchas(evento.clone());
        match evento {
            EventoServidor::ServidorAbajo => {
                self.detener();
            },
            _ => {

            }
        }
    }

    fn maneja_conexion(&mut self, socket: TcpStream, direccion_socket: SocketAddr, tx: CanalServidor) {
        let clientes = Arc::clone(&self.clientes);
        let salas = Arc::clone(&self.salas);
        thread::spawn(move || {
                match Servidor::aceptar_cliente(&clientes, &socket, direccion_socket) {
                    Ok(_) => {
                        tx.send(EventoServidor::NuevoCliente).unwrap();
                        loop {
                            let _tx = tx.clone();
                            let _socket = socket.try_clone().expect("Error al clonar el socket");
                            Servidor::reaccionar(_socket, direccion_socket, &clientes, &salas, _tx);
                        };
                    },
                    Err(error) => {
                        println!("Error al aceptar al cliente: {:?}", error);
                    }
                };
            }
        );
    }

    fn detener(&mut self) {
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
        println!("{:?}", evento);
    }

    fn matar_escuchas(&mut self) {
        self.anunciar_escuchas(EventoServidor::ServidorAbajo);
        for escucha in self.escuchas.iter() {
            drop(escucha);
        }
    }

    fn matar_clientes(&mut self) {
        let clientes = Arc::clone(&self.clientes);
        let mut clientes = clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            cliente.detener();
            drop(cliente);
        }
    }


    fn cambiar_sala(socket: &TcpStream, mutex_salas: &MutexSala, nombre_sala: String) {
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.nombre() == nombre_sala {
                sala.agregar_miembro(&socket);
            }
        }
    }

    fn crear_sala(socket: &TcpStream, mutex_salas: &MutexSala, nombre_sala: String) {
        let mut salas = mutex_salas.lock().unwrap();
        let mut sala = Sala::new(nombre_sala);
        sala.agregar_miembro(&socket);
        salas.push(sala);
    }

    fn aceptar_cliente(mutex_clientes: &MutexCliente, socket: &TcpStream,
        direccion_socket: SocketAddr) -> Result<(), Error> {

        util::mandar_evento(&socket, EventoConexion::EmpiezaConexion);
        let nombre = Servidor::obtener_nombre(socket)?;
        let _socket = socket.try_clone()?;
        let cliente = Cliente::new(nombre, _socket, direccion_socket);
        let mut clientes = mutex_clientes.lock().unwrap();
        clientes.push(cliente.clone());
        drop(clientes);

        Ok(())
    }

    fn obtener_nombre(mut socket: &TcpStream) -> Result<String, Error> {
        let nombre = util::obtener_mensaje_conexion(&mut socket);
        if nombre.len() < 1 || nombre.len() > 20 {
            Err(Error::new(ErrorKind::ConnectionRefused,
                "El nombre debe tener una longitud entre 1 y 20 caracteres"))
        }
        else {
            Ok(nombre)
        }
    }

    fn esparcir_mensaje_a_clientes(mutex_clientes: &MutexCliente, mensaje: String, _direccion_socket: SocketAddr) {
        let mut clientes = mutex_clientes.lock().unwrap();
        let mensaje = &mensaje[..];
        for cliente in clientes.iter_mut() {
            util::mandar_evento(&cliente.socket(), EventoConexion::Mensaje);
            util::mandar_mensaje(&cliente.socket(), mensaje.to_string());
        }
        drop(clientes);
    }

    fn reaccionar(socket: TcpStream, direccion_socket: SocketAddr, mutex_clientes: &MutexCliente,
        mutex_salas: &MutexSala, tx: CanalServidor) {
        match util::obtener_evento_conexion(&socket) {
            EventoConexion::Mensaje => {
                util::mandar_evento(&socket, EventoConexion::Mensaje);
                let mensaje = util::obtener_mensaje_conexion(&socket);
                Servidor::esparcir_mensaje_a_clientes(mutex_clientes, mensaje, direccion_socket);
            },
            EventoConexion::TerminaConexion => {
                tx.send(EventoServidor::ServidorAbajo).unwrap();
            },
            EventoConexion::CambiarSala => {
                util::mandar_evento(&socket, EventoConexion::CambiarSala);
                let sala = util::obtener_mensaje_conexion(&socket);
                Servidor::cambiar_sala(&socket, mutex_salas, sala);
            },
            EventoConexion::NuevaSala => {
                util::mandar_evento(&socket, EventoConexion::NuevaSala);
                let nombre_sala = util::obtener_mensaje_conexion(&socket);
                Servidor::crear_sala(&socket, mutex_salas, nombre_sala);
                tx.send(EventoServidor::NuevaSala).unwrap();
            },
            _ => {},
        }
    }
}
