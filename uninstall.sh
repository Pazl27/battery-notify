#!/bin/bash

# Variables
EXECUTABLE_NAME="battery-notify"

# Remove the executable from /usr/bin
sudo rm /usr/bin/"$EXECUTABLE_NAME"

echo "Executable $EXECUTABLE_NAME has been removed from /usr/bin."
