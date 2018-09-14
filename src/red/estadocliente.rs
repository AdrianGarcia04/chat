use std::str::FromStr;

#[derive(Clone)]
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
