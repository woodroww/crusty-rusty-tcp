use etherparse::{TcpHeader, Ipv4Header};

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        match *self {
            State::Closed => {
                return;
            }
            State::Listen => {
                if !tcp_header.syn() {
                    // only expected syn packet
                    return;
                }
                // need to establish connection
                let syn_ack = TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0,
                    0,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                // syn flood attack ?
                let mut ip = Ipv4Header::new(
                    syn_ack.slice().len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    ip_header.destination_addr(),
                    ip_header.source_addr(),
                );
                
            }
            State::SynRcvd => {}
            State::Estab => {}
        }

        eprintln!(
            "{}:{} -> {}:{} {}b of protocol",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );
    }
}
