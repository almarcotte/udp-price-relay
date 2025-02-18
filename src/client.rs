use std::sync::Arc;
use crate::message::PriceUpdate;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

pub async fn send_price_update(addr: &str, symbol: &str, price: f64) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let price_update = PriceUpdate {
        symbol: symbol.to_string(),
        price,
        timestamp,
    };

    let msg = serde_json::to_string(&price_update)?;
    socket.send_to(msg.as_bytes(), addr).await?;

    println!("Sent: {:?}", price_update);

    Ok(())
}

pub async fn receive_price_update(addr: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind(addr).await?;
    println!("Listening for price updates on: {}", addr);

    let mut buf = [0u8; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..size]).unwrap();

        if let Ok(price_update) = serde_json::from_str::<PriceUpdate>(msg) {
            println!("Received price update from {}: {:?}", src, price_update);
        }
    }
}

pub async fn start(server_addr: &str, listen_addr: &str) -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);

    let socket_clone = socket.clone();
    tokio::spawn(async move {
        listen_for_updates(socket_clone).await.unwrap();
    });

    send_price_updates_loop(server_addr, socket).await?;

    Ok(())
}

async fn listen_for_updates(socket: Arc<UdpSocket>) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf).await?;
        let msg = std::str::from_utf8(&buf[..size]).unwrap();
        if let Ok(price_update) = serde_json::from_str::<PriceUpdate>(msg) {
            println!("Received price update from {}: {:?}", src, price_update);
        }
    }
}

async fn send_price_updates_loop(server_addr: &str, socket: Arc<UdpSocket>) -> std::io::Result<()> {
    let mut counter = 1;

    loop {
        let price_update = PriceUpdate {
            symbol: "BTC/USD".to_string(),
            price: 50000.0 + (counter as f64),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        let msg = serde_json::to_string(&price_update).unwrap();
        socket.send_to(msg.as_bytes(), server_addr).await?;

        println!("📤 Sent: {:?}", price_update);

        counter += 1;
        sleep(Duration::from_secs(5)).await; // Wait before sending next update
    }
}