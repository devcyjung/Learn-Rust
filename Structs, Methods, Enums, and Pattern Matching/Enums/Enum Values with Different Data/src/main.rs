enum IpAddr<'a> {
    V4(IpV4Addr),
    V6(IpV6Addr<'a>),
}

struct IpV4Addr(u8, u8, u8, u8);

impl IpV4Addr {
    fn new(n0: u8, n1: u8, n2: u8, n3: u8) -> IpV4Addr {
        IpV4Addr(n0, n1, n2, n3)
    }
}
struct IpV6Addr<'a> {
    address: &'a str,
}

impl<'a> IpV6Addr<'a> {
    fn new(address: &str) -> IpV6Addr {
        IpV6Addr { address }
    }
}

impl<'a> IpAddr<'a> {
    fn route(&self) {
        match self {
            IpAddr::V4(addr) => {
                println!("IpV4 {}.{}.{}.{}", addr.0, addr.1, addr.2, addr.3);
            }
            IpAddr::V6(addr) => {
                println!("IpV6 {}", addr.address);
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(IpV4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(IpV6Addr::new("::1"));

    home.route();
    loopback.route();
}
