#!/bin/bash

# This file install all the relevant libraries to compile the project.
# This was tested on ubuntu lts.

HEADER="\e[1;32m"
BODY="\e[0m"

printf ${HEADER}"\n \n \n \n ---------- INSTALLING RUST ---------\n\n"${BODY}
sudo apt-get install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env


printf ${HEADER}"\n \n \n \n ---------- INSTALLING REQUIRED C LIBRARIES ---------\n\n"${BODY}
# cc
sudo apt-get install build-essential -y

# openssl-sys v0.9.65
sudo apt-get install libssl-dev -y


# For reading .fits files for lens correction.
sudo apt-get install libcfitsio-dev -y


# freetype-sys v0.13.1
sudo apt-get install cmake -y


# tiff v0.6.1
sudo apt-get install libfontconfig libfontconfig1-dev -y

# For Lens Calibration
sudo brew install opencv -y
sudo apt-get install libopencv-dev -y

sudo apt-get update

sudo apt-get install llvm -y
sudo apt-get install clang -y
sudo apt-get install libclang-dev -y


printf ${HEADER}"\n \n \n \n ---------- UPGRADING RUST TO RECOMMENDED VERSION ---------\n\n"${BODY}
# new resources required
rustup default nightly-2021-08-20-x86_64-unknown-linux-gnu



printf ${HEADER}"\n \n \n \n ---------- CONSTRUCTING DATABASE ---------\n\n"${BODY}
# Creates database
cargo build --bin database_generator --features "setup"








