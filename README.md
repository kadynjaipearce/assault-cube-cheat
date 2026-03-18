# AC Cheat

A cheat tool for Assault Cube with aimbot functionality.

## Requirements

- **Assault Cube version 1.3.0.2** - Must be installed and running
- **Windows** - This tool is Windows-only
- **Rust** - For building the project

## Installation

1. Clone this repository
2. Download Assault Cube 1.3.0.2 [Link Here](https://assault.cubers.net/download.html)
3. Ensure Assault Cube 1.3.0.2 is installed and running
4. Build and run with:

```bash
cargo run --target i686-pc-windows-msvc
```

## Usage

- **Hold Right Click** - Toggles aimbot on/off

## Features

- ✅ Aimbot - Automatically aims at the closest enemy (toggle with right click)
- ⚠️ ESP - Currently unimplemented (Need Overlay)

## Notes

- The program reads memory from the `ac_client.exe` process
- Make sure Assault Cube is running before starting the cheat
- The aimbot will only target alive enemies
