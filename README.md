# esp-rust-camera

Example repo for streaming frames from ESP32-WROVER-E camera to a web server.

## Setup

1. Create a `cfg.toml` file, which looks like the following, at the root of the repo:

```toml
[esp-rust-camera]
wifi_ssid = "..."
wifi_psk = "..."
```

Pins may need to be adjusted based on your board's layout in `camera.rs`.

## Running

```bash
# Get the ip address from logs
cargo run
# Open browser to `/stream` endpoint.
open http://IP_ADDRESS/stream
```
