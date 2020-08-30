#!/bin/bash

rm -rf dist
wasm-pack build $1 --target web --out-name wasm --out-dir ./dist
cp static/*.html dist/.
cp static/*.css dist/.
#cp static/*.png dist/.
