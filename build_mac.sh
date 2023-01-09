#!/bin/bash

set -e

APP_NAME=snake
MACOS_BIN_NAME=snake-bin
MACOS_APP_NAME=Snake
MACOS_APP_DIR=$MACOS_APP_NAME.app
BUNDLE_DIR=bundle_mac

rm -rf $BUNDLE_DIR
mkdir -p $BUNDLE_DIR
cd $BUNDLE_DIR
echo "Creating app directory structure"
rm -rf $MACOS_APP_NAME
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo build \
    --verbose \
    --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp ../target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying resources directory"
cp -r ../resources $MACOS_APP_DIR/Contents/MacOS

echo "Copying launcher"
cp ../launch_mac.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Copying icon"
mkdir -p $MACOS_APP_DIR/Contents/Resources
cp ../resources/Info.plist $MACOS_APP_DIR/Contents/
cp ../resources/snake.icns $MACOS_APP_DIR/Contents/Resources/

echo "Setting permissions ..."
chmod -R 777 .
echo "Done."

echo "Creating .dmg image"
mkdir -p $MACOS_APP_NAME
cp -r $MACOS_APP_DIR $MACOS_APP_NAME/
rm -rf $MACOS_APP_NAME/.Trashes

hdiutil create -srcfolder $MACOS_APP_NAME -volname "Snake" -fs HFS+ \
      -fsargs "-c c=64,a=16,e=16" -format UDRW -megabytes 20 $MACOS_APP_NAME.dmg

device=$(hdiutil attach -readwrite -noverify -noautoopen "Snake.dmg" | \
         egrep '^/dev/' | sed 1q | awk '{print $1}')
sleep 5

echo "Copying background image"
BACKGROUND_IMAGE_NAME=background.png
mkdir -p /Volumes/$MACOS_APP_NAME/.background
cp ../resources/$BACKGROUND_IMAGE_NAME /Volumes/$MACOS_APP_NAME/.background/

echo '
   tell application "Finder"
     tell disk "'${MACOS_APP_NAME}'"
           open
           set current view of container window to icon view
           set toolbar visible of container window to false
           set statusbar visible of container window to false
           set the bounds of container window to {400, 100, 885, 430}
           set theViewOptions to the icon view options of container window
           set arrangement of theViewOptions to not arranged
           set icon size of theViewOptions to 72
           set background picture of theViewOptions to file ".background:'${BACKGROUND_IMAGE_NAME}'"
           make new alias file at container window to POSIX file "/Applications" with properties {name:"Applications"}
           set position of item "'${MACOS_APP_NAME}'" of container window to {100, 100}
           set position of item "Applications" of container window to {375, 100}
           update without registering applications
           delay 5
           close
     end tell
   end tell
' | osascript

chmod -Rf go-w /Volumes/"${MACOS_APP_NAME}"
sync
sync
hdiutil detach ${device}
hdiutil convert "/Snake.dmg" -format UDZO -imagekey zlib-level=9 -o Snake.dmg