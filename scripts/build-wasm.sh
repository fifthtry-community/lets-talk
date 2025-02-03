#!/bin/bash

set -ue
set -o pipefail


cargo build --release --target wasm32-unknown-unknown

# Define the common source folder paths
SOURCE1="./target/wasm32-unknown-unknown/release/"
SOURCE2="$HOME/target/wasm32-unknown-unknown/release/"

# Define destination folders
DEST="./lets-talk.fifthtry.site"

# Ensure WASM files exist and determine the source folder to use
if [ -d "$SOURCE1" ]; then
    SOURCE_DIR=$SOURCE1
elif [ -d "$SOURCE2" ]; then
    SOURCE_DIR=$SOURCE2
else
    echo "Source folder not found."
    exit 1
fi

# Copy files to destinations
cp "${SOURCE_DIR}talk.wasm" "$DEST"

echo "WASM files copied successfully to ${DEST}"
