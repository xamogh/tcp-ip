use std::io;

fn main() -> io::Result<()> {
    let iface =
        tun_tap::Iface::new("mytun", tun_tap::Mode::Tun).expect("Failed to create a TUN device");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = iface.recv(&mut buf[..])?;

        // https://www.kernel.org/doc/Documentation/networking/tuntap.txt
        // ethernet level protocal and flags through tun
        let _ether_flag = u16::from_be_bytes([buf[0], buf[1]]);
        let ether_proto = u16::from_be_bytes([buf[2], buf[3]]);

        match ether_proto {
            // 800 is ipv4
            0x0800 => {
                match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
                    Ok(packet) => {
                        let source_addr = packet.source_addr();
                        let destination_addr = packet.destination_addr();
                        match etherparse::TcpHeaderSlice::from_slice(
                            &buf[4 + packet.slice().len()..nbytes],
                        ) {
                            Ok(packet) => {
                                eprintln!(
                                    "Source Addr: {}, Destination Addr: {}, port: {}",
                                    source_addr,
                                    destination_addr,
                                    packet.destination_port()
                                )
                            }
                            Err(_) => {
                                eprintln!("Ignoring non tcp packet")
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Ignoring packet (etherparse cant parse)")
                    }
                };
            }
            _ => {
                eprintln!("Ignoring non ipv4 packet")
            }
        }
    }
}
