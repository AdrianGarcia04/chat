use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EventoConexion {
    IDENTIFY,
    STATUS,
    USERS,
    MESSAGE,
    PUBLICMESSAGE,
    CREATEROOM,
    INVITE,
    JOINROOM,
    TerminaConexion,
    EventoInvalido,
    Desconexion,
}

impl FromStr for EventoConexion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IDENTIFY" => Ok(EventoConexion::IDENTIFY),
            "STATUS" => Ok(EventoConexion::STATUS),
            "USERS" => Ok(EventoConexion::USERS),
            "MESSAGE" => Ok(EventoConexion::MESSAGE),
            "PUBLICMESSAGE" => Ok(EventoConexion::PUBLICMESSAGE),
            "CREATEROOM" => Ok(EventoConexion::CREATEROOM),
            "INVITE" => Ok(EventoConexion::INVITE),
            "JOINROOM" => Ok(EventoConexion::JOINROOM),
            "TerminaConexion" => Ok(EventoConexion::TerminaConexion),
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
