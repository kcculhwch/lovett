#!/bin/bash

cargo doc --no-deps

rm -rf docs/implementors
rm -rf docs/src
rm -rf docs/lovett

cp -r target/armv7-unknown-linux-musleabihf/doc/implementors docs/implementors
cp -r target/armv7-unknown-linux-musleabihf/doc/src docs/src
cp -r target/armv7-unknown-linux-musleabihf/doc/lovett docs/lovett
