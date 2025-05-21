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

| Term                         | What it is / why it matters                                                                                            |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **DeviceDescriptor**         | A little struct that holds a device’s “business card” (vendor\_id, product\_id, class, etc.).                          |
| **vendor\_id / product\_id** | Hex codes identifying *who* made the device (vendor) and *what* it is (product).                                       |
| **DeviceHandle<\_>**         | Your exclusive “key” to the device. The `<_>` is just a placeholder telling Rust “you figure out the exact type here.” |
| **Configuration**            | A mode or “floor” of the device. We chose configuration 1 because that’s where MTP lives.                              |
| **Interface**                | An “office” on that floor. Claiming it locks it for your use.                                                          |
| **Endpoint**                 | A “mail slot” on the office door. Data goes in or out here.                                                            |
| **addr**                     | The slot’s number + direction bit (IN vs. OUT).                                                                        |
| **attr**                     | The slot’s transfer type (Bulk, Interrupt, …).                                                                         |
| **Bulk transfer**            | High-volume, reliable data (no timing guarantees). Ideal for file data.                                                |
| **Interrupt transfer**       | Small, low-latency messages. Good for status or event notifications.                                                   |


---

## Progress

* **USB detection**: used `rusb::Context::new()` and `ctx.devices()` to list all USB devices.

  ```rust
  let ctx = Context::new()?;
  for device in ctx.devices()?.iter() { /* ... */ }
  ```

* **Device setup**: detached the kernel driver, set config #1, and claimed interface 0.

  ```rust
  if handle.kernel_driver_active(0)? {
      handle.detach_kernel_driver(0)?;
  }
  handle.set_active_configuration(1)?;
  handle.claim_interface(0)?;
  ```

* **Endpoint discovery**: printed Bulk-IN and Bulk-OUT endpoints for MTP.

  ```rust
  for ep in descriptor.endpoint_descriptors() {
      println!("endpoint 0x{:02x}, {:?}", ep.address(), ep.transfer_type());
  }
  ```

* **MTP session**: framed and sent a `GetDeviceInfo` command over bulk transfers.

  ```rust
  let mut session = MtpSession::new(handle)?;
  let info = session.get_device_info()?;
  ```

## Quick run

* Build and scan with:

  ```bash
  cargo build
  cargo run -- scan --xiaomi
  ```

## Notes

* Code is organized into modules: `cli.rs`, `usb.rs`, `mtp.rs`.
* The `scan` command opens the phone, lists endpoints, and performs a simple MTP `GetDeviceInfo`.
* Next: parse responses and implement `list`, `pull`, and `push` commands.
