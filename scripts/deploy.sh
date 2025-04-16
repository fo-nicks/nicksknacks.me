#!/bin/bash

# if debug is set, deploy the debug build
if [ "$1" == "debug" ]; then
    EXE="debug/nicksknacks"
    TARGET="debug"
else
    EXE="release/nicksknacks"
    TARGET="release"
fi

cargo build --$TARGET
rsync --rsync-path="sudo rsync" "./target/$EXE" nicksknacks.me:/var/www/nicksknacks.me
rsync --rsync-path="sudo rsync" -r ./static/ nicksknacks.me:/var/www/nicksknacks.me/static
ssh nicksknacks.me "sudo chown -R nicksknacks /var/www/nicksknacks.me"
ssh nicksknacks.me "sudo setcap 'cap_net_bind_service=+ep' /var/www/nicksknacks.me/nicksknacks"

ssh nicksknacks.me "sudo systemctl daemon-reload"
ssh nicksknacks.me "sudo systemctl restart nicksknacks.me"
ssh nicksknacks.me "sudo systemctl status nicksknacks.me"