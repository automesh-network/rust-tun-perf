use std::io::{Read, Write};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::common::{create_tcp_client, Args, Client, AMOUNT, BUFFER_SIZE};

pub fn client(
    mut netstack_ready: Receiver<()>,
    mut server_ready: Receiver<()>,
    shutdown: Sender<()>,
    args: Args,
) {
    netstack_ready.blocking_recv().unwrap();

    let client_mode: Client = args.mode;
    let mut stream = create_tcp_client(args).unwrap();
    let mut buffer = vec![0; BUFFER_SIZE];

    server_ready.blocking_recv().unwrap();

    let start = std::time::Instant::now();

    let mut processed = 0;
    while processed < AMOUNT {
        let length = std::cmp::min(buffer.len(), AMOUNT - processed);
        let result = match client_mode {
            Client::Reader => stream.read(&mut buffer[..length]),
            Client::Writer => stream.write(&buffer[..length]),
        };
        match result {
            Ok(0) => break,
            Ok(result) => {
                // print!("(P:{})", result);
                processed += result;
            }
            Err(err) => panic!("cannot process: {err}"),
        }
    }

    let end = std::time::Instant::now();

    let elapsed = (end - start).as_millis() as f64 / 1000.0;

    println!(
        "client io mode: {}, throughput: {:.3} Gbps",
        client_mode,
        AMOUNT as f64 / elapsed / 0.125e9
    );

    shutdown.blocking_send(()).unwrap();
}
