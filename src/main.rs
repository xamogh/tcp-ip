use std::io;

fn main() -> io::Result<()> {
    let iface =
        tun_tap::Iface::new("mytun", tun_tap::Mode::Tun).expect("Failed to create a TUN device");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = iface.recv(&mut buf[..])?;
        eprintln!("{} bytes, data: {:x?}", nbytes, &buf[..nbytes]);
    }
}
