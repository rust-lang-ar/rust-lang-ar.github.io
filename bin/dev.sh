#!/bin/sh
cargo watch -s 'bash $PWD/bin/build.sh && simple-http-server -i --nocache --cors ./static' -i ./static
