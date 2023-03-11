enum IpAddr {
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
