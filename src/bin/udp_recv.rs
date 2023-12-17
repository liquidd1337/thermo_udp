use tokio::net::UdpSocket;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let recv_addr = args.nth(1).expect("sender address expected");

    let receiver = UdpSocket::bind(recv_addr)
        .await
        .expect("Failed to bind UDP socket");

    let receive_thread = tokio::spawn(async move {
        let mut buf = [0u8; 4];
        loop {
            match receiver.recv_from(&mut buf).await {
                Ok((_size, src)) => {
                    let temperature = f32::from_be_bytes(buf);
                    println!(
                        "Received temperature from sender {}: {:.1}",
                        src, temperature
                    );
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }
            sleep(Duration::from_secs(5)).await
        }
    });

    receive_thread.await.unwrap();
}
