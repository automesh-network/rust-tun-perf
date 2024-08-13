mod common;

mod client_sync;
mod server_async;

mod server_ipstack;
mod server_lwip;
mod server_smoltcp;

fn main() {
    let args = common::parse_cli_args();

    let (netstack_ready_tx, netstack_ready_rx) = tokio::sync::mpsc::channel(1);
    let (server_ready_tx, server_ready_rx) = tokio::sync::mpsc::channel(1);
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel(1);

    // A tcp test client for server io read/write.
    std::thread::spawn({
        let args = args.clone();
        move || client_sync::client(netstack_ready_rx, server_ready_rx, shutdown_tx, args)
    });

    // A tcp test server for client io read/write.
    server_async::server(netstack_ready_tx, server_ready_tx, shutdown_rx, args);
}
