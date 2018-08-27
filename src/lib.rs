pub mod red;

#[cfg(test)]
mod tests {
    use red::{servidor::Servidor, cliente::Cliente, eventoservidor::EventoServidor};
    use std::thread;

    #[test]
    fn test_servidor() {
        let puerto = "7878";
        let n = 1000;
        let mut servidor = Servidor::new(puerto);

        // Generando n clientes distintos
        for _ in 1..n {
            let escucha = servidor.nuevo_escucha();
            thread::spawn(move || {
                let mut cliente = Cliente::new(Some("test".to_string()), Some("127.0.0.1:".to_string() + puerto), None, None);

                let evento = escucha.recv();
                assert_eq!(evento, Ok(EventoServidor::ServidorArriba));

                cliente.conectar();
                let evento = escucha.recv();
                assert_eq!(evento, Ok(EventoServidor::NuevoCliente));
            });

        }

        thread::spawn(move || {
            servidor.comenzar();
        });

    }

}
