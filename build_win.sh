#!/bin/bash

BUNDLE_DIR=bundle_win
rm -rf bundle_win

mkdir -p $BUNDLE_DIR
cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
cp -r resources $BUNDLE_DIR
cp target/release/snake.exe $BUNDLE_DIR/snake.exe
cp $BUNDLE_DIR/resources/snake.ico $BUNDLE_DIR/snake.ico

cd $BUNDLE_DIR

# expects https://github.com/electron/rcedit to be on path
rcedit "snake.exe" --set-icon "snake.ico"
rm snake.ico