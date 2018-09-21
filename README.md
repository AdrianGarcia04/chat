# Chat
Chat usando el protocolo TCP para el curso de Modelado y Programación

## Requisitos

* [Rust](https://www.rust-lang.org/en-US/install.html)

## Compilación

```bash
$ cargo build
```

## Corriendo pruebas unitarias y de integración

```bash
$ cargo test
```

Corriendo pruebas unitarias consecutivamente:

```bash
$ cargo test -- --test-threads=1
```
_Nota: debido a la naturaleza de las pruebas de integración, es posible que no
se realicen correctamente en alguna ocasión, y no es posible determinar cuándo
suceda esto._

## Generando documentación

```bash
$ cargo doc --no-deps --lib --open
```

## Funcionamiento

### Levantando el servidor

```bash
$ cargo run --bin servidor <puerto>
```
### Log
Indicando un archivo log de salida (_servidor.log_ por omisión).

```bash
$ cargo run --bin servidor <puerto> -o <ARCHIVO>
```
ó

```bash
$ cargo run --bin servidor <puerto> --output <ARCHIVO>
```
Indicando un nivel de log (_aka verbosity_).

```bash
$ cargo run --bin servidor <puerto> -v
```
Verbosity  | Nivel de log
------------ | -------------
0 | _Off_
1 (_-v_) | _Info_
2 (_-vv \| -v -v_) | _Warn_
3 (_-vvv \| -v -v -v_) | _Max_

Para más información acerca del servidor:

```bash
$ cargo run --bin servidor -- -h
```

## Conectando un cliente

```bash
$ cargo run --bin cliente
```

### Protocolo de comunicación

**IDENTIFY** _username_

**STATUS** _userstatus_

**USERS**

**MESSAGE** _username messageContent_

**PUBLICMESSAGE** _messageContent_

**CREATEROOM** _roomname_

**INVITE** _roomname username1 username2..._

**JOINROOM** _roomname_

**ROOMESSAGE** _roomname messageContent_

**DISCONNECT**
