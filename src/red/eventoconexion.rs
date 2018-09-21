use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug)]
/// Eventos del protocolo de comunicación. Los eventos permiten saber las acciones que
/// los clientes buscan realizar dentro del chat, enviando dichos eventos como cadenas,
/// donde el servidor las interpreta y reacciona a ellas.
pub enum EventoConexion {
    /// Darse a conocer al servidor con un nombre de usuario.
    IDENTIFY,
    /// Asignarse un estado dentro de los disponibles
    /// [`EstadoCliente`](../estadocliente/enum.EstadoCliente.html).
    STATUS,
    /// Ver usuarios identificados.
    USERS,
    /// Enviar un mensaje privado.
    MESSAGE,
    /// Enviar un mensaje público.
    PUBLICMESSAGE,
    /// Crear una [`Sala`](../sala/struct.Sala.html).
    CREATEROOM,
    /// Invitar usuarios a la [`Sala`](../sala/struct.Sala.html).
    INVITE,
    /// Unirse a una [`Sala`](../sala/struct.Sala.html).
    JOINROOM,
    /// Enviar mensaje a [`Sala`](../sala/struct.Sala.html).
    ROOMESSAGE,
    /// Desconectarse.
    DISCONNECT,
    /// Si el evento es inválido.
    INVALID,
    /// Si hubo un error al procesar el evento.
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
