use std::net::Ipv4Addr;

fn ping<A>(address: A)
where
    A: Into<Ipv4Addr>,
{
    let ipv4_address = address.into();
    println!("{:?}", ipv4_address);
}

fn main() {
    ping(Ipv4Addr::new(23, 21, 68, 141));
    ping([66, 146, 219, 98]);
    ping(0xd076eb94);

    let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
    let addr2 = Ipv4Addr::from(0xd076eb94);
    println!("{:?}", addr1);
    println!("{:?}", addr2);

    let text = "UTF-8로만 이루어진 문자열".to_string();
    let bytes: Vec<u8> = text.into();
    println!("{:?}", bytes);
}
