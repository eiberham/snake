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

### How to build it for mac :question:

Well this way I managed to build the app's bundle, it's not the most intelligent approach though but it worked for me.
First off, run the `build_mac.sh` script within the projects folder, you'll get a bundle file within the macos folder:

```shell
foo@bar:~$ sh build_mac.sh
```

If it doesn't display the icon right away you can always right click over the bundle file, get information and drag and drop your own `app.icns` file.

Second step would be to create a custom `.dmg` so, in order to do that we'd need to follow these steps:

1. Create a .dmg with the disk utility app:
  > File > New image > Empty image

  Screenshot images coming soon

2. Create a background image for the installer:

  > For this I prefered to use gimp

3. Change the `.dmg` file permissions to read only.

```shell
foo@bar:~$ chmod -R a=r Snake.dmg
```

### How to build it for win :question:

Coming soon

### :trollface: Where to download it from and how to check if the download is legit?

Coming soon

