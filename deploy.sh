#!/bin/bash

SERVER="falcon"
BUILD_PATH="/tmp/speedtester-rs"

# copy the folder to the server
echo "Pushing folder to server"
ssh $SERVER rm $BUILD_PATH
scp -r . $SERVER:$BUILD_PATH

# build and install the program
ssh $SERVER cargo install --path BUILD_PATH

# restart the service