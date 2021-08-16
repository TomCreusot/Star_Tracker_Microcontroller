#!/bin/bash

# This file install all the relevant libraries to compile the project.
# This was tested on ubuntu lts.

sudo apt-get install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# cc
sudo apt-get install build-essential

# openssl-sys v0.9.65
sudo apt-get install libssl-dev

# freetype-sys v0.13.1
sudo apt-get install cmake

# tiff v0.6.1
sudo apt-get install libfontconfig libfontconfig1-dev
