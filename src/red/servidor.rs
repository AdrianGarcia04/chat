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
            if let Ok((socket, direccion)) = escucha_tcp.accept() {
                let mut cliente = self.aceptar_cliente(socket, direccion);
                self.maneja_conexion(cliente);
            }

            if let Ok(evento) = rx.try_recv() {
                self.maneja_evento_servidor(evento);
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn aceptar_cliente(&mut self, socket: TcpStream, direccion: SocketAddr) -> Cliente {
        let cliente = Cliente::new(None, socket, direccion);
        let mut clientes = self.clientes.lock().unwrap();
        clientes.push(cliente.clone());
        cliente
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

    fn maneja_conexion(&mut self, cliente: Cliente) {
        let clientes = Arc::clone(&self.clientes);
        let salas = Arc::clone(&self.salas);
        thread::spawn(move || loop {
                match Servidor::reaccionar(cliente.clone(), &clientes, &salas) {
                    Ok(_) => {

                    },
                    Err(error) => {
                        println!("{:?}", error);
                        Servidor::desconectar_cliente(&cliente, &clientes, &salas);
                        break;
                    }
                }
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

    fn cambiar_nombre_usuario(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        let nombre = Servidor::obtener_nombre(argumentos, mutex_clientes)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente_iter in clientes.iter_mut() {
            if cliente.eq(cliente_iter) {
                cliente_iter.set_nombre(&nombre);
            }
        }
        let confirmacion = format!("Nombre cambiado a: {}", nombre);
        Ok(confirmacion)
    }

    fn cambiar_estado_usuario(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        let estado = Servidor::obtener_estado(argumentos)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente_iter in clientes.iter_mut() {
            if cliente.eq(cliente_iter) {
                cliente_iter.set_estado(estado.clone());
            }
        }
        let confirmacion = format!("Estado cambiado a: {}", estado);
        Ok(confirmacion)
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

    fn envia_mensaje_privado(cliente: &Cliente, mutex_clientes: &MutexCliente, mut argumentos: Vec<String>)
        -> Result<String, Error> {
        let destinatario = Servidor::obtener_destinatario(mutex_clientes, &mut argumentos)?;
        let mensaje = argumentos.join(" ");
        if mensaje.len() > 0 {
            util::mandar_mensaje(destinatario.get_socket(), mensaje)?;
        }
        let confirmacion = format!("Mensaje enviado");
        return Ok(confirmacion);
    }

    fn envia_mensaje_publico(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        let mensaje = argumentos.join(" ");
        if mensaje.len() > 0 {
            let mut clientes = mutex_clientes.lock().unwrap();
            for cliente_iter in clientes.iter_mut() {
                util::mandar_mensaje(cliente_iter.get_socket(), mensaje.clone()).unwrap();
            }
        }
        let confirmacion = format!("Mensaje enviado");
        return Ok(confirmacion);
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

    fn crear_sala(cliente: &Cliente, mutex_salas: &MutexSala, mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() != 0 {
            let nombre_nueva_sala = argumentos.remove(0);
            if Servidor::sala_es_unica(&nombre_nueva_sala, mutex_salas) {
                let mut salas = mutex_salas.lock().unwrap();
                let mut nueva_sala = Sala::new(&nombre_nueva_sala, cliente.get_direccion());
                nueva_sala.agregar_miembro(cliente.get_direccion(), cliente.get_socket());
                salas.push(nueva_sala);
                let confirmacion = format!("Creación de la sala {} exitosa", nombre_nueva_sala);
                return Ok(confirmacion);
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

    fn enviar_invitacion(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.es_propietario(cliente.get_direccion()) {
                    let invitados = Servidor::buscar_clientes(mutex_clientes, argumentos);
                    for cliente_iter in invitados.iter() {
                        sala.invitar_miembro(cliente_iter.get_direccion(), cliente_iter.get_socket());
                    }
                    let confirmacion = format!("Invitaciones de la sala {} enviadas", nombre_sala);
                    return Ok(confirmacion);
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

    fn unirse_a_sala(cliente: &Cliente, mutex_salas: &MutexSala, mut argumentos: Vec<String>)
        -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.cliente_es_invitado(cliente.get_direccion()) {
                    let invitado = cliente.get_socket().try_clone().expect("Error al clonar socket");
                    sala.agregar_miembro(cliente.get_direccion(), &invitado);
                    let confirmacion = format!("Te uniste a la sala: {}", nombre_sala);
                    return Ok(confirmacion);
                }
                else {
                    return  Err(Error::new(ErrorKind::ConnectionRefused,
                                "No estás invitado para unirte"));
                }
            }
        }
        Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
    }

    fn envia_mensaje_sala(cliente: &Cliente, mutex_salas: &MutexSala, mut argumentos: Vec<String>)
        -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(&nombre_sala) {
                if sala.cliente_es_miembro(cliente.get_direccion()) {
                    let mensaje = argumentos.join(" ");
                    if mensaje.len() > 0 {
                        for (_, socket_miembro) in sala.get_miembros().iter_mut() {
                            util::mandar_mensaje(socket_miembro, mensaje.clone())?;
                        }
                    }
                    return Ok(String::from("Mensaje enviado"));
                }
                else {
                    return  Err(Error::new(ErrorKind::ConnectionRefused,
                                "No eres miembro de esa sala"));
                }
            }
        }
        Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
    }

    fn desconectar_cliente(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala) {
        let mut clientes = mutex_clientes.lock().unwrap();
        let indice_cliente = clientes.iter().
                position(|cliente_iter| cliente.eq(cliente_iter)).unwrap();
        let mut cliente = clientes.remove(indice_cliente);
        let mut salas = mutex_salas.lock().unwrap();
        for mut sala in salas.iter_mut() {
            if sala.cliente_es_invitado(cliente.get_direccion()) {
                sala.elimina_invitado(cliente.get_direccion());
            }
            if sala.cliente_es_miembro(cliente.get_direccion()) {
                sala.elimina_miembro(cliente.get_direccion());
            }
        }
        cliente.detener();
    }

    fn reaccionar(cliente: Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala)
        -> Result<(), Error> {
        let (evento, argumentos) = util::obtener_mensaje_cliente(cliente.get_socket())?;
        match evento {
            EventoConexion::IDENTIFY => {
                let mensaje = match Servidor::cambiar_nombre_usuario(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::STATUS => {
                let mensaje = match Servidor::cambiar_estado_usuario(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::USERS => {
                let usuarios = Servidor::obtener_usuarios(mutex_clientes);
                util::mandar_mensaje(cliente.get_socket(), usuarios.join(" ")).unwrap();
                Ok(())
            },
            EventoConexion::MESSAGE => {
                let mensaje = match Servidor::envia_mensaje_privado(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::PUBLICMESSAGE => {
                let mensaje = match Servidor::envia_mensaje_publico(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::CREATEROOM => {
                let mensaje = match Servidor::crear_sala(&cliente, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::INVITE => {
                let mensaje = match Servidor::enviar_invitacion(&cliente, mutex_clientes, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::JOINROOM => {
                let mensaje = match Servidor::unirse_a_sala(&cliente, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::ROOMESSAGE => {
                let mensaje = match Servidor::envia_mensaje_sala(&cliente, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
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
                util::mandar_mensaje(cliente.get_socket(), mensaje).unwrap();
                Ok(())
            },
            EventoConexion::ERROR => {
                Err(Error::new(ErrorKind::ConnectionAborted, "La conexión fue interrumpida"))
            },
        }
    }
}
