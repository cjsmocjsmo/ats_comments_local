#!/bin/bash

if [ ! -d "/usr/share/ats_comments" ]; then
    sudo mkdir -p "/usr/share/ats_comments";
    sudo chown -R $USER:$USER "/usr/share/ats_comments";
    sudo chmod -R 755 "/usr/share/ats_comments";
    echo "Created dir";
fi

if [ -d "/usr/share/ats_comments/uploads" ]; then
    rm -rf "/usr/share/ats_comments/uploads";
    echo "Removed uploads dir";
fi 


if [ -d "/usr/share/ats_comments/ats_comments/db" ]; then
    rm -rf "/usr/share/ats_comments/ats_comments/db";
    echo "Removed db dir";
fi 


git pull;

# RUST_LOG=info cargo run --release;
cargo build --release;

ATSCOMMENTS="~/ats_comments/target/release/ats_comments";

if [ ! -f /usr/bin/ats_comments ]; then
    sudo mv -v $ATSCOMMENTS /usr/bin/ats_comments;
    sudo chown root:root /usr/bin/ats_comments;
    sudo chmod +x /usr/bin/ats_comments;
    echo "Installed ats_comments";
fi

if [ -f /usr/bin/ats_comments ]; then
    sudo rm /usr/bin/ats_comments;
    sudo cp -pvr $ATSCOMMENTS /usr/bin/ats_comments;
    sudo chown root:root /usr/bin/ats_comments;
    sudo chmod +x /usr/bin/ats_comments;
    echo "Updated ats_comments";
fi

SYSD="/etc/systemd/system/";
SERVFILE="/usr/share/ats_comments/ats_comments/ats_comments.service";
if [ ! -f $SYSD"/ats_comments.service" ]; then
    sudo cp -pvr $SERVFILE $SYSD;
    sudo chown root:root $SYSD"ats_comments.service";
    sudo chmod 644 $SYSD"ats_comments.service";
    echo "Installed ats_comments.service";
fi 

# sudo systemctl start ats_comments.service;
# sudo systemctl enable ats_comments.service
