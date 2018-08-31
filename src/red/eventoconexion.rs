use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EventoConexion {
    EmpiezaConexion,
    Mensaje,
    TerminaConexion,
    CambiarSala,
    NuevaSala,
    EventoInvalido,
}

impl FromStr for EventoConexion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EmpiezaConexion" => Ok(EventoConexion::EmpiezaConexion),
            "Mensaje" => Ok(EventoConexion::Mensaje),
            "TerminaConexion" => Ok(EventoConexion::TerminaConexion),
            "EventoInvalido" => Ok(EventoConexion::EventoInvalido),
            "CambiarSala" => Ok(EventoConexion::CambiarSala),
            "NuevaSala" => Ok(EventoConexion::NuevaSala),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventoConexion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
