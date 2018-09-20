extern crate chat;

use chat::red::{servidor::Servidor, eventoservidor::EventoServidor, util};
use std::{time, thread};
use std::net::TcpStream;

#[test]
fn t1_acepta_conexiones() {
    let puerto = "9090";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t1-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t1-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t1-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No se especificó el nombre".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };
        thread::sleep(time::Duration::from_secs(2));
    }).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Ya existe un usuario con ese nombre".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(3));
}

#[test]
fn t2_asignar_estado() {
    let puerto = "9091";
    let mut servidor = Servidor::new(puerto);
    let escucha = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t2-servidor".into());
    let hilo_cliente = thread::Builder::new().name("t2-cliente".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente.spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("STATUS")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No se especificó el estado".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("STATUS ACTIVE")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Estado cambiado a: ACTIVE".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("STATUS AWAY")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Estado cambiado a: AWAY".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("STATUS BUSY")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Estado cambiado a: BUSY".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(2));
}

#[test]
fn t3_obtener_usuarios() {
    let puerto = "9092";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();
    let escucha3 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t3-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t3-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t3-cliente2".into());
    let hilo_cliente3 = thread::Builder::new().name("t3-cliente3".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };
        thread::sleep(time::Duration::from_secs(3));
    }).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };
        thread::sleep(time::Duration::from_secs(2));
    }).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    hilo_cliente3.spawn(move || {
        let evento = escucha3.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente3")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente3".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("USERS")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente1 cliente2 cliente3".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(4));
}

#[test]
fn t4_manda_mensajes_publicos() {
    let puerto = "9093";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t4-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t4-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t4-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("PUBLICMESSAGE mensaje cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Público-cliente1: mensaje cliente1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Público-cliente1: mensaje cliente1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(2));
}

#[test]
fn t5_manda_mensajes_privados() {
    let puerto = "9094";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t5-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t5-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t5-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        thread::sleep(time::Duration::from_secs(2));

        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(2));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2: Mensaje del cliente2".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("MESSAGE cliente1 Mensaje del cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No se encontró al usuario cliente1".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(2));

        util::mandar_mensaje(&cliente, String::from("MESSAGE cliente1 Mensaje del cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2: Mensaje del cliente2".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(5));
}

#[test]
fn t6_crea_salas() {
    let puerto = "9095";
    let mut servidor = Servidor::new(puerto);
    let escucha = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t6-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t6-cliente".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No se especificó el nombre de la sala".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Creación de la sala S1 exitosa".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Ya existe una sala con ese nombre".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(2));
}

#[test]
fn t7_enviar_invitaciones() {
    let puerto = "9096";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t7-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t7-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t7-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Creación de la sala S1 exitosa".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        util::mandar_mensaje(&cliente, String::from("INVITE S1 cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitaciones de la sala S1 enviadas".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitación de unirse a la sala S1 por cliente1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(3));
}

#[test]
fn t8_aceptar_invitacion() {
    let puerto = "9097";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t8-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t8-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t8-cliente2".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Creación de la sala S1 exitosa".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(2));

        util::mandar_mensaje(&cliente, String::from("INVITE S1 cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitaciones de la sala S1 enviadas".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2 se unió a la sala S1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        util::mandar_mensaje(&cliente, String::from("JOINROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No estás invitado para unirte".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(2));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitación de unirse a la sala S1 por cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("JOINROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2 se unió a la sala S1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(4));
}

#[test]
fn t9_manda_mensaje_sala() {
    let puerto = "9098";
    let mut servidor = Servidor::new(puerto);
    let escucha1 = servidor.nuevo_escucha();
    let escucha2 = servidor.nuevo_escucha();
    let escucha3 = servidor.nuevo_escucha();

    let hilo_servidor = thread::Builder::new().name("t9-servidor".into());
    let hilo_cliente1 = thread::Builder::new().name("t9-cliente1".into());
    let hilo_cliente2 = thread::Builder::new().name("t9-cliente2".into());
    let hilo_cliente3 = thread::Builder::new().name("t9-cliente3".into());

    hilo_servidor.spawn(move || {
        servidor.comenzar();
    }).unwrap();

    hilo_cliente1.spawn(move || {
        let evento = escucha1.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("CREATEROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Creación de la sala S1 exitosa".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(3));

        util::mandar_mensaje(&cliente, String::from("INVITE S1 cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitaciones de la sala S1 enviadas".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2 se unió a la sala S1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("ROOMESSAGE S1 Mensaje S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "S1-cliente1: Mensaje S1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente2.spawn(move || {
        let evento = escucha2.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente2")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente2".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(2));

        util::mandar_mensaje(&cliente, String::from("JOINROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "No estás invitado para unirte".to_string()),
            Err(_) => assert!(false),
        };

        thread::sleep(time::Duration::from_secs(1));

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Invitación de unirse a la sala S1 por cliente1".to_string()),
            Err(_) => assert!(false),
        };

        util::mandar_mensaje(&cliente, String::from("JOINROOM S1")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "cliente2 se unió a la sala S1".to_string()),
            Err(_) => assert!(false),
        };

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "S1-cliente1: Mensaje S1".to_string()),
            Err(_) => assert!(false),
        };
    }).unwrap();

    hilo_cliente3.spawn(move || {
        let evento = escucha3.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");
        util::mandar_mensaje(&cliente, String::from("IDENTIFY cliente3")).unwrap();
        match util::obtener_mensaje_conexion(&cliente) {
            Ok(mensaje) => assert_eq!(mensaje, "Nombre cambiado a: cliente3".to_string()),
            Err(_) => assert!(false),
        };

        match util::obtener_mensaje_conexion(&cliente) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }).unwrap();

    thread::sleep(time::Duration::from_secs(5));
}
