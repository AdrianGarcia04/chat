use super::{cliente::Cliente, eventoservidor::EventoServidor, eventoconexion::EventoConexion,
    sala::Sala, util, estadocliente::EstadoCliente};

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use std::io::{Error, ErrorKind};

type MutexCliente = Arc<Mutex<Vec<Cliente>>>;
type MutexSala = Arc<Mutex<Vec<Sala>>>;
type CanalServidor = mpsc::Sender<EventoServidor>;

/// Representación abstracta del servidor.
/// Los servidores tienen una dirección IP asociada, un contador de referencias atómico
/// [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
/// que contiene un primitiva de exclusión mutua
/// [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) de un vector de clientes, y a su vez
/// uno de salas, así como un vector de escuchas y un boolean que indica si el servidor se
/// encuentra aceptando conexiones.
pub struct Servidor {
    direccion: String,
    clientes: MutexCliente,
    escuchas: Vec<CanalServidor>,
    aceptando_conexiones: bool,
    salas: MutexSala
}

impl Servidor {

    /// Crea una nueva instancia de un servidor, recibiendo un puerto y creando
    /// una dirección IP donde escuchar conexiones.
    pub fn new(puerto: &str) -> Servidor {
        let direccion = format!("0.0.0.0:{}", puerto);
        Servidor {
            direccion: direccion,
            clientes: Arc::new(Mutex::new(Vec::new())),
            escuchas: Vec::new(),
            aceptando_conexiones: false,
            salas: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Intenta enlazarse con la dirección IP creada, para posteriormente comenzar a escuchar
    /// peticiones en dicha dirección. Al recibir una petición nueva, crea un nuevo cliente
    /// y lanza un hilo de ejecución que se encargue de escuchar a dicho cliente. El método se
    /// realiza de forma repetida hasta que se modifique el valor de la variable
    /// "aceptando_conexiones".
    /// El servidor toma "pausas" de 500 milisegundos para evitar consumir recursos de manera
    /// excesiva.
    pub fn comenzar(&mut self) {
        let escucha_tcp = match TcpListener::bind(&self.direccion) {
            Ok(escucha) => escucha,
            Err(error) => {
                error!("Ocurrió un problema al iniciar el servidor: {}", error);
                panic!("{:?}", error);
            },
        };

        self.anunciar_escuchas(EventoServidor::ServidorArriba);
        self.aceptando_conexiones = true;
        escucha_tcp.set_nonblocking(true).expect("Error al inicializar el non-blocking");
        info!(target: "Servidor", "Aceptando conexiones en: {}", &self.direccion);
        while self.aceptando_conexiones {
            if let Ok((socket, direccion)) = escucha_tcp.accept() {
                let mut cliente = self.aceptar_cliente(socket, direccion);
                self.maneja_conexion(cliente);
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    /// Crea un nuevo cliente y lo guarda dentro del vector de clientes, regresando una copia
    pub fn aceptar_cliente(&mut self, socket: TcpStream, direccion: SocketAddr) -> Cliente {
        let cliente = Cliente::new(None, socket, direccion);
        let mut clientes = self.clientes.lock().unwrap();
        clientes.push(cliente.clone());
        info!(target: "Servidor", "Nuevo cliente: {}", direccion);
        cliente
    }

    /// Lanza un hilo de ejecución encargado de escuchar al cliente recibido y reaccionar
    /// dependiendo de los eventos que el cliente especifique. En caso de un error o que el
    /// mismo cliente interrumpa la conexión, el servidor lo desconecta y lo elimina
    /// de la lista de clientes.
    pub fn maneja_conexion(&mut self, cliente: Cliente) {
        let clientes = Arc::clone(&self.clientes);
        let salas = Arc::clone(&self.salas);
        thread::spawn(move || loop {
                match Servidor::reaccionar(cliente.clone(), &clientes, &salas) {
                    Ok(_) => {

                    },
                    Err(_) => {
                        warn!(target: "Servidor", "Se perdió la conexión con el cliente {}",
                                cliente.get_direccion());
                        Servidor::desconectar_cliente(&cliente, &clientes, &salas);
                        break;
                    }
                }
            }
        );
    }

    /// Detiene la ejecución del servidor, eliminando de la memoria a los clientes y los escuchas.
    pub fn detener(&mut self) {
        info!(target: "Servidor", "Desconectando servidor");
        self.eliminar_clientes();
        info!(target: "Servidor", "Clientes eliminados");
        self.eliminar_escuchas();
        info!(target: "Servidor", "Escuchas eliminados");
        self.aceptando_conexiones = false;
        info!(target: "Servidor", "Servidor desconectado");
    }

    /// Crea una nueva tupla escucha-emisor, guardando el emisor en la lista de escuchas y
    /// regresando a su correspondiente escucha.
    pub fn nuevo_escucha(&mut self) -> mpsc::Receiver<EventoServidor> {
        info!(target: "Servidor", "Creando nuevo escucha");
        let (tx, rx) = mpsc::channel::<EventoServidor>();
        self.escuchas.push(tx);
        rx
    }

    /// Anuncia a los escuchas existentens sobre un evento ocurrido en el servidor.
    pub fn anunciar_escuchas(&mut self, evento: EventoServidor) {
        info!(target: "Servidor", "Anunciando escuchas del evento: {:?}", evento);
        for escucha in &self.escuchas {
            &escucha.send(evento.clone());
        }
    }

    /// Elimina de memoria a los escuchas creados.
    pub fn eliminar_escuchas(&mut self) {
        info!(target: "Servidor", "Eliminando escuchas");
        self.anunciar_escuchas(EventoServidor::ServidorAbajo);
        for escucha in self.escuchas.iter() {
            drop(escucha);
        }
    }


    /// Elimina de memoria a los clientes creados.
    pub fn eliminar_clientes(&mut self) {
        info!(target: "Servidor", "Eliminando clientes");
        let clientes = Arc::clone(&self.clientes);
        let mut clientes = clientes.lock().unwrap();
        for cliente in clientes.iter_mut() {
            info!(target: "Servidor", "Eliminando al cliente {}", cliente.get_direccion());
            cliente.detener();
            drop(cliente);
            info!(target: "Servidor", "Cliente eliminado");
        }
    }

    /// Regresa un nombre único con longitud entre 1 - 20 caracteres dentro de un vector de argumentos.
    /// Si el nombre no cumple con alguna condición, regresa un error.
    pub fn obtener_nombre(mut argumentos: Vec<String>, mutex_clientes: &MutexCliente) -> Result<String, Error> {
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

    /// Regresa un estado válido dentro de un vector de argumentos.
    /// Si el estado no cumple con alguna condición, regresa un error.
    pub fn obtener_estado(mut argumentos: Vec<String>) -> Result<EstadoCliente, Error> {
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

    /// Determina si un nombre es único entre todos los clientes ya identificados.
    pub fn es_nombre_unico(nombre: &str, mutex_clientes: &MutexCliente) -> bool {
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

    /// Define el nuevo nombre único de un cliente.
    pub fn cambiar_nombre_usuario(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        let nombre = Servidor::obtener_nombre(argumentos, mutex_clientes)?;
        let mut clientes = mutex_clientes.lock().unwrap();
        for cliente_iter in clientes.iter_mut() {
            if cliente.eq(cliente_iter) {
                cliente_iter.set_nombre(&nombre);
                break;
            }
        }
        info!(target: "Servidor",
            "El cliente con dirección {} se identificó como {}", cliente.get_direccion(), nombre);
        let confirmacion = format!("Nombre cambiado a: {}", nombre);
        Ok(confirmacion)
    }

    /// Define el nuevo estado de un cliente.
    pub fn cambiar_estado_usuario(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        if let Some(nombre_cliente) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
            let estado = Servidor::obtener_estado(argumentos)?;
            let mut clientes = mutex_clientes.lock().unwrap();
            for cliente_iter in clientes.iter_mut() {
                if cliente.eq(cliente_iter) {
                    cliente_iter.set_estado(estado.clone());
                    break;
                }
            }
            info!(target: "Servidor",
                "{} actualizó su estado a {}", nombre_cliente, estado);
            let confirmacion = format!("Estado cambiado a: {}", estado);
            Ok(confirmacion)
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("Debes identificarte para actualizar tu estado")))
        }
    }

    /// Regresa un vector de nombres de los clientes identificados en el servidor.
    pub fn obtener_usuarios(mutex_clientes: &MutexCliente) -> Vec<String> {
        let clientes = mutex_clientes.lock().unwrap();
        let mut lista_clientes = Vec::new();
        for cliente in clientes.iter() {
            if let Some(nombre) = cliente.get_nombre() {
                lista_clientes.push(nombre.to_owned());
            }
        }
        lista_clientes
    }

    /// Envía un mensaje privado a un cliente en específico.
    pub fn envia_mensaje_privado(cliente: &Cliente, mutex_clientes: &MutexCliente, mut argumentos: Vec<String>)
        -> Result<String, Error> {
        if let Some(remitente) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
            let mut destinatario = Servidor::obtener_destinatario(mutex_clientes, &mut argumentos)?;
            let mut mensaje = argumentos.join(" ");
            if mensaje.len() > 0 {
                let remitente = format!("{}: ", remitente);
                mensaje = remitente + &mensaje;
                let confirmacion = mensaje.clone();
                destinatario.enviar_mensaje(&mensaje)?;
                Ok(confirmacion)
            }
            else {
                Err(Error::new(ErrorKind::ConnectionRefused,
                    format!("No se identificó el contenido del mensaje")))
            }
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("Debes identificarte para enviar un mensaje")))
        }
    }

    /// Envía un mensaje público a todos los clientes en el servidor.
    /// Regresa un error si el remitente no está identificado ó no se especifica un mensaje.
    pub fn envia_mensaje_publico(cliente: &Cliente, mutex_clientes: &MutexCliente, argumentos: Vec<String>)
        -> Result<String, Error> {
        if let Some(remitente) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
            let mut mensaje = argumentos.join(" ");
            if mensaje.len() > 0 {
                let remitente = format!("Público-{}: ", remitente);
                mensaje = remitente + &mensaje;
                let mut clientes = mutex_clientes.lock().unwrap();
                for cliente_iter in clientes.iter_mut() {
                    cliente_iter.enviar_mensaje(&mensaje.clone())?;
                }
                Ok(String::new())
            }
            else {
                Err(Error::new(ErrorKind::ConnectionRefused,
                    format!("No se identificó el contenido del mensaje")))
            }
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("Debes identificarte para enviar un mensaje")))
        }
    }

    /// Regresa el cliente al cual se busca enviar un mensaje. Regresa un error si no se encuentra.
    pub fn obtener_destinatario(mutex_clientes: &MutexCliente, argumentos: &mut Vec<String>)
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

    /// Crea una nueva sala, cuyo propietario es el creador de la misma.
    /// Regresa un error si la sala ya existe.
    pub fn crear_sala(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() != 0 {
            let nombre_nueva_sala = argumentos.remove(0);
            if Servidor::sala_es_unica(&nombre_nueva_sala, mutex_salas) {
                let mut salas = mutex_salas.lock().unwrap();
                let mut nueva_sala = Sala::new(&nombre_nueva_sala, cliente.get_direccion());
                nueva_sala.agregar_miembro(cliente.get_direccion(), cliente.get_socket());
                salas.push(nueva_sala);
                if let Some(nombre) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
                    info!(target: "Servidor", "{} creó la sala {}", nombre, nombre_nueva_sala);
                }
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

    /// Determina si el nombre de una nueva sala ya ha sido utilizado.
    pub fn sala_es_unica(nombre: &str, mutex_salas: &MutexSala) -> bool {
        let mut salas = mutex_salas.lock().unwrap();
        for sala in salas.iter_mut() {
            if sala.get_nombre().eq(nombre) {
                return false;
            }
        }
        true
    }

    /// Dado un vector de clientes y una sala, envía una invitación de unirse a la sala a cada cliente.
    /// Regresa un error si el remitente no se ha identificado, la sala no existe o si no
    /// se es propietario de la misma.
    pub fn enviar_invitacion(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        if let Some(nombre_anfitrion) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes){
            let mut salas = mutex_salas.lock().unwrap();
            for sala in salas.iter_mut() {
                if sala.get_nombre().eq(&nombre_sala) {
                    if sala.es_propietario(cliente.get_direccion()) {
                        let invitacion = format!("Invitación de unirse a la sala {} por {}",
                        &nombre_sala, nombre_anfitrion);
                        let mut invitados = Servidor::buscar_clientes(mutex_clientes, argumentos);
                        for cliente_iter in invitados.iter_mut() {
                            sala.invitar_miembro(cliente_iter.get_direccion(), cliente_iter.get_socket());
                            cliente_iter.enviar_mensaje(&invitacion.to_owned())?;
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
        else {
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("Debes identificarte para invitar usuarios")))
        }
    }

    /// Dado un vector de nombres, busca a los clientes con dichos nombres.
    pub fn buscar_clientes(mutex_clientes: &MutexCliente, nombres_clientes: Vec<String>) -> Vec<Cliente> {
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

    /// Permite que un cliente se una a una sala, notificando sobre su llegada al resto de los
    /// miembros de la habitación.
    /// Regresa un error si la sala no existe o no se tiene una invitación.
    pub fn unirse_a_sala(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        if let Some(nombre_cliente) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
            let nombre_sala = argumentos.remove(0);
            let mut salas = mutex_salas.lock().unwrap();
            for sala in salas.iter_mut() {
                if sala.get_nombre().eq(&nombre_sala) {
                    if sala.cliente_es_invitado(cliente.get_direccion()) {
                        let invitado = cliente.get_socket().try_clone().expect("Error al clonar socket");
                        sala.agregar_miembro(cliente.get_direccion(), &invitado);
                        info!(target: "Servidor", "{} se unió a la sala {}", nombre_cliente, nombre_sala);
                        let mensaje = format!("{} se unió a la sala {}", nombre_cliente, nombre_sala);
                        for (_, socket_miembro) in sala.get_miembros().iter_mut() {
                            util::enviar_mensaje(socket_miembro, mensaje.clone())?;
                        }
                        return Ok(String::new());
                    }
                    else {
                        return Err(Error::new(ErrorKind::ConnectionRefused,
                                "No estás invitado para unirte"));
                    }
                }
            }
            return Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"));
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused, "Debes identificarte primero"))
        }
    }

    /// Envía un mensaje a todos los miembros de una sala en específico.
    /// Regresa un error si la sala no existe o no se es miembro de la sala.
    pub fn envia_mensaje_sala(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala,
        mut argumentos: Vec<String>) -> Result<String, Error> {
        if argumentos.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionRefused,
                "No se especificó la sala"));
        }
        let nombre_sala = argumentos.remove(0);
        if let Some(remitente) = Servidor::obtener_nombre_cliente(&cliente, &mutex_clientes) {
            let mut salas = mutex_salas.lock().unwrap();
            for sala in salas.iter_mut() {
                if sala.get_nombre().eq(&nombre_sala) {
                    if sala.cliente_es_miembro(cliente.get_direccion()) {
                        let mut mensaje = argumentos.join(" ");
                        if mensaje.len() > 0 {
                            let remitente = format!("{}-{}: ", &nombre_sala, &remitente);
                            mensaje = remitente + &mensaje;
                            for (_, socket_miembro) in sala.get_miembros().iter_mut() {
                                util::enviar_mensaje(socket_miembro, mensaje.clone())?;
                            }
                            return Ok(String::new());
                        }
                        else {
                            return Err(Error::new(ErrorKind::ConnectionRefused,
                                format!("No se identificó el contenido del mensaje")));
                        }
                    }
                    else {
                        return  Err(Error::new(ErrorKind::ConnectionRefused,
                            "No eres miembro de esa sala"));
                    }
                }
            }
            Err(Error::new(ErrorKind::ConnectionRefused, "La sala no existe"))
        }
        else {
            Err(Error::new(ErrorKind::ConnectionRefused,
                format!("Debes identificarte enviar mensajes a la sala")))
        }
    }

    /// Elimina de memoria a un cliente creado en el servidor.
    pub fn desconectar_cliente(cliente: &Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala) {
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
        info!(target: "Servidor", "Se desconectó al cliente {}", cliente.get_direccion());
    }

    /// Obtiene el nombre de un cliente existente en el servidor.
    pub fn obtener_nombre_cliente(cliente: &Cliente, mutex_clientes: &MutexCliente) -> Option<String> {
        let clientes = mutex_clientes.lock().unwrap();
        let indice_cliente = clientes.iter().
                position(|cliente_iter| cliente.eq(cliente_iter)).unwrap();
        let cliente = &clientes[indice_cliente];
        match cliente.get_nombre() {
            Some(nombre) => {
                Some(nombre.to_owned())
            },
            None => {
                None
            },
        }
    }

    /// Determina que acción llevar a cabo dependiendo de los mensajes enviados por un cliente.
    /// En caso de error o que el cliente especifique su desconexión, el servidor termina la
    /// comunicación con el cliente y lo elimina de memoria.
    pub fn reaccionar(mut cliente: Cliente, mutex_clientes: &MutexCliente, mutex_salas: &MutexSala)
        -> Result<(), Error> {
        let (evento, argumentos) = util::obtener_mensaje_cliente(cliente.get_socket())?;
        match evento {
            EventoConexion::IDENTIFY => {
                let mensaje = match Servidor::cambiar_nombre_usuario(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::STATUS => {
                let mensaje = match Servidor::cambiar_estado_usuario(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje)?;
                Ok(())
            },
            EventoConexion::USERS => {
                let usuarios = Servidor::obtener_usuarios(mutex_clientes);
                cliente.enviar_mensaje(&usuarios.join(" "))?;
                Ok(())
            },
            EventoConexion::MESSAGE => {
                let mensaje = match Servidor::envia_mensaje_privado(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::PUBLICMESSAGE => {
                let mensaje = match Servidor::envia_mensaje_publico(&cliente, mutex_clientes, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::CREATEROOM => {
                let mensaje = match Servidor::crear_sala(&cliente, mutex_clientes, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::INVITE => {
                let mensaje = match Servidor::enviar_invitacion(&cliente, mutex_clientes, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::JOINROOM => {
                let mensaje = match Servidor::unirse_a_sala(&cliente, mutex_clientes, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::ROOMESSAGE => {
                let mensaje = match Servidor::envia_mensaje_sala(&cliente, mutex_clientes, mutex_salas, argumentos) {
                    Ok(confirmacion) => confirmacion,
                    Err(error) => error.to_string(),
                };
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::DISCONNECT => {
                Err(Error::new(ErrorKind::ConnectionAborted, "El cliente terminó la conexión"))
            },
            EventoConexion::INVALID => {
                let mut mensaje = String::new();
                mensaje += "Mensaje inválido, lista de mensajes válidos:\n";
                mensaje += "IDENTIFY nombre\n";
                mensaje += "STATUS [ACTIVE, AWAY, BUSY]\n";
                mensaje += "USERS\n";
                mensaje += "MESSAGE destinatario mensaje\n";
                mensaje += "PUBLICMESSAGE mensaje\n";
                mensaje += "CREATEROOM nombre_sala\n";
                mensaje += "INVITE nombre_sala usuarios...\n";
                mensaje += "JOINROOM nombre_sala\n";
                mensaje += "ROOMESSAGE nombre_sala mensaje\n";
                mensaje += "DISCONNECT\n";
                cliente.enviar_mensaje(&mensaje[..])?;
                Ok(())
            },
            EventoConexion::ERROR => {
                Err(Error::new(ErrorKind::ConnectionAborted, "La conexión fue interrumpida"))
            },
        }
    }
}
