use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{Receiver, Sender},
};

use crate::common::{Args, Client, Netstack, AMOUNT, BUFFER_SIZE};

pub fn server(
    netstack_ready: Sender<()>,
    server_ready: Sender<()>,
    mut shutdown: Receiver<()>,
    args: Args,
) {
    let rt = if args.current_thread {
        tokio::runtime::Builder::new_current_thread()
    } else {
        tokio::runtime::Builder::new_multi_thread()
    }
    .enable_all()
    .build()
    .unwrap();

    macro_rules! spawn_server {
        ($server_impl:path) => {
            rt.spawn($server_impl(netstack_ready, server_ready, args))
        };
    }

    // Spawn a tcp server task to serve the tcp test.
    let server_handle = match args.netstack {
        Netstack::Lwip => spawn_server!(crate::server_lwip::server),
        Netstack::Ipstack => spawn_server!(crate::server_ipstack::server),
        Netstack::Smoltcp => spawn_server!(crate::server_smoltcp::server),
    };

    // Waits util the test tcp client thread shutdown.
    rt.block_on(async move {
        shutdown.recv().await.unwrap();
        server_handle.abort();
    });
}

pub async fn handle_tcp_stream<S>(
    client_mode: Client,
    server_ready: Sender<()>,
    mut stream: S,
    local_addr: std::net::SocketAddr,
    remote_addr: std::net::SocketAddr,
) where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    println!("test tcp stream {}<->{}", local_addr, remote_addr);

    let mut processed = 0;
    let mut buffer = vec![0; BUFFER_SIZE];

    // Notify the tcp test client thread to read/write io.
    server_ready.send(()).await.unwrap();

    while processed < AMOUNT {
        if matches!(client_mode, Client::Reader) {
            // println!("Writing data to client");
            let length = std::cmp::min(buffer.len(), AMOUNT - processed);
            let result = stream.write(&buffer[..length]).await;
            match result {
                Ok(0) => break,
                Ok(result) => {
                    // print!("(P:{})", result);
                    processed += result;
                }
                Err(err) => panic!("cannot process: {err}"),
            }
        }

        if matches!(client_mode, Client::Writer) {
            // println!("Reading data from client");
            let length = std::cmp::min(buffer.len(), AMOUNT - processed);
            let result = stream.read(&mut buffer[..length]).await;
            match result {
                Ok(0) => break,
                Ok(result) => {
                    // print!("(P:{})", result);
                    processed += result;
                }
                Err(err) => panic!("cannot process: {err}"),
            }
        }
    }
}
