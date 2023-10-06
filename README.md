Small utility for mapping Display Serial Number to Sway's variable.

In `Sway Variable` will be stored short name like `HDMI-A-1` which may vary after reboot if multilpe displays is used.

## Build
```bash
cargo build --release
```

## Usage
```
Usage: ./target/release/sway-output-mapper [OPTIONS]

Options:
      --list           Display table of Short Names and Serial Numbers
      --map <VAR:S/N>  Mappings rule from Sway Variable Name to Serial Number (multiple allowed)
  -h, --help           Print help
  -V, --version        Print version
```

Run command before `sway` started and store it's output in `.conf` file (e.g. `/tmp/sway-output-variables.conf`).
Include stored file on top of `./config/sway/config`:
```
include /tmp/sway-output-variables.conf
```
