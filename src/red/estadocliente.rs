use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug)]
/// Estados de los clientes. Los estados definen en qué situación se encuentra
/// el cliente mientras participa en el chat.
pub enum EstadoCliente {
    /// Cuando el usuario está activo en el chat.
    ACTIVE,
    /// Cuando el usuario está lejos de la computadora.
    AWAY,
    /// Cuando el usuario está ocupado.
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
