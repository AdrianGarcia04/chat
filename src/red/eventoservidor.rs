use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum EventoServidor {
    ServidorArriba,
    NuevoCliente,
    ServidorAbajo,
    NuevaSala,
    EventoInvalido,
}

impl FromStr for EventoServidor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ServidorArriba" => Ok(EventoServidor::ServidorArriba),
            "NuevoCliente" => Ok(EventoServidor::NuevoCliente),
            "ServidorAbajo" => Ok(EventoServidor::ServidorAbajo),
            "NuevaSala" => Ok(EventoServidor::NuevaSala),
            "EventoInvalido" => Ok(EventoServidor::EventoInvalido),
            _ => Err(()),
        }
    }
}
