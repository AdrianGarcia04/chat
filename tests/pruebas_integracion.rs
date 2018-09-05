extern crate chat;

use chat::red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion,
        util};
use std::thread;
use std::net::TcpStream;

#[test]
fn t1_acepta_conexiones() {
    let puerto = "7878";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t1-servidor".into());
    let hilo_cliente = thread::Builder::new().name("t1-cliente".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente.spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("nombre_test"));
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));
    }).unwrap();
}

#[test]
fn t2_manda_mensajes() {
    let puerto = "9090";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t2-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t2-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t2-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("cliente1_test"));
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        util::mandar_evento(&cliente, EventoConexion::Mensaje);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::Mensaje);

        util::mandar_mensaje(&cliente, String::from("Mensaje del cliente 1"));
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("cliente2_test"));
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::Mensaje);

        let mensaje = util::obtener_mensaje_conexion(&cliente);
        assert_eq!(mensaje, "Mensaje del cliente 1");
    }).unwrap();
}

#[test]
fn t3_crea_salas() {
    let puerto = "7070";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t3-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t3-cliente".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("nombre_test"));
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        util::mandar_evento(&cliente, EventoConexion::NuevaSala);
        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::NuevaSala);

        util::mandar_mensaje(&cliente, String::from("sala"));
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevaSala));
    }).unwrap();
}
