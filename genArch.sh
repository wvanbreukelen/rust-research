#!/bin/bash

# Config
targetpath="rust-platforms"

echo "Generating architecture for $1"

filename=$(basename -- "$1")
extension="${filename##*.}"
filename="${filename%.*}"

# Generates an Rust microcontroller crate for a given svd file.
folder=$targetpath/$filename

echo "Creating $folder and copying svd file..."
rm -rf $folder
mkdir $folder
cp $1 $folder

cd $folder

echo "Running svd2rust upon $folder/$filename.$extension..."
svd2rust -i $filename.$extension

echo "Reformatting..."
rm -rf src
form -i lib.rs -o src/ && rm lib.rs

echo "Performing crate init..."
cargo init

realpath() {
  OURPWD=$PWD
  cd "$(dirname "$1")"
  LINK=$(readlink "$(basename "$1")")
  while [ "$LINK" ]; do
    cd "$(dirname "$LINK")"
    LINK=$(readlink "$(basename "$1")")
  done
  REALPATH="$PWD/$(basename "$1")"
  cd "$OURPWD"
  echo "$REALPATH"
}

cat > temp.toml << EOF
bare-metal = "0.2.5"
cortex-m = "0.6.0"
vcell = "0.1.2"

[dependencies.cortex-m-rt]
optional = true
version = "0.6.10"

[features]
rt = ["cortex-m-rt/device"]

[package.metadata.docs.rs]
features = ["rt"]

[profile.release]
codegen-units = 1 # better optimizations
debug = true # symbols are nice and they don't increase the size on Flash
lto = true # better optimizations
EOF

cat temp.toml >> Cargo.toml

rm temp.toml

echo "Finalising..."
cargo fmt