#!/bin/bash

cargo build --release

sudo cp target/release/astra /usr/local/bin/astra
