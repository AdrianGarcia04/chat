use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
/// Eventos del servidor. Los eventos permiten saber que ha ocurrido durante la ejecución
/// del servidor. Generalmente dichos eventos son enviados a los escuchas que se han
/// "suscrito" al servidor, enviando la representación en cadena de cada evento.
pub enum EventoServidor {
    /// Si el servidor empieza a aceptar conexiones.
    ServidorArriba,
    /// Si se acepta un nuevo [`Cliente`](../cliente/struct.Cliente.html)
    /// (aún no identificado con un nombre).
    NuevoCliente,
    /// Si el servidor deja de aceptar conexiones.
    ServidorAbajo,
    /// Si el evento es inválido.
    EventoInvalido,
}

impl FromStr for EventoServidor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ServidorArriba" => Ok(EventoServidor::ServidorArriba),
            "NuevoCliente" => Ok(EventoServidor::NuevoCliente),
            "ServidorAbajo" => Ok(EventoServidor::ServidorAbajo),
            "EventoInvalido" => Ok(EventoServidor::EventoInvalido),
            _ => Err(()),
        }
    }
}
