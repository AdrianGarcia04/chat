#[derive(Clone, Debug, PartialEq)]
pub enum EventoConexion {
    EmpiezaConexion,
    EstablecerNombre,
    Mensaje,
    TerminarConexion,
    EventoInvalido,
}
