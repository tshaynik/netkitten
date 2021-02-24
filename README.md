netkitten
=========

A new take on netcat, written in Rust.


Installation
------------

A


Usage
-------

**netkitten** aims to improve on the usability and discoverability
of netcat by using subcommands to separate out the core functionalities.

To see the basic options:

```
❯ nk help
netkitten 0.1.0

USAGE:
    nk <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    connect
    help       Prints this message or the help of the given subcommand(s)
    listen
    scan

```

#### Make an HTTP request 
(You'll just have to settle with a redirect response, until TLS is implemented).

```
❯ nk connect rust-lang.org 80
GET / HTTP/1.1
Host: rust-lang.org

```

#### Listen concurrently

Everything entering `stdin` on the listener will be sent to all connected clients.

```

❯ nk listen -k localhost 9999

```

#### Use in a pipeline

```
❯ nk listen localhost 9999 | ncat connect google.com 80
```


Features
---------

This is an early-stage project, but it aims to add many of the features of `netcat` and `ncat`.

* [x] Basic TCP client and server
* [x] Concurrent listener
* [x] Pipelining
* [] UDP
* [] TLS
