#!/bin/bash

# This file install all the relevant libraries to compile the project.
# This was tested on ubuntu lts.

sudo apt-get install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# cc
sudo apt-get install build-essential -y

# openssl-sys v0.9.65
sudo apt-get install libssl-dev -y

# freetype-sys v0.13.1
sudo apt-get install cmake -y

# tiff v0.6.1
sudo apt-get install libfontconfig libfontconfig1-dev -y

# new resources required
rustup default nightly-2021-08-20-x86_64-unknown-linux-gnu

# Creates database
cargo build --bin database_generator
