# Throughput on macOS

```
OS: macOS Sonoma 14.4.1 arm64
Kernel: Darwin 23.4.0
CPU: Apple M1 (8) @ 3.20 GHz
Memory: 16 GiB
Swap: 4.00 GiB
Disk (/): 926.35 GiB - apfs
```

## Netstack Lwip

### Single-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack lwip --current-thread; done) | grep Gbps
```

```
client io mode: reader, throughput: 2.949 Gbps
client io mode: reader, throughput: 2.929 Gbps
client io mode: reader, throughput: 2.966 Gbps
client io mode: reader, throughput: 2.962 Gbps
client io mode: reader, throughput: 2.962 Gbps
client io mode: reader, throughput: 2.934 Gbps
client io mode: reader, throughput: 2.966 Gbps
client io mode: reader, throughput: 2.950 Gbps
client io mode: reader, throughput: 2.947 Gbps
client io mode: reader, throughput: 2.970 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack lwip --current-thread; done) | grep Gbps
```

```
client io mode: writer, throughput: 2.169 Gbps
client io mode: writer, throughput: 2.194 Gbps
client io mode: writer, throughput: 2.066 Gbps
client io mode: writer, throughput: 2.073 Gbps
client io mode: writer, throughput: 2.116 Gbps
client io mode: writer, throughput: 2.068 Gbps
client io mode: writer, throughput: 2.075 Gbps
client io mode: writer, throughput: 2.042 Gbps
client io mode: writer, throughput: 2.073 Gbps
client io mode: writer, throughput: 2.062 Gbps
```

### Multi-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack lwip; done) | grep Gbps
```

```
client io mode: reader, throughput: 2.883 Gbps
client io mode: reader, throughput: 2.896 Gbps
client io mode: reader, throughput: 2.904 Gbps
client io mode: reader, throughput: 2.896 Gbps
client io mode: reader, throughput: 2.900 Gbps
client io mode: reader, throughput: 2.899 Gbps
client io mode: reader, throughput: 2.898 Gbps
client io mode: reader, throughput: 2.894 Gbps
client io mode: reader, throughput: 2.895 Gbps
client io mode: reader, throughput: 2.910 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack lwip; done) | grep Gbps
```

```
client io mode: writer, throughput: 1.902 Gbps
client io mode: writer, throughput: 1.937 Gbps
client io mode: writer, throughput: 1.937 Gbps
client io mode: writer, throughput: 1.926 Gbps
client io mode: writer, throughput: 1.938 Gbps
client io mode: writer, throughput: 1.933 Gbps
client io mode: writer, throughput: 1.932 Gbps
client io mode: writer, throughput: 1.923 Gbps
client io mode: writer, throughput: 1.936 Gbps
client io mode: writer, throughput: 1.918 Gbps
```

## Netstack IpStack

### Single-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack ipstack --current-thread; done) | grep Gbps
```

```
There is a problem with the IpStack TCP state machine or implementation. The reader mode benchmark cannot be performed.
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack ipstack --current-thread; done) | grep Gbps
```

```
client io mode: writer, throughput: 0.703 Gbps
client io mode: writer, throughput: 0.706 Gbps
client io mode: writer, throughput: 0.705 Gbps
client io mode: writer, throughput: 0.705 Gbps
client io mode: writer, throughput: 0.694 Gbps
client io mode: writer, throughput: 0.702 Gbps
client io mode: writer, throughput: 0.698 Gbps
client io mode: writer, throughput: 0.700 Gbps
client io mode: writer, throughput: 0.697 Gbps
client io mode: writer, throughput: 0.700 Gbps
```

### Multi-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack ipstack; done) | grep Gbps
```

```
There is a problem with the IpStack TCP state machine or implementation. The reader mode benchmark cannot be performed.
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack ipstack; done) | grep Gbps
```

```
client io mode: writer, throughput: 0.674 Gbps
client io mode: writer, throughput: 0.670 Gbps
client io mode: writer, throughput: 0.673 Gbps
client io mode: writer, throughput: 0.667 Gbps
client io mode: writer, throughput: 0.666 Gbps
client io mode: writer, throughput: 0.658 Gbps
client io mode: writer, throughput: 0.639 Gbps
client io mode: writer, throughput: 0.675 Gbps
client io mode: writer, throughput: 0.668 Gbps
client io mode: writer, throughput: 0.672 Gbps
```

## Netstack Smoltcp

### Single-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack smoltcp --current-thread; done) | grep Gbps
```

```
client io mode: reader, throughput: 9.889 Gbps
client io mode: reader, throughput: 10.076 Gbps
client io mode: reader, throughput: 10.296 Gbps
client io mode: reader, throughput: 10.349 Gbps
client io mode: reader, throughput: 10.152 Gbps
client io mode: reader, throughput: 10.336 Gbps
client io mode: reader, throughput: 10.349 Gbps
client io mode: reader, throughput: 10.323 Gbps
client io mode: reader, throughput: 10.349 Gbps
client io mode: reader, throughput: 10.165 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack smoltcp --current-thread; done) | grep Gbps
```

```
client io mode: writer, throughput: 2.067 Gbps
client io mode: writer, throughput: 2.105 Gbps
client io mode: writer, throughput: 2.127 Gbps
client io mode: writer, throughput: 2.006 Gbps
client io mode: writer, throughput: 2.095 Gbps
client io mode: writer, throughput: 2.063 Gbps
client io mode: writer, throughput: 2.103 Gbps
client io mode: writer, throughput: 2.123 Gbps
client io mode: writer, throughput: 2.118 Gbps
client io mode: writer, throughput: 2.113 Gbps
```

### Multi-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack smoltcp; done) | grep Gbps
```

```
client io mode: reader, throughput: 9.780 Gbps
client io mode: reader, throughput: 9.840 Gbps
client io mode: reader, throughput: 9.988 Gbps
client io mode: reader, throughput: 9.926 Gbps
client io mode: reader, throughput: 9.988 Gbps
client io mode: reader, throughput: 9.988 Gbps
client io mode: reader, throughput: 9.975 Gbps
client io mode: reader, throughput: 10.038 Gbps
client io mode: reader, throughput: 10.025 Gbps
client io mode: reader, throughput: 9.901 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack smoltcp; done) | grep Gbps
```

```
client io mode: writer, throughput: 1.733 Gbps
client io mode: writer, throughput: 1.661 Gbps
client io mode: writer, throughput: 1.742 Gbps
client io mode: writer, throughput: 1.749 Gbps
client io mode: writer, throughput: 1.750 Gbps
client io mode: writer, throughput: 1.741 Gbps
client io mode: writer, throughput: 1.751 Gbps
client io mode: writer, throughput: 1.750 Gbps
client io mode: writer, throughput: 1.751 Gbps
client io mode: writer, throughput: 1.757 Gbps
```
