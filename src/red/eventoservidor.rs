#[derive(Clone, Debug, PartialEq)]
pub enum EventoServidor {
    ServidorArriba,
    NuevoCliente,
    ServidorAbajo,
    EventoInvalido,
}
