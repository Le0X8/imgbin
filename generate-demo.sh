#!/usr/bin/env bash

cargo build --release

rm -rf demo/*

./target/release/imgbin b README.md --noalpha
mv README.md.png demo/rgb.png

./target/release/imgbin b README.md
mv README.md.png demo/rgba.png

./target/release/imgbin b README.md --grayscale --noalpha
mv README.md.png demo/luma.png

./target/release/imgbin b README.md --grayscale
mv README.md.png demo/lumaa.png
