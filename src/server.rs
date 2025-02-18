use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use crate::message::PriceUpdate;
use serde_json;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind(addr).await?);
    println!("UDP server listening on {}", addr);

    let (tx, _rx) = broadcast::channel::<String>(100);

    let mut buf = [0u8; 1024];
    let mut clients: HashSet<SocketAddr> = HashSet::new();

    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..size]).unwrap();

        if clients.insert(src) {
            let rx = tx.subscribe();
            let socket_clone = Arc::clone(&socket);
            tokio::spawn(handle_client(rx, src, socket_clone));
        }

        if let Ok(price_update) = serde_json::from_str::<PriceUpdate>(msg) {
            println!("Received from {}: {:?}", src, price_update);

            let serialized = serde_json::to_string(&price_update)?;
            tx.send(serialized).unwrap();
        }

        // Future: Update client subscriptions
    }
}

async fn handle_client(mut rx: Receiver<String>, addr: SocketAddr, socket: Arc<UdpSocket>) {
    while let Ok(msg) = rx.recv().await {
        if let Err(e) = socket.send_to(&msg.as_bytes(), addr).await {
            eprintln!("Failed to write to {}; error = {:?}", addr, e);
        }
    }
}