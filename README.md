# Snake game

A implementation of the good old snake game in rust programming language using the ggez game framework.
The sprite images were created with [aseprite](https://www.aseprite.org/).

Here's a screenshot of the game:

<p align="center">
  <img src="./snake.png" alt="snake" />
</p>

### How to run it locally :question:

Run the following commands on the terminal:

```shell
foo@bar:~$ git clone https://github.com/eiberham/snake.git
foo@bar:~$ cd snake
foo@bar:~$ cargo run
```

### How to create an installer for mac :question:

Well this way I managed to build the app's bundle and create a DMG file, following the [documentation](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html#//apple_ref/doc/uid/20001119-110730) you can get an idea of how the bundle folder structure is.

Simply run the `build_mac.sh` script within the projects folder, you'll get a DMG file within the bundle_mac folder:

```shell
foo@bar:~$ sh build_mac.sh
```

Now you can share it with any friend or acquaintance who owns a mac.

### How to create an installer for windows :question:

Coming soon

### How to create an installer for linux :question:

Coming soon

### :trollface: For lazy ones where to download it from and how to check if the download is legit?

Coming soon

