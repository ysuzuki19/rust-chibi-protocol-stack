# rust-chibi-protocol-stack

Rust tiny protocol stack for self learning

chibi (ちび) means tiny in Japanese.

## Supported protocols

- icmp
- udp
- tcp

## Supported features

- icmp
  - [x] echo reply
- udp
  - [x] recv
- tcp
  - [x] connection establishment
  - [x] connection termination
  - [x] recv message

## Supported platforms

- Linux

# usage

## Start chibi-protocol-stack

```bash
$ cargo build
$ sudo target/debug/chibi-protocol-stack # `sudo` is required to bind tun device
```

## Send a message

### icmp

```bash
$ ping 192.0.2.1
```

### udp

```bash
$ nc -v -u 192.0.2.1 3000
# input message and press `Enter`
```

### tcp

```bash
$ nc -v 192.0.2.1 3000
# input message and press `Enter`
```
