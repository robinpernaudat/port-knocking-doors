# port-knocking-doors
It's a binary to run as a daemon that allows port knocking.

# Intro
Basically, it listens on some UDP ports.
When a client sends a datagram on several of these ports, following a certain sequence, this program opens a network port for a predefined period of time for the client's IP.

When used as a client, it can execute the sequence. It also allows to define an interval for the server to keep its port open. 

# Why
The purpose of port knocking is to reduce the attack surface of a server to an opportunistic hacker. Your SSH or SMB port is closed by default for web clients. Legitimate clients proceed with the port knocking sequence, which opens the door for them.

# How
The router of the internet connection redirects UDP ports to our server. It also routes the TCP port to cover.

The daemon listens to the concerned UDP ports looking for the sequence. It checks the firewall of the server and opens the port for the client (for its IP).

This project is developed in Rust: to mitigate a family of flaws based on buffer overflow, to allow to build easily a binary for any target (raspberry pi, other ARM cards, NAS Synology, Android, conventional server...); to build a binary without dependencies; to improve my skills in this language.

# historique
2021-02-11 This repo is just started. Not buildable yet !
