enum IpAddrKind {
    V4,
    V6,
}

//attach data to enum
enum IpAddr {
    V4(String),
    V6(String),
}
//with different types
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
//more flexible way
struct Ipv4Addr {
    arr: (u8, u8, u8, u8),
}
struct Ipv6Addr {
    s: String,
}
enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(dead_code)]
pub fn run() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    //
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
    //
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));
    //
    let ipv4 = IpAddr3::V4(Ipv4Addr {
        arr: (127, 0, 0, 1),
    });
    ipv4.print_self();
    let ipv6 = IpAddr3::V6(Ipv6Addr {
        s: String::from("::1"),
    });
    ipv6.print_self();
    //optional +match
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
    //placeholder match
    match_with_placeholder(1);
    match_with_placeholder(115);
}

//also we can define methods in enums
impl IpAddr3 {
    fn print_self(&self) {
        //method body
        print!("Call method inside enum:");
        match self {
            IpAddr3::V4(ip) => println!(
                "im v4 address: {}.{}.{}.{}.",
                ip.arr.0, ip.arr.1, ip.arr.2, ip.arr.3
            ),
            IpAddr3::V6(ip) => println!("im v6 addresss: {}", ip.s),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_with_placeholder(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("alot"),
    }
}
