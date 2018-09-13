use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion,
    sala::Sala, util, estadocliente::EstadoCliente};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use std::io::{Error, ErrorKind};
use serde_json;

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
                            match Servidor::reaccionar(_socket, direccion_socket, &clientes, &salas, _tx) {
                                Ok(_) => {

                                },
                                Err(error) => {
                                    println!("Error: {:?}", error);
                                    break;
                                }
                            }
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
            cliente.detener().expect("Error al detener cliente");
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
        let _socket = socket.try_clone()?;
        let cliente = Cliente::new(None, _socket, direccion_socket);
        let mut clientes = mutex_clientes.lock().unwrap();
        clientes.push(cliente.clone());
        drop(clientes);

        Ok(())
    }

    fn obtener_nombre(mut argumentos: Vec<String>, mutex_clientes: &MutexCliente) -> Result<String, Error> {
        if argumentos.len() != 0 {
            let nombre = argumentos.remove(0);
            if nombre.len() < 1 || nombre.len() > 20 {
                Err(Error::new(ErrorKind::ConnectionRefused,
                    "El nombre debe tener una longitud entre 1 y 20 caracteres"))
            }
            else if Servidor::es_nombre_unico(&nombre, mutex_clientes) {
                Ok(nombre)
            }
            else {
                Err(Error::new(ErrorKind::ConnectionRefused, "Ya existe un usuario con ese nombre"))
            }
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused, "No se especificó el nombre"))
        }
    }

    fn obtener_estado(mut argumentos: Vec<String>) -> Result<EstadoCliente, Error> {
        if argumentos.len() != 0 {
            let estado = argumentos.remove(0);
            match estado.parse::<EstadoCliente>() {
                Ok(estado) => {
                    Ok(estado)
                },
                Err(_) => {
                    Err(Error::new(ErrorKind::ConnectionRefused,
                        "Proporciona un estado válido: ACTIVE, AWAY, BUSY"))
                }
            }
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused, "No se especificó el estado"))
        }
    }

    fn es_nombre_unico(nombre: &str, mutex_clientes: &MutexCliente) -> bool {
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            if let Some(nombre_cliente) = cliente.get_nombre() {
                if nombre_cliente.eq(nombre) {
                    return false;
                }
            }
        }
        true
    }

    fn esparcir_mensaje_a_clientes(mutex_clientes: &MutexCliente, mensaje: String,
        _direccion_socket: SocketAddr) -> Result<(), Error>{
        let mut clientes = mutex_clientes.lock().unwrap();
        let mensaje = &mensaje[..];
        for cliente in clientes.iter_mut() {
            // util::mandar_evento(&cliente.get_socket(), EventoConexion::Mensaje)?;
            util::mandar_mensaje(&cliente.get_socket(), mensaje.to_string())?;
        }
        drop(clientes);
        Ok(())
    }

    fn cambiar_nombre_usuario(mutex_clientes: &MutexCliente, direccion_socket: SocketAddr,
        argumentos: Vec<String>) -> Result<(), Error> {
        let nombre = Servidor::obtener_nombre(argumentos, mutex_clientes)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            if cliente.get_direccion_socket().eq(&direccion_socket) {
                cliente.set_nombre(&nombre);
            }
        }
        Ok(())
    }

    fn cambiar_estado_usuario(mutex_clientes: &MutexCliente, direccion_socket: SocketAddr,
        argumentos: Vec<String>) -> Result<(), Error> {
        let estado = Servidor::obtener_estado(argumentos)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            if cliente.get_direccion_socket().eq(&direccion_socket) {
                cliente.set_estado(estado.clone());
            }
        }
        Ok(())
    }

    fn obtener_usuarios(mutex_clientes: &MutexCliente) -> Vec<Cliente> {
        let clientes = mutex_clientes.lock().unwrap();
        let mut lista_clientes = Vec::new();
        for cliente in clientes.iter() {
            lista_clientes.push(cliente.clone());
        }
        lista_clientes
    }

    fn envia_mensaje_privado(mutex_clientes: &MutexCliente, mut argumentos: Vec<String>) -> Result<(), Error> {
        let destinatario = Servidor::obtener_destinatario(mutex_clientes, &mut argumentos)?;
        let mensaje = argumentos.join(" ");
        if mensaje.len() > 0 {
            util::mandar_mensaje(destinatario.get_socket(), mensaje)?;
        }
        Ok(())
    }
    fn envia_mensaje_publico(mutex_clientes: &MutexCliente, argumentos: Vec<String>) -> Result<(), Error> {
        let mensaje = argumentos.join(" ");
        if mensaje.len() > 0 {
            let mut clientes = mutex_clientes.lock().unwrap();
            for cliente in clientes.iter_mut() {
                util::mandar_mensaje(cliente.get_socket(), mensaje.clone()).unwrap();
            }
        }
        Ok(())
    }

    fn obtener_destinatario(mutex_clientes: &MutexCliente, argumentos: &mut Vec<String>)
        -> Result<Cliente, Error> {
        if argumentos.len() != 0 {
            let nombre_a_buscar = argumentos.remove(0);
            let clientes = mutex_clientes.lock().unwrap();
            for cliente in clientes.iter() {
                if let Some(nombre) = cliente.get_nombre() {
                    if nombre.eq(&nombre_a_buscar) {
                        return Ok(cliente.clone())
                    }
                }
            }
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("No se encontró al usuario {}", nombre_a_buscar)))
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused, "No se especificó el destinatario"))
        }
    }

    fn reaccionar(socket: TcpStream, direccion_socket: SocketAddr, mutex_clientes: &MutexCliente,
        mutex_salas: &MutexSala, tx: CanalServidor) -> Result<(), Error> {
        let (evento, argumentos) = util::obtener_mensaje_cliente(&socket)?;
        match evento {
            EventoConexion::IDENTIFY => {
                match Servidor::cambiar_nombre_usuario(mutex_clientes, direccion_socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket,
                            "Cambiaste tu nombre exitosamente.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::STATUS => {
                match Servidor::cambiar_estado_usuario(mutex_clientes, direccion_socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket,
                            "Cambiaste tu estado exitosamente.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::USERS => {
                let usuarios = Servidor::obtener_usuarios(mutex_clientes);
                util::mandar_mensaje(&socket, serde_json::to_string(&usuarios).unwrap()).unwrap();
                Ok(())
            },
            EventoConexion::MESSAGE => {
                match Servidor::envia_mensaje_privado(mutex_clientes, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Mensaje enviado.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::PUBLICMESSAGE => {
                match Servidor::envia_mensaje_publico(mutex_clientes, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Mensaje enviado.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::TerminaConexion => {
                tx.send(EventoServidor::ServidorAbajo).unwrap();
                Ok(())
            },
            EventoConexion::CambiarSala => {
                util::mandar_evento(&socket, EventoConexion::CambiarSala)?;
                let sala = util::obtener_mensaje_conexion(&socket)?;
                // Servidor::cambiar_sala(&socket, mutex_salas, sala);
                Ok(())
            },
            EventoConexion::NuevaSala => {
                util::mandar_evento(&socket, EventoConexion::NuevaSala)?;
                let nombre_sala = util::obtener_mensaje_conexion(&socket)?;
                // Servidor::crear_sala(&socket, mutex_salas, nombre_sala);
                tx.send(EventoServidor::NuevaSala).unwrap();
                Ok(())
            },
            EventoConexion::EventoInvalido => {
                Ok(())
            },
            _ => {
                Err(Error::new(ErrorKind::ConnectionAborted, "El cliente terminó la conexión"))
            },
        }
    }
}
