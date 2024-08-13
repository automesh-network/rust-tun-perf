use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc::Sender;

use crate::{
    common::{create_tun_device, is_test_stream, Args, AsyncWaitGroup},
    server_async::handle_tcp_stream,
};

pub async fn server(netstack_ready: Sender<()>, server_ready: Sender<()>, args: Args) {
    let client_mode = args.mode;
    let tun_dest = args.tun_dest.parse::<std::net::IpAddr>().unwrap();
    let tun_dev = match create_tun_device(args) {
        Ok(res) => res,
        Err(err) => panic!("create tun device got error: {}", err),
    };

    let mut futs: Vec<futures::future::BoxFuture<'static, ()>> = Vec::new();

    let netstack_wg = AsyncWaitGroup::default();

    let (stack, runner, _, tcp_listener) = netstack_smoltcp::StackBuilder::default()
        .stack_buffer_size(512)
        .enable_tcp(true)
        .tcp_buffer_size(4096)
        .build()
        .unwrap();
    let mut tcp_listener = tcp_listener.unwrap(); // tcp enabled
    if let Some(runner) = runner {
        let netstack_worker = netstack_wg.worker();
        futs.push(Box::pin(async move {
            netstack_worker.done();
            runner.await;
        }));
    }

    let framed = tun_dev.into_framed();
    let (mut tun_sink, mut tun_stream) = framed.split();
    let (mut stack_sink, mut stack_stream) = stack.split();

    // Reads packet from stack and sends to TUN.
    futs.push(Box::pin({
        let netstack_worker = netstack_wg.worker();
        async move {
            netstack_worker.done();
            while let Some(pkt) = stack_stream.next().await {
                match pkt {
                    Ok(pkt) => {
                        if let Err(e) = tun_sink.send(pkt).await {
                            eprintln!("sending packet to tun device failed: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("netstack erorr: {}", e);
                        return;
                    }
                }
            }
        }
    }));

    // Reads packet from TUN and sends to stack.
    futs.push(Box::pin({
        let netstack_worker = netstack_wg.worker();
        async move {
            netstack_worker.done();
            while let Some(pkt) = tun_stream.next().await {
                match pkt {
                    Ok(pkt) => {
                        if let Err(e) = stack_sink.send(pkt).await {
                            eprintln!("sending packet to netstack failed: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("tun device error: {}", e);
                        return;
                    }
                }
            }
        }
    }));

    // Extracts TCP connections from stack and process.
    futs.push(Box::pin({
        let netstack_worker = netstack_wg.worker();
        async move {
            netstack_worker.done();
            // Only accept a test tcp stream (from client thread).
            let mut already_accpeted = false;
            while let Some((stream, local_addr, remote_addr)) = tcp_listener.next().await {
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
