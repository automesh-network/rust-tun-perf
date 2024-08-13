use tokio::sync::mpsc::Sender;

use crate::{
    common::{create_tun_device, is_test_stream, Args, AsyncWaitGroup, MTU},
    server_async::handle_tcp_stream,
};

pub async fn server(netstack_ready: Sender<()>, server_ready: Sender<()>, args: Args) {
    let client_mode = args.mode;
    let tun_dest = args.tun_dest.parse::<std::net::IpAddr>().unwrap();
    let tun_dev = match create_tun_device(args) {
        Ok(res) => res,
        Err(err) => panic!("create tun device got error: {}", err),
    };

    let netstack_wg = AsyncWaitGroup::default();

    let mut futs: Vec<futures::future::BoxFuture<'static, ()>> = Vec::new();

    let mut ipstack_config = netstack_ipstack::IpStackConfig::default();
    ipstack_config.mtu(MTU);
    let mut ip_stack = netstack_ipstack::IpStack::new(ipstack_config, tun_dev);

    // Extracts TCP connections from stack and process.
    futs.push(Box::pin({
        let netstack_worker = netstack_wg.worker();
        async move {
            netstack_worker.done();
            // Only accept a test tcp stream (from client thread).
            let mut already_accpeted = false;
            while let Ok(stream) = ip_stack.accept().await {
                // Currently only test tcp stream, for other stream just drop it.
                let netstack_ipstack::stream::IpStackStream::Tcp(stream) = stream else {
                    continue;
                };
                let local_addr = stream.local_addr();
                let remote_addr = stream.peer_addr();
                if !is_test_stream(remote_addr, tun_dest) {
                    continue;
                }
                if already_accpeted {
                    continue;
                } else {
                    already_accpeted = true;
                }
                tokio::spawn(handle_tcp_stream(
                    client_mode,
                    server_ready.clone(),
                    stream,
                    local_addr,
                    remote_addr,
                ));
            }
        }
    }));

    tokio::spawn(async move {
        // Waits util all of netstack tasks is ready.
        netstack_wg.wait().await;
        // Notify the tcp test client thread to connect network.
        netstack_ready.send(()).await.unwrap();
    });

    _ = futures::future::select_all(futs).await;
}
