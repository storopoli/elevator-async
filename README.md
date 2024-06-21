# Elevator System

[![AGPL-v3](https://img.shields.io/badge/License-AGPL&nbsp;v3-lightgrey.svg)](https://opensource.org/license/agpl-v3/)

This is a simple study on how to build an [actor-based](https://en.wikipedia.org/wiki/Actor_model)
async parallel elevator system in Rust.

It uses [tokio](https://tokio.rs) for async IO with
the [`tokio::sync::mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/) channel for message passing.

## License

The source code is licensed under an
[AGPL v3 License](https://opensource.org/license/agpl-v3/)

[![AGPL-v3](https://upload.wikimedia.org/wikipedia/commons/thumb/0/06/AGPLv3_Logo.svg/320px-AGPLv3_Logo.svg.png)](https://opensource.org/license/agpl-v3/)