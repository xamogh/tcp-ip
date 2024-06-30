use std::io;

fn main() -> io::Result<()> {
    let iface =
        tun_tap::Iface::new("mytun", tun_tap::Mode::Tun).expect("Failed to create a TUN device");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = iface.recv(&mut buf[..])?;

        // https://www.kernel.org/doc/Documentation/networking/tuntap.txt
        let flag = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        match proto {
            // 800 is ipv4
            0x0800 => {
                match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
                    Ok(packet) => {
                        eprintln!(
                            "Flag: {:x}, Proto: {:x}, Source Addr: {}, Destination Addr: {}",
                            flag,
                            proto,
                            packet.source_addr(),
                            packet.destination_addr()
                        )
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
