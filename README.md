# Elevator System

[![AGPL-v3](https://img.shields.io/badge/License-AGPL&nbsp;v3-lightgrey.svg)](https://opensource.org/license/agpl-v3/)

This is a simple study on how to build an [actor-based](https://en.wikipedia.org/wiki/Actor_model)
async parallel elevator system in Rust.

It uses [tokio](https://tokio.rs) for async IO with
the [`tokio::sync::mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/) channel for message passing.

## How to use

First run the server with:

```bash
cargo run
```

Please note that the elevator system will only print the state of the system to the console,
if you run without `--release` flag.
This enables the `cfg(debug_assertions)` macro in the code base.

This will bind the server to port `3000`.
Then open a new terminal and send commands to the socket:

```bash
# Linux
nc localhost 3000
# macOS
netcat localhost 3000
```

The commands are JSON objects.
To simulate an:

- Floor switch input: `{"FloorSwitch":{"floor":3,"direction":"Up"}}`
- Elevator switch input: `{"ElevatorSwitch":{"destination":10}}`

Then watch your elevator spam your console with messages.

## License

The source code is licensed under an
[AGPL v3 License](https://opensource.org/license/agpl-v3/)

[![AGPL-v3](https://upload.wikimedia.org/wikipedia/commons/thumb/0/06/AGPLv3_Logo.svg/320px-AGPLv3_Logo.svg.png)](https://opensource.org/license/agpl-v3/)