/// Contiene la representación abstracta de los clientes en el servidor.
pub mod cliente;
/// Contiene una enumeración de los posibles estados de los clientes.
pub mod estadocliente;
/// Contiene una enumeración de los eventos del protocolo de comunicación.
pub mod eventoconexion;
/// Contiene una enumeración de los eventos de los eventos del servidor.
pub mod eventoservidor;
/// Contiene la representación abstracta de las salas de chat en el servidor.
pub mod sala;
/// Contiene la estructura del servidor TCP.
pub mod servidor;
/// Módulo de utilidades para escritura y lectura en red con sockets.
pub mod util;
