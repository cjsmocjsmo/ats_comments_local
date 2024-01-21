#!/bin/bash

sudo apt -y update;
sudo apt -y dist-upgrade;
sudo apt -y autoremove;
sudo apt -y autoclean;
sudo apt -y install \
    build-essential \
    sqlite3 \
    libsqlite3-dev \
    openssl \
    libssl-dev \
    pkg-config \
    git \
    curl;

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;
source $HOME/.cargo/env;

sudo snap install --classic certbot;
sudo ln -s /snap/bin/certbot /usr/bin/certbot;
sudo certbot certonly --standalone;


