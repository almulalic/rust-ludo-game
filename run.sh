#!/bin/bash

if [[ "$(uname -s)" == "Linux" ]]; then
    echo "Running on Linux"
elif [[ "$(uname -s)" == "Darwin" ]]; then
    echo "Running on macOS"
    ./terminal/Alacritty.app/Contents/MacOS/alacritty \
        --option font.size=48 \
        --config-file /dev/null \
        --working-directory="$(pwd)" \
        -e "/bin/bash -c pwd" \
        #--command "cargo run --bin programming-languages-project"
elif [[ "$(uname -s)" == "CYGWIN"* || "$(uname -s)" == "MINGW"* ]]; then
    echo "Running on Windows (Cygwin or MinGW)"
else
    echo "Unsupported operating system"
fi
