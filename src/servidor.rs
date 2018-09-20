extern crate chat;
extern crate simplelog;
extern crate clap;

use simplelog::{Level, LevelFilter, WriteLogger, Config};
use std::fs::File;
use chat::red;
use clap::{Arg, App};

fn main() {

    let matches = App::new("chat")
                    .version("1.0")
                    .author("Adrián G. <adrian.garcia04@ciencias.unam.mx>")
                    .about("Chat para el curso de Modelado y Programación")
                    .arg(Arg::with_name("puerto")
                        .value_name("PUERTO")
                        .help("El puerto donde se escuchan las peticiones")
                        .index(1)
                        .required(true))
                    .arg(Arg::with_name("salida")
                        .short("o")
                        .long("output")
                        .value_name("ARCHIVO")
                        .help("El archivo de log")
                        .takes_value(true))
                    .arg(Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Indica el nivel de log"))
                    .get_matches();

    let puerto = matches.value_of("puerto").unwrap();
    let nombre_archivo = matches.value_of("salida").unwrap_or("servidor.log");
    
    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Warn,
        3 | _ => LevelFilter::max(),
    };

    let config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Error),
        location: Some(Level::Trace),
        time_format: Some("%r"),
    };

    let archivo_log = File::create(nombre_archivo).unwrap();
    WriteLogger::init(log_level, config, archivo_log).unwrap();
    let mut servidor = red::servidor::Servidor::new(puerto);
    servidor.comenzar();
}
