use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EventoConexion {
    EmpiezaConexion,
    EstablecerNombre,
    Mensaje,
    TerminarConexion,
    EventoInvalido,
}

impl FromStr for EventoConexion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EmpiezaConexion" => Ok(EventoConexion::EmpiezaConexion),
            "EstablecerNombre" => Ok(EventoConexion::EstablecerNombre),
            "Mensaje" => Ok(EventoConexion::Mensaje),
            "TerminarConexion" => Ok(EventoConexion::TerminarConexion),
            "EventoInvalido" => Ok(EventoConexion::EventoInvalido),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventoConexion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
