# bps-file-browser
So this will start out working for my Xiaomi phone.

13:55

Skeleton idea

usb_file_browser/
├── Cargo.toml
└── src/
    ├── main.rs         # CLI entrypoint with clap
    ├── usb.rs          # device discovery & USB claiming
    ├── mtp.rs          # MTP packet framing & commands
    └── cli.rs          # higher-level commands: list, pull, push


[dependencies]
rusb   = "0.9" 
anyhow = "1.0" 
clap   = { version = "4.0", features = ["derive"] }  

rusb
`
Allows you to list all USB devices, open handles, claim interfaces, do bulk transfers, etc.—everything you need to spot and talk MTP over USB.
`
anyhow
`
 Lets you return Result<…, anyhow::Error> from your main(), use ? everywhere, and get backtraces without boilerplate.
`
 clap
`Gives you a CLI framework so later you can do usb_file_browser list vs. pull vs. push without hand-rolling your own argument parsing.`

