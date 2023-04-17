
# sim_connect_sys

A package which generates Rust bindings for SimConnect (for use with Microsoft Flight Simulator)

## Installation

- Install the SimConnect sdk
    - Open MSFS
    - Enable develop mode in Options/General Options/Developers
    - Click "Help" on the top menu bar
    - Click SDK Installer (Core)
- If installed to default location of `C:\MSFS SDK\`, then you may stop here
- Add an environment variable to `cargo.toml` in the `[env]` section as follows:
```toml
[env]
SIMCONNECT_SDK = "path/to/sdk/lib"
```
- The path MUST contain both `SimConnect.dll` AND `SimConnect.lib`. This is usually in `...\MSFS SDK\SimConnect SDK\lib`
## Features

- `static_link` - This will tell the compiler to staticly link SimConnect instead of requiring a `.dll` dependency