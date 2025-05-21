# bps-file-browser
***WIP***
This tool will start out working for my Xiaomi phone.

## Overview

Here’s the project skeleton:

```plaintext
usb_file_browser/
├── Cargo.toml
└── src/
    ├── main.rs         # CLI entrypoint with clap
    ├── usb.rs          # device discovery & USB claiming
    ├── mtp.rs          # MTP packet framing & commands
    └── cli.rs          # higher-level commands: list, pull, push
```

## Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rusb   = "0.9"
anyhow = "1.0"
clap   = { version = "4.0", features = ["derive"] }
```

* **rusb**
  Allows you to list all USB devices, open handles, claim interfaces, and perform bulk transfers—everything you need to detect and talk MTP over USB.

* **anyhow**
  Lets you return `Result<_, anyhow::Error>` from your `main()`, use `?` everywhere, and get nicely formatted error contexts and backtraces.

* **clap**
  Provides a CLI framework so later you can support commands like:

  ```bash
  usb_file_browser list
  usb_file_browser pull <remote> <local>
  usb_file_browser push <local> <remote>
  ```

  without writing your own argument parsing.

---
