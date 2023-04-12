extern crate pnet;

use pnet::packet::{ipv4::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket, icmp::IcmpPacket};
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};

use std::env;

// Invoke as echo <interface name>
fn main() {
    let interface_name = "Wi-Fi"; // Change this to the name of your WLAN interface
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                match packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
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
            },
            Err(e) => {
                println!("Failed to read packet: {}", e);
            }
        }
    }
}