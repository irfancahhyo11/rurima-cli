#!/bin/bash

echo "Compiling rurima-cli..."
gcc -Wall -Wextra -std=c99 -o rurima-cli rurima-cli.c

if [ $? -eq 0 ]; then
    echo "Compilation successful!"
    chmod +x rurima-cli
    echo "Made executable. You can now run: ./rurima-cli"
else
    echo "Compilation failed!"
    exit 1
fi
