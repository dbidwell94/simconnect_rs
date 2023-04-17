
# sim_connect_rs

An opinionated wrapper around SimConnect which allows safe calling between the C api and Rust. Easy to use, and easy to create new datatypes for use in SimConnect.

## Installation

- Install the SimConnect SDK. Instructions located [here](https://crates.io/crates/sim_connect_sys)
- Specify if you want to dynamically or staticly link the library via a feature flag (`static_link`)
## Cargo Addons

- `static_link`
    - This will tell the compiler to staticly link SimConnect instead of requiring a `.dll` dependency

- `async`
    - This will allow you to use asyncrounous versions of this API, such as waiting for data to be retrieved from SimConnect.

- `derive`
    - This will allow you to easily create SimConnect structs by using a `derive` macro
## Features

- Async runtime
    - data is fetched on a background thread and returned to the client
- Rust style enums
    - Use rust enums instead of C-Style vars to communicate with SimConnect
- Auto-struct serialization
    - Using the `derive` macro provided, easily create structs which can communicate with  SimConnect
- Listener based event system
    - Instead of polling for events, just subscribe to an event and your callback will be invoked when an event is recieved.