use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion,
    sala::Sala, util, estadocliente::EstadoCliente};

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
                            match Servidor::reaccionar(_socket, direccion_socket, &clientes, &salas, _tx) {
                                Ok(_) => {

                                },
                                Err(error) => {
                                    println!("{:?}", error);
                                    Servidor::desconectar_cliente(direccion_socket, &clientes, &salas);
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
            cliente.detener();
            drop(cliente);
        }
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

    fn cambiar_nombre_usuario(mutex_clientes: &MutexCliente, direccion_socket: SocketAddr,
        argumentos: Vec<String>) -> Result<(), Error> {
        let nombre = Servidor::obtener_nombre(argumentos, mutex_clientes)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            if cliente.get_direccion().eq(&direccion_socket) {
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
            if cliente.get_direccion().eq(&direccion_socket) {
                cliente.set_estado(estado.clone());
            }
        }
        Ok(())
    }

    fn obtener_usuarios(mutex_clientes: &MutexCliente) -> Vec<String> {
        let clientes = mutex_clientes.lock().unwrap();
        let mut lista_clientes = Vec::new();
        for cliente in clientes.iter() {
            if let Some(nombre) = cliente.get_nombre() {
                lista_clientes.push(nombre.to_owned());
            }
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

    fn crear_sala(mutex_salas: &MutexSala, socket_propietario: &TcpStream, direccion_propietario: SocketAddr,
        mut argumentos: Vec<String>) -> Result<(), Error> {
        if argumentos.len() != 0 {
            let nombre_nueva_sala = argumentos.remove(0);
            if Servidor::sala_es_unica(&nombre_nueva_sala, mutex_salas) {
                let mut salas = mutex_salas.lock().unwrap();
                let mut nueva_sala = Sala::new(&nombre_nueva_sala, direccion_propietario);
                nueva_sala.agregar_miembro(direccion_propietario, socket_propietario);
                salas.push(nueva_sala);
                Ok(())
            }
            else {
                Err(Error::new(ErrorKind::ConnectionRefused, "Ya existe una sala con ese nombre"))
            }
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused, "No se especificó el nombre de la sala"))
        }
    }

    fn sala_es_unica(nombre: &str, mutex_salas: &MutexSala) -> bool {
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(nombre) {
                return false;
            }
        }
        true
    }

    fn enviar_invitacion(mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        direccion_propietario: SocketAddr, mut argumentos: Vec<String>) -> Result<(), Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.es_propietario(direccion_propietario) {
                    let invitados = Servidor::buscar_clientes(mutex_clientes, argumentos);
                    for cliente in invitados.iter() {
                        sala.invitar_miembro(cliente.get_direccion(), cliente.get_socket());
                    }
                    return Ok(());
                }
                else {
                    return  Err(Error::new(ErrorKind::ConnectionRefused,
                                "Debes ser propietario de la sala para invitar personas a unirse"));
                }
            }
        }
        Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
    }

    fn buscar_clientes(mutex_clientes: &MutexCliente, nombres_clientes: Vec<String>) -> Vec<Cliente> {
        let clientes = mutex_clientes.lock().unwrap();
        let mut encontrados: Vec<Cliente> = Vec::new();
        for cliente in clientes.iter() {
            if let Some(nombre) = cliente.get_nombre() {
                if nombres_clientes.contains(&nombre) {
                    encontrados.push(cliente.clone())
                }
            }
        }
        encontrados
    }

    fn unirse_a_sala(mutex_salas: &MutexSala, direccion_invitado: SocketAddr, socket_invitado: &TcpStream,
        mut argumentos: Vec<String>) -> Result<(), Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.cliente_es_invitado(direccion_invitado) {
                    let invitado = socket_invitado.try_clone().expect("Error al clonar socket");
                    sala.agregar_miembro(direccion_invitado, &invitado);
                    return Ok(());
                }
                else {
                    return  Err(Error::new(ErrorKind::ConnectionRefused,
                                "No estás invitado para unirte"));
                }
            }
        }
        Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
    }

    fn envia_mensaje_sala(mutex_salas: &MutexSala, direccion_remitente: SocketAddr,
        mut argumentos: Vec<String>) -> Result<(), Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.cliente_es_miembro(direccion_remitente) {
                    let mensaje = argumentos.join(" ");
                    if mensaje.len() > 0 {
                        for (_, socket_miembro) in sala.get_miembros().iter_mut() {
                            util::mandar_mensaje(socket_miembro, mensaje.clone())?;
                        }
                    }
                    return Ok(());
                }
                else {
                    return  Err(Error::new(ErrorKind::ConnectionRefused,
                                "No eres miembro de esa sala"));
                }
            }
        }
        Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
    }

    fn desconectar_cliente(direccion_socket: SocketAddr, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala) {
        let mut clientes = mutex_clientes.lock().unwrap();
        let indice_cliente = clientes.iter().
                position(|cliente| cliente.get_direccion().eq(&direccion_socket)).unwrap();
        let mut cliente = clientes.remove(indice_cliente);
        let mut salas = mutex_salas.lock().unwrap();
        for mut sala in salas.iter_mut() {
            if sala.cliente_es_invitado(direccion_socket) {
                sala.elimina_invitado(direccion_socket);
            }
            if sala.cliente_es_miembro(direccion_socket) {
                sala.elimina_miembro(direccion_socket);
            }
        }
        cliente.detener();
    }

    fn reaccionar(socket: TcpStream, direccion_socket: SocketAddr, mutex_clientes: &MutexCliente,
        mutex_salas: &MutexSala, _tx: CanalServidor) -> Result<(), Error> {
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
                util::mandar_mensaje(&socket, usuarios.join(" ")).unwrap();
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
            EventoConexion::CREATEROOM => {
                match Servidor::crear_sala(mutex_salas, &socket, direccion_socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Sala creada exitosamente.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::INVITE => {
                match Servidor::enviar_invitacion(mutex_clientes, mutex_salas, direccion_socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Invitaciones enviadas.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::JOINROOM => {
                match Servidor::unirse_a_sala(mutex_salas, direccion_socket, &socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Te uniste exitosamente.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::ROOMESSAGE => {
                match Servidor::envia_mensaje_sala(mutex_salas, direccion_socket, argumentos) {
                    Ok(_) => {
                        util::mandar_mensaje(&socket, "Mensaje enviado.".to_string()).unwrap();
                    },
                    Err(error) => {
                        util::mandar_mensaje(&socket, error.to_string()).unwrap();
                    }
                };
                Ok(())
            },
            EventoConexion::DISCONNECT => {
                Err(Error::new(ErrorKind::ConnectionAborted, "El cliente terminó la conexión"))
            },
            EventoConexion::INVALID => {
                let mensaje = String::from("Mensaje inválido, lista de mensajes válidos:\n
                    IDENTIFY nombre \n
                    STATUS [ACTIVE, AWAY, BUSY] \n
                    USERS \n
                    MESSAGE destinatario mensaje \n
                    PUBLICMESSAGE mensaje \n
                    CREATEROOM nombre_sala \n
                    INVITE nombre_sala usuarios... \n
                    JOINROOM nombre_sala \n
                    ROOMESSAGE nombre_sala mensaje \n
                    DISCONNECT \n
                ");
                util::mandar_mensaje(&socket, mensaje).unwrap();
                Ok(())
            },
            EventoConexion::ERROR => {
                Err(Error::new(ErrorKind::ConnectionAborted, "La conexión fue interrumpida"))
            },
        }
    }
}
