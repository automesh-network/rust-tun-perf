# Throughput on Windows

```
OS: Windows 10 (Enterprise LTSC) x86_64
Kernel: WIN32_NT 10.0.17763.6054 (1809)
CPU: Intel(R) Xeon(R) E5-2620 v4 (8) @ 2.10 GHz
Memory: 16 GiB
Swap: 2.38GB
Disk (C:\): 149.46 GiB (71%) - NTFS
```

*Note: The following scripts are all run in PowerShell with administrator privileges.*

## Netstack Lwip

### Single-Thread Server

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack lwip --current-thread | Select-String -Pattern "Gbps" }
```

```
client io mode: reader, throughput: 0.638 Gbps
client io mode: reader, throughput: 0.504 Gbps
client io mode: reader, throughput: 0.583 Gbps
client io mode: reader, throughput: 0.537 Gbps
client io mode: reader, throughput: 0.516 Gbps
client io mode: reader, throughput: 0.525 Gbps
client io mode: reader, throughput: 0.432 Gbps
client io mode: reader, throughput: 0.538 Gbps
client io mode: reader, throughput: 0.518 Gbps
client io mode: reader, throughput: 0.606 Gbps
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack lwip --current-thread | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 1.199 Gbps
client io mode: writer, throughput: 1.008 Gbps
client io mode: writer, throughput: 1.067 Gbps
client io mode: writer, throughput: 1.146 Gbps
client io mode: writer, throughput: 1.019 Gbps
client io mode: writer, throughput: 1.053 Gbps
client io mode: writer, throughput: 0.984 Gbps
client io mode: writer, throughput: 1.159 Gbps
client io mode: writer, throughput: 1.031 Gbps
client io mode: writer, throughput: 1.007 Gbps
```

### Multi-Thread Server
```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack lwip | Select-String -Pattern "Gbps" }
```

```
client io mode: reader, throughput: 0.622 Gbps
client io mode: reader, throughput: 0.493 Gbps
client io mode: reader, throughput: 0.540 Gbps
client io mode: reader, throughput: 0.560 Gbps
client io mode: reader, throughput: 0.616 Gbps
client io mode: reader, throughput: 0.582 Gbps
client io mode: reader, throughput: 0.554 Gbps
client io mode: reader, throughput: 0.616 Gbps
client io mode: reader, throughput: 0.574 Gbps
client io mode: reader, throughput: 0.497 Gbps
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack lwip | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 0.928 Gbps
client io mode: writer, throughput: 0.999 Gbps
client io mode: writer, throughput: 0.940 Gbps
client io mode: writer, throughput: 0.928 Gbps
client io mode: writer, throughput: 0.905 Gbps
client io mode: writer, throughput: 0.955 Gbps
client io mode: writer, throughput: 0.942 Gbps
client io mode: writer, throughput: 0.932 Gbps
client io mode: writer, throughput: 0.933 Gbps
client io mode: writer, throughput: 0.941 Gbps
```

## Netstack IpStack

### Single-Thread Server

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack ipstack --current-thread | Select-String -Pattern "Gbps" }
```

```
There is a problem with the IpStack TCP state machine or implementation. The reader mode benchmark cannot be performed.
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack ipstack --current-thread | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 0.267 Gbps
client io mode: writer, throughput: 0.249 Gbps
client io mode: writer, throughput: 0.242 Gbps
client io mode: writer, throughput: 0.250 Gbps
client io mode: writer, throughput: 0.248 Gbps
client io mode: writer, throughput: 0.244 Gbps
client io mode: writer, throughput: 0.279 Gbps
client io mode: writer, throughput: 0.291 Gbps
client io mode: writer, throughput: 0.284 Gbps
client io mode: writer, throughput: 0.289 Gbps
```

### Multi-Thread Server

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack ipstack | Select-String -Pattern "Gbps" }
```

```
There is a problem with the IpStack TCP state machine or implementation. The reader mode benchmark cannot be performed.
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack ipstack | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 0.236 Gbps
client io mode: writer, throughput: 0.241 Gbps
client io mode: writer, throughput: 0.261 Gbps
client io mode: writer, throughput: 0.248 Gbps
client io mode: writer, throughput: 0.242 Gbps
client io mode: writer, throughput: 0.238 Gbps
client io mode: writer, throughput: 0.239 Gbps
client io mode: writer, throughput: 0.236 Gbps
client io mode: writer, throughput: 0.212 Gbps
client io mode: writer, throughput: 0.241 Gbps
```

## Netstack Smoltcp

### Single-Thread Server

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack smoltcp --current-thread | Select-String -Pattern "Gbps" }
```

```
client io mode: reader, throughput: 0.308 Gbps
client io mode: reader, throughput: 0.374 Gbps
client io mode: reader, throughput: 0.264 Gbps
client io mode: reader, throughput: 0.380 Gbps
client io mode: reader, throughput: 0.332 Gbps
client io mode: reader, throughput: 0.352 Gbps
client io mode: reader, throughput: 0.338 Gbps
client io mode: reader, throughput: 0.386 Gbps
client io mode: reader, throughput: 0.356 Gbps
client io mode: reader, throughput: 0.386 Gbps
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack smoltcp --current-thread | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 2.834 Gbps
client io mode: writer, throughput: 2.529 Gbps
client io mode: writer, throughput: 2.684 Gbps
client io mode: writer, throughput: 2.965 Gbps
client io mode: writer, throughput: 2.765 Gbps
client io mode: writer, throughput: 2.549 Gbps
client io mode: writer, throughput: 2.705 Gbps
client io mode: writer, throughput: 2.844 Gbps
client io mode: writer, throughput: 2.963 Gbps
client io mode: writer, throughput: 2.819 Gbps
```

### Multi-Thread Server
```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode reader --netstack smoltcp | Select-String -Pattern "Gbps" }
```

```
client io mode: reader, throughput: 0.223 Gbps
client io mode: reader, throughput: 0.248 Gbps
client io mode: reader, throughput: 0.246 Gbps
client io mode: reader, throughput: 0.290 Gbps
client io mode: reader, throughput: 0.324 Gbps
client io mode: reader, throughput: 0.303 Gbps
client io mode: reader, throughput: 0.370 Gbps
client io mode: reader, throughput: 0.347 Gbps
client io mode: reader, throughput: 0.394 Gbps
client io mode: reader, throughput: 0.353 Gbps
```

```powershell
cargo build --release --quiet; 1..10 | ForEach-Object { .\target\release\rust-tun-perf.exe --mode writer --netstack smoltcp | Select-String -Pattern "Gbps" }
```

```
client io mode: writer, throughput: 2.286 Gbps
client io mode: writer, throughput: 2.358 Gbps
client io mode: writer, throughput: 2.229 Gbps
client io mode: writer, throughput: 2.352 Gbps
client io mode: writer, throughput: 2.302 Gbps
client io mode: writer, throughput: 2.312 Gbps
client io mode: writer, throughput: 2.387 Gbps
client io mode: writer, throughput: 2.270 Gbps
client io mode: writer, throughput: 2.327 Gbps
client io mode: writer, throughput: 2.215 Gbps
```
