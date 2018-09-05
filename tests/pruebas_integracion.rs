extern crate chat;

use chat::red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion,
        util};
use std::thread;
use std::net::{TcpStream};

#[test]
fn test_servidor_arriba() {
    let puerto = "2020";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha = servidor.nuevo_escucha();

    thread::spawn(move || {
        servidor.comenzar();
    });

    let evento = escucha.recv();
    assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

    let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
        .expect("Error al conectar");

    util::mandar_evento(&cliente, EventoConexion::TerminaConexion);

    let evento = escucha.recv();
    assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));
}

#[test]
fn test_acepta_conexiones() {
    let puerto = "7878";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha = servidor.nuevo_escucha();

    thread::spawn(move || {
        servidor.comenzar();
    });


    thread::spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_evento(&cliente, EventoConexion::EmpiezaConexion);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("test"));

        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        util::mandar_evento(&cliente, EventoConexion::TerminaConexion);

        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));
    });
}

#[test]
fn test_manda_mensajes() {
    let puerto = "9090";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    thread::spawn(move || {
        servidor.comenzar();
    });


    thread::spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_evento(&cliente, EventoConexion::EmpiezaConexion);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("cliente1"));

        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        util::mandar_evento(&cliente, EventoConexion::Mensaje);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::Mensaje);

        util::mandar_mensaje(&cliente, String::from("Mensaje del cliente 1"));
    });

    thread::spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_evento(&cliente, EventoConexion::EmpiezaConexion);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("cliente2"));

        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::Mensaje);

        let mensaje = util::obtener_mensaje_conexion(&cliente);
        assert_eq!(mensaje, "Mensaje del cliente 1");

        util::mandar_evento(&cliente, EventoConexion::TerminaConexion);
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));
    });
}

#[test]
fn test_crea_salas() {
    let puerto = "7070";
    let mut servidor = Servidor::new("localhost", puerto);
    let escucha = servidor.nuevo_escucha();

    thread::spawn(move || {
        servidor.comenzar();
    });

    thread::spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_evento(&cliente, EventoConexion::EmpiezaConexion);

        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::EmpiezaConexion);

        util::mandar_mensaje(&cliente, String::from("cliente"));

        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

        util::mandar_evento(&cliente, EventoConexion::NuevaSala);
        let evento = util::obtener_evento_conexion(&cliente);
        assert_eq!(evento, EventoConexion::NuevaSala);

        util::mandar_mensaje(&cliente, String::from("sala"));
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::NuevaSala));

        util::mandar_evento(&cliente, EventoConexion::TerminaConexion);
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));
    });
}
