use std::net::{Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use get_if_addrs::{get_if_addrs, IfAddr};

fn ip_available() -> Option<Ipv4Addr> {
    if let Ok(interfaces) = get_if_addrs() {
        for iface in interfaces {
            if iface.name == "wlan0" {
                if let IfAddr::V4(addr) = iface.addr {
                    return Some(addr.ip);
                }
            }
        }
    }
    None
}

fn broadcast_ip(port: u16) {
    if let Some(ip) = ip_available() {
        println!("BroadcastIP is: {}", ip);
        println!("Current PORT: {}", port);

        let sock = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        sock.set_broadcast(true).expect("Failed to set socket as broadcast");

        let message = ip.to_string().into_bytes();
        loop {
            match sock.send_to(&message, (Ipv4Addr::new(255, 255, 255, 255), port)) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to send broadcast message, this proc will get killed immediately");
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn main() {
    let port: u16 = 12346; // Choose a port for broadcasting
    broadcast_ip(port);
}

