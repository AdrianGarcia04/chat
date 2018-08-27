pub mod red;

#[cfg(test)]
mod tests {
    use red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion};
    use std::thread;
    use std::net::{TcpStream};
    use std::io::{Read, Write};

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

        let mut cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = EventoConexion::TerminaConexion.to_string();
        let bytes = evento.into_bytes();
        cliente.write(&bytes[..]).expect("Error al escribir");
        cliente.flush().expect("Error al enviar");

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

        let mut cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
            .expect("Error al conectar");

        let evento = EventoConexion::TerminaConexion.to_string();
        let bytes = evento.into_bytes();
        cliente.write(&bytes[..]).unwrap();
        cliente.flush().unwrap();

        let evento = escucha.recv();
        assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));

        TcpStream::connect("127.0.0.1:".to_string() + puerto).expect("Error al conectar");
    }

    #[test]
    fn test_acepta_conexiones() {
        let puerto = "7878";
        let mut servidor = Servidor::new(puerto);

        let escucha = servidor.nuevo_escucha();
        let handle = thread::spawn(move || {
            let evento = escucha.recv();
            assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

            let mut cliente = TcpStream::connect("127.0.0.1:".to_string() + puerto)
                .expect("Error al conectar");

            let evento = EventoConexion::EmpiezaConexion.to_string();
            let bytes = evento.into_bytes();
            cliente.write(&bytes[..]).expect("Error al escribir");
            cliente.flush().expect("Error al enviar");

            let mut buffer = [0; 180];
            match cliente.read(&mut buffer) {
                Ok(count) => {
                    if count > 0 {
                        let mensaje: Vec<u8> = buffer.to_vec().into_iter()
                            .filter(|&x| x != 00000000 as u8).collect();
                        let mensaje = String::from_utf8(mensaje).unwrap();
                        assert_eq!(mensaje, "EmpiezaConexion");
                    }
                },
                _ => {
                }
            }

            let nombre = String::from("test");
            let bytes = nombre.into_bytes();
            cliente.write(&bytes[..]).unwrap();
            cliente.flush().unwrap();

            let evento = escucha.recv();
            assert_eq!(evento, Ok(EventoServidor::NuevoCliente));

            let evento = EventoConexion::TerminaConexion.to_string();
            let bytes = evento.into_bytes();
            cliente.write(&bytes[..]).unwrap();
            cliente.flush().unwrap();
            let evento = escucha.recv();
            assert_eq!(evento, Ok(EventoServidor::ServidorAbajo));
        });

        thread::spawn(move || {
            servidor.comenzar();
        });

        handle.join().unwrap();
    }

}
