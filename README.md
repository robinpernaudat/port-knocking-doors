# port-knocking-doors
It's a binary to run as a daemon that allows port knocking.

# Intro
Basically, it listens on some UDP ports.
When a client sends a datagram on several of these ports, following a certain sequence, this program opens a network port for a predefined period of time for the client's IP.

When used as a client, it can execute the sequence. It also allows to define an interval for the server to keep its port open. 


This project is developed in Rust: to mitigate a family of flaws based on buffer overflow (that C programs suffer), to allow to build easily a binary for any target (raspberry pi, other ARM cards, NAS Synology, Android, conventional server...); to build a binary without dependencies; to improve my skills in this Rust language; I'm tired of using buggy batch or python scripts.

# Why
The purpose of port knocking is to reduce the attack surface of a server to an opportunistic hacker. Your SSH or SMB port is closed by default for web clients. Legitimate clients proceed with the port knocking sequence, which opens the door for them.

# How
The router of the internet connection redirects UDP ports to our server (it could be in the DMZ). It also routes the TCP ports to cover.

The daemon listens to the concerned UDP ports looking for the sequence. It checks the firewall of the server and opens the port for the client (for the client IP only).

This program works together with Firewalld. So it is supposed to work under Linux. It's up to the user to run it in the background or to do what is needed with systemd or another scheduler.

# Usage
## help message
```text
port-knocking-doors 1.0.0
This software manage the port knocking

USAGE:
    port-knocking-doors [OPTIONS]

OPTIONS:
    -h, --help                     Print help information
    -m, --magic-seq <MAGIC_SEQ>    this is the magic sequence (between 5 and 20 UDP ports) in the
                                   format "port_num,port_num,port_num,port_num,port_num" (example:
                                   "10001,10002,10003,10004,10005") [default: #####]
    -p, --ports <PORTS>            this is the ports that we want to control. The format is like
                                   this "port1,port2,..." (examples: "22", "22,80,8080") [default:
                                   #####]
    -V, --version                  Print version information
```
## env
You can also use the environnement variables:
 - `KNOCKER_SEQ` to define the magic sequence at least 5 UDP ports (ex `export KNOCKER_SEQ=1,2,3,4,5,6,7`)
 - `KNOCKER_PORTS` to define the list of ports to control (ex `export KNOCKER_PORTS=22,80,10080`)

These two variables are only used in the absence of the argument concerned in the command line.

# history
|when|what|
|-|-|
|2022-02-25 | v 1.0.0 |
|2022-02-11 | This repo is just started. Not buildable yet ! |
