#[macro_use]
extern crate log;

pub mod red;

#[cfg(test)]
mod tests {
    use red::{servidor::Servidor, eventoservidor::EventoServidor, eventoconexion:: EventoConexion,
            util};
    use std::thread;
    use std::net::{TcpStream};

    #[test]
    fn test_mensaje_de_buffer() {
        let mut buffer: [u8; 180] = [0; 180];
        assert_eq!("", util::mensaje_de_buffer(&buffer));

        let mut mensaje = String::new();
        for i in 0..180 {
            mensaje = mensaje + "a";
            buffer[i] = b'a';
        }
        assert_eq!(mensaje, util::mensaje_de_buffer(&buffer));

        for i in 0..180 {
            buffer[i] = util::CHAR_NULL;
        }
        assert_eq!("", util::mensaje_de_buffer(&buffer));
    }
}
