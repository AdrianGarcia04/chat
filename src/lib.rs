pub mod red;

#[cfg(test)]
mod tests {
    use red;

    #[test]
    fn test_servidor() {
        let puerto = "7878";
        let servidor = red::Servidor::new(puerto);
        assert_eq!(servidor.direccion(), "127.0.0.1:".to_string() + puerto);
    }
}
