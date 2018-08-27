pub mod red;

#[cfg(test)]
mod tests {
    use red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion};
    use std::thread;
    use std::net::{TcpStream};
    use std::io::Write;

    #[test]
    fn test_servidor() {
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
            cliente.write(&bytes[..]).unwrap();
            cliente.flush().unwrap();

            let nombre = String::from("test");
            let bytes = nombre.into_bytes();
            cliente.write(&bytes[..]).unwrap();
            cliente.flush().unwrap();

            let evento = escucha.recv();
            assert_eq!(evento, Ok(EventoServidor::NuevoCliente));
        });

        thread::spawn(move || {
            servidor.comenzar();
        });

        handle.join().unwrap();

    }

}
