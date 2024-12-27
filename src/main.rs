mod dns;

use clap::Parser;
use dns::packet::DnsPacket;
use std::net::UdpSocket;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    resolver: String,
}

fn main() {
    // Get resolver address from command line arguments
    let config = Args::parse();    
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to DNS server address");
    println!("Starting DNS forwarder. Forwarding to resolver: {}", &config.resolver);
    let mut buffer = [0; 512];
    loop {
        match udp_socket.recv_from(&mut buffer) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let dns_request = DnsPacket::new(&buffer);
                let dns_response = resolve_request_upstream(&udp_socket, &config.resolver, dns_request);
                udp_socket.send_to(&dns_response.to_bytes(), source).expect("Failed to send response back to client");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn resolve_request_upstream(udp_socket: &UdpSocket, upstream_address: &String, dns_request: DnsPacket) -> DnsPacket {
    let upstream_requests = dns_request.split();
    let upstream_replies = upstream_requests
        .into_iter()
        .map(|request| {
            udp_socket.send_to(&request.to_bytes(), upstream_address).expect("Failed to send request upstream");
            let mut forward_buf = [0; 512];
            udp_socket.recv_from(&mut forward_buf).expect("Failed to receive response from upstream");            
            let packet = DnsPacket::new(&forward_buf);            
            packet
        })
        .collect();
    DnsPacket::merge(upstream_replies)
}
