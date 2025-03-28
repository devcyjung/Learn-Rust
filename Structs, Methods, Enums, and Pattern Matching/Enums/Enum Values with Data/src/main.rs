enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn route(&self) {
        match self {
            IpAddr::V4(addr) => {
                println!("route IPv4 {}", addr);
            }
            IpAddr::V6(addr) => {
                println!("route IPv6 {}", addr);
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    home.route();
    loopback.route();
}
