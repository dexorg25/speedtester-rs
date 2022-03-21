#!/bin/bash

set -e

echo "Rebuilding binary..."
cargo install --path .

echo "Installing unit file..."
sudo cp speedtester-rs.service /etc/systemd/system 
sudo systemctl daemon-reload 

echo "Restarting service..."
sudo systemctl restart speedtester-rs