#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_addr: IpAddrKind) {
    println!("route ip addr is {:?}", ip_addr);
    match ip_addr {
        IpAddrKind::V4 => {
            println!("route IPv4");
        }
        IpAddrKind::V6 => {
            println!("route IPv6");
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six)
}
