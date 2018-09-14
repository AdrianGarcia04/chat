extern crate chat;
extern crate gtk;
extern crate glib;

use chat::red;
use std::thread;
use std::net::TcpStream;
use std::io::{Error, Write};
use std::cell::RefCell;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use gtk::prelude::*;
use std::path::Path;

pub struct Cliente {
    socket: Option<TcpStream>,
    escuchas: Vec<Sender<TcpStream>>,
}

impl Cliente {

    pub fn new() -> Cliente {
        Cliente {
            socket: None,
            escuchas: Vec::new(),
        }
    }

    pub fn conectar(&mut self, direccion: &str) -> Result<(), Error> {
        let socket = TcpStream::connect(direccion)?;
        self.socket = Some(socket);
        Ok(())
    }

    pub fn escribe(&mut self, mensaje: &str) {
        let bytes = mensaje.to_string().into_bytes();
        if let Some(ref mut socket) = self.socket {
            socket.write(&bytes[..]).unwrap();
            socket.flush().unwrap();
        }
    }

    pub fn nuevo_escucha(&mut self) -> Receiver<TcpStream> {
        let (tx, rx) = mpsc::channel();
        self.escuchas.push(tx);
        rx
    }

    pub fn enviar_clon_a_escuchas(&mut self) {
        if let Some(ref mut socket) = self.socket {
            for escucha in &self.escuchas {
                escucha.send(socket.try_clone().expect("Error al clonar socket")).unwrap();
            }
        }
    }
}

thread_local!(
    static GLOBAL: RefCell<Option<(gtk::TextBuffer, Receiver<String>)>> = RefCell::new(None);
);

fn recibir() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref buffer, ref rx)) = *global.borrow() {
            if let Ok(mensaje) = rx.try_recv() {
                let iter_in = buffer.get_start_iter();
                let iter_fin = buffer.get_end_iter();
                if let Some(historial) = buffer.get_text(&iter_in, &iter_fin, true) {
                    let mut texto;
                    if historial.len() > 1 {
                        texto = format!("{}\n{}", historial, mensaje);
                    }
                    else {
                        texto = mensaje;
                    }
                    buffer.set_text(&texto);
                }
            }
        }
    });
    glib::Continue(false)
}

fn main() {
    if gtk::init().is_err() {
        println!("Error al inicializar GTK.");
        return;
    }

    gtk::Window::set_default_icon_from_file(Path::new("./src/ui/rust_logo.png")).unwrap();
    let dialog_glade = include_str!("ui/dialog.glade");
    let builder = gtk::Builder::new_from_string(dialog_glade);

    let dialogo: gtk::Dialog = builder.get_object("dialog").unwrap();
    let boton_conectar: gtk::Button = builder.get_object("boton_conectar").unwrap();
    let input_direccion: gtk::Entry = builder.get_object("input_direccion").unwrap();

    let dialogo_error_glade = include_str!("ui/dialog_error.glade");
    let builder = gtk::Builder::new_from_string(dialogo_error_glade);
    let dialog_error: gtk::Dialog = builder.get_object("dialog_error").unwrap();

    let boton_reconectar: gtk::Button = builder.get_object("boton_reconectar").unwrap();

    let chat_glade = include_str!("ui/chat.glade");
    let builder = gtk::Builder::new_from_string(chat_glade);
    let window: gtk::Window = builder.get_object("ventana_chat").unwrap();

    let boton_enviar: gtk::Button = builder.get_object("boton_enviar").unwrap();
    let input_mensaje: gtk::Entry = builder.get_object("input_mensaje").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    dialogo.show();

    dialogo.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let cliente = Arc::new(Mutex::new(Cliente::new()));
    let cliente_ref = Arc::clone(&cliente);

    let boton_conectar_clon = boton_conectar.clone();
    input_direccion.connect_activate(move |_| {
        boton_conectar_clon.emit_activate();
    });

    let dialog_error_clon = dialog_error.clone();
    boton_conectar.connect_clicked(move |_| {
        if let Some(direccion) = input_direccion.get_text() {
            let mut cliente = cliente_ref.lock().unwrap();
            match cliente.conectar(&direccion) {
                Ok(_) => {
                    cliente.enviar_clon_a_escuchas();
                    dialogo.hide();
                    window.show_all();
                },
                Err(_) => {
                    dialog_error_clon.show();
                    input_direccion.set_text("");
                }
            }
        }
    });

    let dialog_error_clon = dialog_error.clone();
    boton_reconectar.connect_clicked(move |_| {
        dialog_error_clon.hide();
    });

    let boton_enviar_clon = boton_enviar.clone();
    input_mensaje.connect_activate(move |_| {
        boton_enviar_clon.emit_activate();
    });

    let cliente_ref = Arc::clone(&cliente);
    boton_enviar.connect_clicked(move |_| {
        let mut cliente = cliente_ref.lock().unwrap();
        if let Some(mensaje) = input_mensaje.get_text() {
            input_mensaje.set_text("");
            cliente.escribe(&mensaje);
        }
        drop(cliente);
    });

    let (tx2, rx2) = mpsc::channel();
    let lista_mensajes: gtk::TextView = builder.get_object("sala_principal_mensajes").unwrap();
    GLOBAL.with(|global| {
        *global.borrow_mut() = Some((lista_mensajes.get_buffer().expect("Error al obtener buffer del text view"),
                                    rx2))
    });

    let mut _cliente = cliente.lock().unwrap();
    let rx = _cliente.nuevo_escucha();
    drop(_cliente);
    thread::spawn(move || {
        let cliente = rx.recv().unwrap();
        loop {
            if let Ok(mensaje) = red::util::obtener_mensaje_conexion(&cliente) {
                tx2.send(mensaje).unwrap();
                glib::idle_add(recibir);
            }
            else {
                break;
            }
        }
    });
    gtk::main();
}
