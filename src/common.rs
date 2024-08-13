use clap::{Parser, ValueEnum};

pub const MTU: u16 = 1500;
pub const AMOUNT: usize = 1_000_000_000;
pub const BUFFER_SIZE: usize = 1_000_000;
pub const CLIENT_READER_PORT: u16 = 1234;
pub const CLIENT_WRITER_PORT: u16 = 1235;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Client IO mode: reader/writer.
    #[arg(short = 'm', long, value_enum)]
    pub mode: Client,

    /// Server netstack for tun device.
    #[arg(short = 'n', long, value_enum)]
    pub netstack: Netstack,

    /// Server run on current-thread runtime.
    #[arg(long, default_value_t = false)]
    pub current_thread: bool,

    /// Tun device fd.
    #[arg(long, default_value = None)]
    pub tun_fd: Option<i32>,

    /// Tun device name.
    #[arg(long, default_value = "utun8")]
    pub tun_name: String,

    /// Tun IPv4 address.
    #[arg(long, default_value = "10.10.10.9")]
    pub tun_addr: String,

    /// Tun IPv4 gateway.
    #[arg(long, default_value = "10.10.10.1")]
    pub tun_dest: String,

    /// Tun IPv4 netmask.
    #[arg(long, default_value = "255.255.255.0")]
    pub tun_mask: String,
}

pub fn parse_cli_args() -> Args {
    Args::parse()
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Client {
    Reader,
    Writer,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Client::Reader => f.write_str("reader"),
            Client::Writer => f.write_str("writer"),
        }
    }
}

impl std::str::FromStr for Client {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reader" | "READER" => Ok(Self::Reader),
            "writer" | "WRITER" => Ok(Self::Writer),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid client mode {s}"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Netstack {
    Lwip,
    Ipstack,
    Smoltcp,
}

impl std::fmt::Display for Netstack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Netstack::Lwip => f.write_str("lwip"),
            Netstack::Ipstack => f.write_str("ipstack"),
            Netstack::Smoltcp => f.write_str("smoltcp"),
        }
    }
}

impl std::str::FromStr for Netstack {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lwip" | "LWIP" => Ok(Self::Lwip),
            "ipstack" | "IPSTACK" => Ok(Self::Ipstack),
            "smoltcp" | "SMOLTCP" => Ok(Self::Smoltcp),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid netstack {s}"),
            )),
        }
    }
}

pub fn is_test_stream(remote_addr: std::net::SocketAddr, tun_dest: std::net::IpAddr) -> bool {
    remote_addr.ip() == tun_dest
        && matches!(remote_addr.port(), CLIENT_READER_PORT | CLIENT_WRITER_PORT)
}

pub fn create_tun_device(args: Args) -> std::io::Result<tun::AsyncDevice> {
    let mut config = tun::Configuration::default();
    if let Some(fd) = args.tun_fd {
        config.raw_fd(fd);
    } else {
        config
            .tun_name(args.tun_name)
            .address(args.tun_addr.clone())
            .destination(args.tun_dest)
            .netmask(args.tun_mask)
            .mtu(MTU)
            .up();

        #[cfg(target_os = "linux")]
        config.platform_config(|config| {
            #[allow(deprecated)]
            config.packet_information(true);
        });

        #[cfg(target_os = "windows")]
        config.platform_config(|config| {
            config.device_guid(9099482345783245345345_u128);
        });
    }

    let mut last_err = std::io::Error::other("failed to create tun device");
    for _ in 0..3 {
        match tun::create_as_async(&config) {
            Ok(dev) => {
                return Ok(dev);
            }
            Err(err) => {
                last_err = err.into();
            }
        };
    }
    Err(last_err)
}

pub fn create_tcp_client(args: Args) -> std::io::Result<std::net::TcpStream> {
    let mut last_err = std::io::Error::other("failed to create tcp client");
    let addr = (
        args.tun_dest,
        match args.mode {
            Client::Reader => CLIENT_READER_PORT,
            Client::Writer => CLIENT_WRITER_PORT,
        },
    );
    for _ in 0..5 {
        match std::net::TcpStream::connect(&addr) {
            Ok(stream) => return Ok(stream),
            Err(err) => {
                last_err = err;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Err(last_err)
}

mod wait_group {
    use std::{
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        task::{Context, Poll, Waker},
    };

    pub struct AsyncWaitGroup {
        worker: Worker,
    }

    impl Default for AsyncWaitGroup {
        fn default() -> Self {
            Self {
                worker: Worker {
                    waker: Default::default(),
                    count: Default::default(),
                },
            }
        }
    }

    impl AsyncWaitGroup {
        /// Create a worker for waiting it done.
        pub fn worker(&self) -> Worker {
            self.worker.clone()
        }

        /// Owned self to wait all worker done.
        pub async fn wait(self) {
            WaitGroupFuture { waiter: self }.await
        }
    }

    struct WaitGroupFuture {
        waiter: AsyncWaitGroup,
    }

    impl Future for WaitGroupFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.waiter.worker.count.load(Ordering::Relaxed) == 0 {
                return Poll::Ready(());
            }
            if let Some(old_waker) = self
                .waiter
                .worker
                .waker
                .lock()
                .unwrap()
                .replace(cx.waker().clone())
            {
                if !old_waker.will_wake(cx.waker()) {
                    old_waker.wake();
                }
            }
            if self.waiter.worker.count.load(Ordering::Relaxed) == 0 {
                return Poll::Ready(());
            }
            Poll::Pending
        }
    }

    pub struct Worker {
        waker: Arc<Mutex<Option<Waker>>>,
        count: Arc<AtomicUsize>,
    }

    impl Worker {
        /// Private clone just for waiter.
        fn clone(&self) -> Worker {
            self.count.fetch_add(1, Ordering::Relaxed);
            Self {
                waker: self.waker.clone(),
                count: self.count.clone(),
            }
        }

        /// Owned self to make it done.
        pub fn done(self) {
            drop(self)
        }
    }

    impl Drop for Worker {
        fn drop(&mut self) {
            if self.count.fetch_sub(1, Ordering::Relaxed) != 1 {
                return;
            }
            if let Some(waker) = self.waker.lock().unwrap().take() {
                waker.wake();
            }
        }
    }
}

pub use wait_group::AsyncWaitGroup;
