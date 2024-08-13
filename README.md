# Rust TUN Performance Test

## Prerequisites

- Rust 1.79+
- Tun for Linux
- Utun for macOS
- [Wintun for Windows](https://www.wintun.net/)
- [Clang/LLVM compiler](https://rust-lang.github.io/rust-bindgen/requirements.html)

## Throughput Test

Server runtime: the server uses Tokio to run asynchronously, and handles IO reading and writing of tun device/netstack at the same time.

Client runtime: the client works in a single IO mode (either reading or writing), and runs synchronously in a rust standard single thread.

Test IO mode: the client's reader mode corresponds to the server's writting, and the client's writer mode corresponds to the server's reading.

Benchmark test: the client reads or writes data on a TCP stream to the tun device, and the server uses tun2 to read and hand it over a netstack.

- [macOS](./benchmarks/throughput/macos.md)
- [Linux](./benchmarks/throughput/linux.md)
- [Windows](./benchmarks/throughput/windows.md)

## Bandwidth Test

**TODO**

- [macOS](./benchmarks/bandwidth/macos.md)
- [Linux](./benchmarks/bandwidth/linux.md)
- [Windows](./benchmarks/bandwidth/windows.md)
