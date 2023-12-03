use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

fn main() {
    let mut args = std::env::args();
    let recv_addr = args.nth(1).expect("sender address expected");

    let receiver = UdpSocket::bind(recv_addr).unwrap();

    let receive_thread = thread::spawn(move || {
        let mut buf = [0u8; 4];
        loop {
            match receiver.recv_from(&mut buf) {
                Ok((_size, src)) => {
                    let temperature = f32::from_be_bytes(buf);  
                    println!("Received temperature from sender {}: {:.1}", src, temperature);
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }
            std::thread::sleep(Duration::from_secs(5));
        }
    });

    receive_thread.join().unwrap();  
}