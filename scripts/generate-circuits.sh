#!/bin/bash

while :
do
    # Generate circuits with mcr
    cd mcr
    uv venv
    source .venv/bin/activate
    uv run main.py
    deactivate
    cd ..

    # Copy generated circuits
    cp -r "mcr/circuit_data/"* "temp/"

    # Analyse circuits
    cargo run
done