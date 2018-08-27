pub mod red;

#[cfg(test)]
mod tests {
    use red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion,
            util};
    use std::thread;
    use std::net::{TcpStream};

    #[test]
    fn test_servidor_arriba() {
        let puerto = "2020";
        let mut servidor = Servidor::new(puerto);
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
    #[should_panic]
    fn test_servidor_abajo() {
        let puerto = "4040";
        let mut servidor = Servidor::new(puerto);
        let escucha = servidor.nuevo_escucha();

        thread::spawn(move || {
            servidor.comenzar();
        });

        let cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        util::mandar_evento(&cliente, EventoConexion::TerminaConexion);

        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));

        TcpStream::connect("127.0.0.1:".to_string() + puerto).expect("Error al conectar");
        // Should panic
    }

    #[test]
    fn test_acepta_conexiones() {
        let puerto = "7878";
        let mut servidor = Servidor::new(puerto);
        let escucha = servidor.nuevo_escucha();

        thread::spawn(move || {
            servidor.comenzar();
        });


        let handle = thread::spawn(move || {
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

        handle.join().unwrap();
    }

}
