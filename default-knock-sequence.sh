#!/bin/sh

echo "knock" | nc -u -w 1 127.0.0.1 16001
echo "knock" | nc -u -w 1 127.0.0.1 16002
echo "knock" | nc -u -w 1 127.0.0.1 16003
echo "knock" | nc -u -w 1 127.0.0.1 16004
echo "knock" | nc -u -w 1 127.0.0.1 16005

echo "firewall-cmd --info-zone=knock-access"