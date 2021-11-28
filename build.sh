#!/bin/sh

wasm-pack build --target web --out-name wasm
cp ./pkg/* ./static
