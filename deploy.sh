#!/bin/bash

SERVER="falcon"

# copy the folder to the server
echo "Pushing folder to server"
cargo clean -q
TMP="$(ssh $SERVER mktemp -d)"

echo "Building in '$TMP'"

scp -r . $SERVER:"$TMP"

# build and install the program
ssh $SERVER cargo install --path "$TMP"

# restart the service
ssh $SERVER sudo systemctl restart speedtester-rs

# Cleanup
ssh $SERVER rm -r "$TMP"