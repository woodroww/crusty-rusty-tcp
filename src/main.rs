use std::collections::HashMap;
use std::io::Error;
use std::net::Ipv4Addr;

mod tcp;
use tcp::State;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> Result<(), Error> {
    let mut connections: HashMap<Quad, State> = Default::default();
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
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                let ip_proto = ip_header.protocol();
                if ip_proto != 0x06 {
                    // we only want tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_header.slice().len()..nbytes]) {
                    Ok(tcp_header) => {
                        let datai = 4 + ip_header.slice().len() + tcp_header.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcp_header.source_port()),
                                dst: (dst, tcp_header.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_header, tcp_header, &buf[datai..nbytes]);
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);
            }
        }

        // network is big endian
    }
}

