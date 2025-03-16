#!/bin/bash

# Set the project root directory
PROJECT_DIR="/Users/sudhanshushekhar/Desktop/projects/sss_shared"
cd "$PROJECT_DIR"

# Build the Rust library
echo "Building Rust library..."
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "❌ Failed to build Rust library"
    exit 1
fi

echo "✅ Rust library built successfully"

# Compile the C example
echo "Compiling C example..."
gcc -o c_example examples/c_example.c -L./target/release -lsss_shared

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "❌ Failed to compile C example"
    exit 1
fi

echo "✅ C example compiled successfully"

# Set the library path and run the example
echo "Running C example..."
export DYLD_LIBRARY_PATH="$PROJECT_DIR/target/release"
./c_example

# Clean up
echo "Cleaning up..."
rm -f c_example