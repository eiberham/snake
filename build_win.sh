#!/bin/bash

rm -rf winbuild/

mkdir -p target/debug/
mkdir -p target/release/
cp -r resources target/debug/
cp -r resources target/release/

cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
mkdir -p winbuild
cp -r resources winbuild/
cp target/release/snake.exe winbuild/snake.exe
mv winbuild/resources/logo.ico winbuild/logo.ico

cd winbuild
# Expects https://github.com/electron/rcedit to be on Path
rcedit "snake.exe" --set-icon "logo.ico"
rm logo.ico