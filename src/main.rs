use std::io::Error;

fn main() -> Result<(), Error> {
    let nic =
        tun_tap::Iface::new("tun%d", tun_tap::Mode::Tun).expect("Failed to create a TUN device");
    println!("Made a new tun interface {}", nic.name());
    loop {
        let mut buf = [0u8; 1504];
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // we only want ivp4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let ip_proto = p.protocol();
                eprintln!(
                    "{} -> {} {}b of protocol {}",
                    src,
                    dst,
                    p.payload_len(),
                    ip_proto
                );

            }
            Err(e) => {
                eprintln!("ignoring errored packet {:?}", e);
            }
        }

        // network is big endian
    }
}

// from root of project
// sudo setcap CAP_NET_ADMIN=eip ./target/debug/thunder

