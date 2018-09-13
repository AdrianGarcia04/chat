use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum EstadoCliente {
    ACTIVE,
    AWAY,
    BUSY,
}

impl FromStr for EstadoCliente {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(EstadoCliente::ACTIVE),
            "AWAY" => Ok(EstadoCliente::AWAY),
            "BUSY" => Ok(EstadoCliente::BUSY),
            _ => Err(()),
        }
    }
}

impl fmt::Display for EstadoCliente {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
