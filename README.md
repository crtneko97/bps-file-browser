bps-file-browser

This tool will start out working for my Xiaomi phone.

Overview

Here’s the project skeleton:

usb_file_browser/
├── Cargo.toml
└── src/
    ├── main.rs         # CLI entrypoint with clap
    ├── usb.rs          # device discovery & USB claiming
    ├── mtp.rs          # MTP packet framing & commands
    └── cli.rs          # higher-level commands: list, pull, push

Dependencies

Add the following to your Cargo.toml:

[dependencies]
rusb   = "0.9"
anyhow = "1.0"
clap   = { version = "4.0", features = ["derive"] }

rusbAllows you to list all USB devices, open handles, claim interfaces, and perform bulk transfers—everything you need to detect and talk MTP over USB.

anyhowLets you return Result<_, anyhow::Error> from your main(), use ? everywhere, and get nicely formatted error contexts and backtraces.

clapProvides a CLI framework so later you can support commands like:

usb_file_browser list
usb_file_browser pull <remote> <local>
usb_file_browser push <local> <remote>

without writing your own argument parsing.
