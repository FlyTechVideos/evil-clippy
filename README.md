# Evil Clippy 📎

Clippy, but he is not so friendly.

As seen in [this YouTube video from FlyTech Videos](https://youtu.be/1jQbf59Jq2s).

## Install and use

Install Clippy using the installer. Then, press Ctrl+, (Comma) to make it appear and disappear, or click the icon in the taskbar.

To use Clippy, you need to prepare replay scripts. You can find the scripts from the video in [./demo-scripts](demo-scripts). Place them in `%appdata%/EvilClippy/scripts`. Then load them by typing `/load script1.tsv` into Clippy. If you want to abort a certain script, wait for any message in progress to finish, then type `/clear` and load another script.

## Build locally

- Install tauri as per the official guide (at the time of writing, it is here: https://tauri.app/v1/guides/getting-started/prerequisites)
- Install Node.js using your preferred way of managing Node installations
  - For development, I used Node v20.11.1 (LTS)
  - I recommend using [Volta.sh](https://volta.sh/), but you can use whatever you wish or install Node directly on your system
- Check out this project
- Open a shell in the root directory
- Type `cargo tauri dev` for live debugging with hot reload
- Type `cargo tauri build` for building the release version

Contributions and forks welcome.
