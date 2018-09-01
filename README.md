# chat
Chat usando el protocolo TCP para el curso de Modelado y Programación

## Requisitos

* [Rust](https://www.rust-lang.org/en-US/install.html)

## Compilación

```bash
$ cargo build
```

## Corriendo pruebas unitarias

```bash
$ cargo test
```

Corriendo pruebas unitarias consecutivamente:

```bash
$ cargo test -- --test-threads=1
```

## Funcionamiento

Para levantar el servidor:

```bash
$ cargo run --bin servidor [ip] [puerto]
```

Conectando un cliente:

```bash
$ cargo run --bin cliente [ip] [puerto]
```

### Comunicándose con el servidor

Creando un cliente y enviando un mensaje:
```bash
$ cargo run --bin cliente [ip] [puerto]
$ EmpiezaConexion
  Mensaje del servidor: EmpiezaConexion
$ [nombre]
$ Mensaje
  Mensaje del servidor: Mensaje
$ [mensaje]
```
