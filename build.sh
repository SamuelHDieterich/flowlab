#!/bin/env bash

# Build the project
echo "Building the project..."

## Linux
echo "Building for x86_64-unknown-linux-gnu..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/flowlab build/flowlab-linux

## Windows
echo "Building for x86_64-pc-windows-gnu..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/flowlab.exe build/flowlab-windows.exe