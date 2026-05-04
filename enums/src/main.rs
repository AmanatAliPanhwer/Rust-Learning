#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    address: String,
    kind: IpAddrKind,
}

fn main() {
    let google_address = IpAddress {
        address: String::from("1.2.3.4"),
        kind: IpAddrKind::V4,
    };

    route(google_address);
}

fn route(ip: IpAddress) {
    println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);
}