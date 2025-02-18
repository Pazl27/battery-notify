#!/bin/bash

# Variables
REPO_URL="https://github.com/Pazl27/battery-notify.git"
REPO_DIR="battery-notify"
EXECUTABLE_NAME="battery-notify"

# Clone the repository
git clone "$REPO_URL" "$REPO_DIR"

# Change to the repository directory
cd "$REPO_DIR" || exit

# Build the project with Cargo
cargo build --release

# Move the executable to /usr/bin
sudo mv "target/release/$EXECUTABLE_NAME" /usr/bin/

# Change back to the original directory
cd ..

# Remove the repository
rm -rf "$REPO_DIR"

echo "Executable $EXECUTABLE_NAME has been moved to /usr/bin and the repository has been removed."
