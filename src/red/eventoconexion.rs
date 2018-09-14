use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug)]
pub enum EventoConexion {
    IDENTIFY,
    STATUS,
    USERS,
    MESSAGE,
    PUBLICMESSAGE,
    CREATEROOM,
    INVITE,
    JOINROOM,
    ROOMESSAGE,
    DISCONNECT,
    INVALID,
    ERROR,
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
            "ROOMESSAGE" => Ok(EventoConexion::ROOMESSAGE),
            "DISCONNECT" => Ok(EventoConexion::DISCONNECT),
            "INVALID" => Ok(EventoConexion::INVALID),
            "ERROR" => Ok(EventoConexion::ERROR),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EventoConexion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
