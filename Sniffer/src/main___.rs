use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::datalink::{self, NetworkInterface};

fn main() {
    let (mut tx, mut rx) = match pnet::datalink::channel(&pnet::datalink::interfaces()[0], Default::default()) {
        Ok(pnet::datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create channel: {}", e),
    };
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet: &dyn pnet::packet::Packet = pnet::packet::Packet::new(packet).unwrap();
                match packet.get_ethertype() {
                    Some(ethertype) => {
                        match packet.get_ethertype() {
                            Some(ethertype) => {
                                match ethertype {
                                    pnet::packet::EtherTypes::Ipv4 => {
                                        let ipv4_packet = Ipv4Packet::new(packet.payload()).unwrap();
                                        let src = ipv4_packet.get_source();
                                        let dst = ipv4_packet.get_destination();
                                        match ipv4_packet.get_next_level_protocol() {
                                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                                                let tcp_packet = TcpPacket::new(ipv4_packet.payload()).unwrap();
                                                println!(
                                                    "TCP: {}:{} -> {}:{}",
                                                    src,
                                                    tcp_packet.get_source(),
                                                    dst,
                                                    tcp_packet.get_destination()
                                                );
                                            }
                                            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                                                let udp_packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
                                                println!(
                                                    "UDP: {}:{} -> {}:{}",
                                                    src,
                                                    udp_packet.get_source(),
                                                    dst,
                                                    udp_packet.get_destination()
                                                );
                                            }
                                            pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                                                let icmp_packet = IcmpPacket::new(ipv4_packet.payload()).unwrap();
                                                println!("ICMP: {} -> {}", src, dst);
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
            Err(e) => {
                println!("Failed to read: {}", e);
            }
        }
    }
}