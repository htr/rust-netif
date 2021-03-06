extern crate netif_bpf;

use std::io::Write;

fn main() {
    let mut bpf = netif_bpf::Bpf::new("en0").unwrap();

    let packet = vec![0u8; 512];
    bpf.write_all(&packet).unwrap();
    println!("Packet sent!");
}
