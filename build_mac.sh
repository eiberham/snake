#!/bin/bash

set -e

APP_NAME=snake
MACOS_BIN_NAME=snake-bin
MACOS_APP_NAME=Snake
MACOS_APP_DIR=$MACOS_APP_NAME.app

rm -rf macos
mkdir -p macos
cd macos
echo "Creating app directory structure"
rm -rf $MACOS_APP_NAME
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo rustc \
    --verbose \
    --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp ../target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying resources directory"
cp -r ../resources $MACOS_APP_DIR/Contents/MacOS

echo "Copying launcher"
cp ../launch_mac.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Copying Icon"
mkdir -p $MACOS_APP_DIR/Contents/Resources
cp ../resources/Info.plist $MACOS_APP_DIR/Contents/
cp ../resources/snake.icns $MACOS_APP_DIR/Contents/Resources/

echo "Setting permissions ..."
chmod -R 777 .
echo "Done."