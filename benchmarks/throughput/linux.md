# Throughput on Linux

```
OS: Debian GNU/Linux bullseye 11.10 x86_64
Kernel: Linux 5.10.0-29-amd64
CPU: Intel(R) Xeon(R) E5-2680 v3 (16) @ 2.50 GHz
Memory: 16 GiB
Swap: Disabled
Disk (/): 255.75 GiB - ext4
```

## Netstack Lwip

### Single-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack lwip --current-thread; done) | grep Gbps
```

```
client io mode: reader, throughput: 2.214 Gbps
client io mode: reader, throughput: 2.221 Gbps
client io mode: reader, throughput: 2.222 Gbps
client io mode: reader, throughput: 2.060 Gbps
client io mode: reader, throughput: 2.183 Gbps
client io mode: reader, throughput: 2.081 Gbps
client io mode: reader, throughput: 2.140 Gbps
client io mode: reader, throughput: 2.123 Gbps
client io mode: reader, throughput: 2.243 Gbps
client io mode: reader, throughput: 2.094 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack lwip --current-thread; done) | grep Gbps
```

```
client io mode: writer, throughput: 2.169 Gbps
client io mode: writer, throughput: 2.155 Gbps
client io mode: writer, throughput: 2.114 Gbps
client io mode: writer, throughput: 2.090 Gbps
client io mode: writer, throughput: 2.167 Gbps
client io mode: writer, throughput: 2.109 Gbps
client io mode: writer, throughput: 2.114 Gbps
client io mode: writer, throughput: 2.182 Gbps
client io mode: writer, throughput: 2.172 Gbps
client io mode: writer, throughput: 2.203 Gbps
```

### Multi-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack lwip; done) | grep Gbps
```

```
client io mode: reader, throughput: 1.982 Gbps
client io mode: reader, throughput: 2.062 Gbps
client io mode: reader, throughput: 2.014 Gbps
client io mode: reader, throughput: 1.845 Gbps
client io mode: reader, throughput: 2.088 Gbps
client io mode: reader, throughput: 1.970 Gbps
client io mode: reader, throughput: 2.025 Gbps
client io mode: reader, throughput: 2.067 Gbps
client io mode: reader, throughput: 1.860 Gbps
client io mode: reader, throughput: 2.028 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack lwip; done) | grep Gbps
```

```
client io mode: writer, throughput: 1.865 Gbps
client io mode: writer, throughput: 1.817 Gbps
client io mode: writer, throughput: 1.889 Gbps
client io mode: writer, throughput: 1.867 Gbps
client io mode: writer, throughput: 1.784 Gbps
client io mode: writer, throughput: 1.881 Gbps
client io mode: writer, throughput: 1.867 Gbps
client io mode: writer, throughput: 1.781 Gbps
client io mode: writer, throughput: 1.889 Gbps
client io mode: writer, throughput: 1.857 Gbps
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
client io mode: writer, throughput: 0.744 Gbps
client io mode: writer, throughput: 0.790 Gbps
client io mode: writer, throughput: 0.784 Gbps
client io mode: writer, throughput: 0.790 Gbps
client io mode: writer, throughput: 0.754 Gbps
client io mode: writer, throughput: 0.789 Gbps
client io mode: writer, throughput: 0.773 Gbps
client io mode: writer, throughput: 0.778 Gbps
client io mode: writer, throughput: 0.743 Gbps
client io mode: writer, throughput: 0.737 Gbps
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
client io mode: writer, throughput: 0.752 Gbps
client io mode: writer, throughput: 0.709 Gbps
client io mode: writer, throughput: 0.719 Gbps
client io mode: writer, throughput: 0.740 Gbps
client io mode: writer, throughput: 0.699 Gbps
client io mode: writer, throughput: 0.704 Gbps
client io mode: writer, throughput: 0.722 Gbps
client io mode: writer, throughput: 0.710 Gbps
client io mode: writer, throughput: 0.736 Gbps
client io mode: writer, throughput: 0.718 Gbps
```

## Netstack Smoltcp

### Single-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack smoltcp --current-thread; done) | grep Gbps
```

```
client io mode: reader, throughput: 2.015 Gbps
client io mode: reader, throughput: 2.129 Gbps
client io mode: reader, throughput: 1.970 Gbps
client io mode: reader, throughput: 2.088 Gbps
client io mode: reader, throughput: 2.148 Gbps
client io mode: reader, throughput: 2.059 Gbps
client io mode: reader, throughput: 2.116 Gbps
client io mode: reader, throughput: 2.101 Gbps
client io mode: reader, throughput: 1.982 Gbps
client io mode: reader, throughput: 1.995 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack smoltcp --current-thread; done) | grep Gbps
```

```
client io mode: writer, throughput: 3.682 Gbps
client io mode: writer, throughput: 3.758 Gbps
client io mode: writer, throughput: 3.790 Gbps
client io mode: writer, throughput: 3.831 Gbps
client io mode: writer, throughput: 3.770 Gbps
client io mode: writer, throughput: 3.747 Gbps
client io mode: writer, throughput: 3.752 Gbps
client io mode: writer, throughput: 3.779 Gbps
client io mode: writer, throughput: 3.855 Gbps
client io mode: writer, throughput: 3.861 Gbps
```

### Multi-Thread Server

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode reader --netstack smoltcp; done) | grep Gbps
```

```
client io mode: reader, throughput: 1.775 Gbps
client io mode: reader, throughput: 2.212 Gbps
client io mode: reader, throughput: 1.948 Gbps
client io mode: reader, throughput: 2.060 Gbps
client io mode: reader, throughput: 2.106 Gbps
client io mode: reader, throughput: 2.145 Gbps
client io mode: reader, throughput: 2.169 Gbps
client io mode: reader, throughput: 1.867 Gbps
client io mode: reader, throughput: 2.146 Gbps
client io mode: reader, throughput: 2.086 Gbps
```

```bash
cargo build --release -q; (for _ in {1..10}; do sudo target/release/rust-tun-perf --mode writer --netstack smoltcp; done) | grep Gbps
```

```
client io mode: writer, throughput: 3.288 Gbps
client io mode: writer, throughput: 3.342 Gbps
client io mode: writer, throughput: 3.390 Gbps
client io mode: writer, throughput: 3.417 Gbps
client io mode: writer, throughput: 3.377 Gbps
client io mode: writer, throughput: 3.130 Gbps
client io mode: writer, throughput: 3.383 Gbps
client io mode: writer, throughput: 3.298 Gbps
client io mode: writer, throughput: 3.447 Gbps
client io mode: writer, throughput: 3.320 Gbps
```
