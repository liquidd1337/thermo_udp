use rand::{thread_rng, Rng};
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

struct SmatrThermo {
    _name: String,
    _status: bool,
    themperature: f32,
}
impl SmatrThermo {
    fn generate_temperature(&mut self) -> f32 {
        let rand: f32 = thread_rng().gen_range(0.0..=1.0);
        self.themperature = match rand <= 0.5 {
            true => 20.0 + rand.sin(),
            false => 20.0 - (rand / 2.0).sin(),
        };
        self.themperature
    }
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let send_addr = args.nth(1).expect("sender address expected");
    let recv_addr = args.next().expect("receiver address expected");

    let sender = UdpSocket::bind(send_addr).await.unwrap();
    let mut thermo = SmatrThermo {
        _name: " Thermo".to_string(),
        _status: true,
        themperature: 0.0,
    };

    loop {
        let temperature = thermo.generate_temperature();
        sender
            .send_to(&temperature.to_be_bytes(), &recv_addr)
            .await
            .unwrap();
        println!("Sent temperature: {:?}", temperature);
        sleep(Duration::from_secs(5)).await
    }
}
