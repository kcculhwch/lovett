#!/bin/bash

cargo doc --no-deps

cp -r target/armv7-unknown-linux-musleabihf/doc/implementors docs/implementors
cp -r target/armv7-unknown-linux-musleabihf/doc/src docs/src
cp -r target/armv7-unknown-linux-musleabihf/doc/lovett docs/lovett
