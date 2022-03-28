#!/bin/bash

dist_dir="$PWD/.dist"
dist_file="$PWD/bm_fileio.zip"

[ -d $dist_dir ] && rm -rf $dist_dir
mkdir -p $dist_dir/res/{py,rs}

# Python
pushd py/bm_fileio
rm -rf *.egg-info
pip3 wheel -w $dist_dir/res/py -e .
popd

# Rust
pushd rs/bm_fileio
cargo build --release
mkdir -p $dist_dir/res/rs
cp -vf target/release/bm_fileio $dist_dir/res/rs
popd

# Scripts
cp -vf deploy.sh $dist_dir
cp -vf run.sh $dist_dir

# Dist
(cd $dist_dir && zip -r - *) > $dist_file