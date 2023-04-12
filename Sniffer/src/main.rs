extern crate pnet;

use pnet::packet::{ipv4::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket, icmp::IcmpPacket};
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket, tcp};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};
use colored::Colorize;

use std::env;
use std::io::{self, BufRead};

fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn main() {
    let localaddress = "192.168.2.137";
    let interfaces = datalink::interfaces();
    println!("Available network interfaces:");
    for (i, iface) in interfaces.iter().enumerate() {
        println!("{}: {} ({}), MAC: {:?}", i, iface.name, iface.description, iface.mac);
    }

    let interface = loop {
        let input = read_input("Enter the name or index of the interface: ");
        if let Ok(index) = input.parse::<usize>() {
            if let Some(iface) = interfaces.get(index) {
                break iface.clone();
            }
        } else if let Some(iface) = interfaces.iter().find(|&iface| iface.name == input.trim()) {
            break iface.clone();
        }
        println!("Invalid input, please try again");
    };
    //let interface = interfaces.into_iter()
    //    .filter(|iface| iface.name == interface_name)
    //    .next()
    //    .unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };
    let mut lastincomingaddress: String = "".to_string();
    let mut lastoutgoingaddress: String = "".to_string();
    let mut number: i32 = 1;
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                let mut outputstr: String = "".to_string();
                let mut incoming: bool = false;
                match packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        let ipv4_packet = Ipv4Packet::new(packet.payload()).unwrap();
                        let src = ipv4_packet.get_source();
                        let dst = ipv4_packet.get_destination();
                        match ipv4_packet.get_next_level_protocol() {
                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                                let tcp_packet = TcpPacket::new(ipv4_packet.payload()).unwrap();
                                if format!{"{}", src} == localaddress {
                                    if lastincomingaddress == format!{"{}:{}", src, tcp_packet.get_source()} && lastoutgoingaddress == format!{"{}:{}", dst, tcp_packet.get_destination()} {
                                        number += 1;
                                        outputstr = format!("\rIncoming (->) TCP Packet from: {}:{} to {}:{} [{}]", src, tcp_packet.get_source(), dst, tcp_packet.get_destination(), number);
                                    } else {
                                        outputstr = format!("\nIncoming (->) UDP Packet from: {}:{} to {}:{}", src, tcp_packet.get_source(), dst, tcp_packet.get_destination());
                                        number = 1;
                                    }
                                    incoming = true;
                                    lastincomingaddress = format!{"{}:{}", src, tcp_packet.get_source()};
                                    lastoutgoingaddress = format!{"{}:{}", dst, tcp_packet.get_destination()};
                                } else {
                                    if lastincomingaddress == format!{"{}:{}", src, tcp_packet.get_source()} && lastoutgoingaddress == format!{"{}:{}", dst, tcp_packet.get_destination()} {
                                        number += 1;
                                        outputstr = format!("\rOutgoing (<-) TCP Packet from: {}:{} to {}:{} [{}]", src, tcp_packet.get_source(), dst, tcp_packet.get_destination(), number);
                                    } else {
                                        number = 1;
                                        outputstr = format!("\nOutgoing (<-) TCP Packet from: {}:{} to {}:{}", src, tcp_packet.get_source(), dst, tcp_packet.get_destination());
                                    }
                                    lastincomingaddress = format!{"{}:{}", src, tcp_packet.get_source()};
                                    lastoutgoingaddress = format!{"{}:{}", dst, tcp_packet.get_destination()};
                                }
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                                let udp_packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
                                if format!{"{}", src} == localaddress {
                                    if lastincomingaddress == format!{"{}:{}", src, udp_packet.get_source()} && lastoutgoingaddress == format!{"{}:{}", dst, udp_packet.get_destination()} {
                                        number += 1;
                                        outputstr = format!("\rIncoming (->) UDP Packet from: {}:{} to {}:{} [{}]", src, udp_packet.get_source(), dst, udp_packet.get_destination(), number);
                                    } else {
                                        number = 1;
                                        outputstr = format!("\nIncoming (->) UDP Packet from: {}:{} to {}:{}", src, udp_packet.get_source(), dst, udp_packet.get_destination());
                                    }
                                    incoming = true;
                                    lastincomingaddress = format!{"{}:{}", src, udp_packet.get_source()};
                                    lastoutgoingaddress = format!{"{}:{}", dst, udp_packet.get_destination()};
                                } else {
                                    if lastincomingaddress == format!{"{}:{}", src, udp_packet.get_source()} && lastoutgoingaddress == format!{"{}:{}", dst, udp_packet.get_destination()} {
                                        number += 1;
                                        outputstr = format!("\rOutgoing (<-) UDP Packet from: {}:{} to {}:{} [{}]", src, udp_packet.get_source(), dst, udp_packet.get_destination(), number);
                                    } else {
                                        number = 1;
                                        outputstr = format!("\nOutgoing (<-) UDP Packet from: {}:{} to {}:{}", src, udp_packet.get_source(), dst, udp_packet.get_destination());
                                    }
                                    lastincomingaddress = format!{"{}:{}", src, udp_packet.get_source()};
                                    lastoutgoingaddress = format!{"{}:{}", dst, udp_packet.get_destination()};
                                    
                                }
                                
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                                let icmp_packet = IcmpPacket::new(ipv4_packet.payload()).unwrap();
                                println!("ICMP: {} -> {}", src, dst);
                            }
                            _ => (),
                        }
                        if incoming {
                            print!("{}", outputstr.green());
                        } else {
                            print!("{}", outputstr.yellow());
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