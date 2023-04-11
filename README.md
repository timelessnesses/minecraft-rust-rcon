# minecraft-rust-rcon
A RCON client written in rust (supposed to be built once run everywhere and doesn't need to be configured just one click run!)

## Setup
1. Rust (isn't it obvious) (and with cargo of course)
2. MSVC C++

## Build
1. Edit config.toml (this file will be included in the binary so every machine doesn't need config.toml all the time) (but might plan doing client soon)
2. Run `cargo build -r`, this will build the project in release mode so file size should be under a megabytes!
