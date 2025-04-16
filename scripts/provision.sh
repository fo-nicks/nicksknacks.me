#!/bin/bash

# create service user
ssh nicksknacks.me "sudo useradd -r -s /bin/false nicksknacks"
ssh nicksknacks.me "sudo chown -R nicksknacks /var/www/nicksknacks.me"

ssh nicksknacks.me "mkdir -p /var/www/nicksknacks.me"
rsync  ./scripts/nicksknacks.me.service nicksknacks.me:/etc/systemd/system
ssh nicksknacks.me "sudo chmod 644 /etc/systemd/system/nicksknacks.me.service"