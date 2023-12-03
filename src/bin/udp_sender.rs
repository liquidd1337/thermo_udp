use std::net::UdpSocket;
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::thread;


struct SmatrThermo {
    _name: String,
    _status: bool,
    themperature: f32,
}
impl SmatrThermo {
    fn generate_temperature(&mut self) -> f32 {
        let rand: f32 = thread_rng().gen_range(0.0..=1.0);
        if rand <= 0.5 {
                self.themperature = 20.0 + (rand).sin();
                return self.themperature
            } else {
                self.themperature = 20.0 - (rand/2.0).sin();
                return self.themperature
            }
        }
}

fn main() {
    let mut args = std::env::args();
    let send_addr = args.nth(1).expect("sender address expected");
    let recv_addr = args.next().expect("receiver address expected");

    let sender = UdpSocket::bind(send_addr).unwrap();
    let mut thermo = SmatrThermo {
        _name: " Thermo".to_string(),
        _status: true,
        themperature: 0.0,
    };
    

    loop {
        let temperature = thermo.generate_temperature();
        sender.send_to(&temperature.to_be_bytes(), &recv_addr).expect("Failed to send temperature");
        println!("Sent temperature : {:.1}", temperature);

        thread::sleep(Duration::from_secs(5));
    }
}