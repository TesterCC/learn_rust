use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::process;

/*
Q: write a port scanner by rust

A: code ...
This port scanner takes two command line arguments: a host and a port range. The port range
should be specified in the format start port-end port, for example 1-1024. The port scanner will
attempt to connect to each port in the range and print a message indicating whether it was able to
connect or not.
*/

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <host> <start port>-<end port>", args[0]);
        process::exit(1);
    }

    let host = &args[1];
    let port_range = &args[2];

    let ports = match parse_port_range(port_range) {
        Ok(ports) => ports,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    for port in ports {
        match TcpStream::connect((host, port)) {
            Ok(stream) => {
                println!("Connected to {}:{}", host, port);
                io::copy(&stream, &mut io::sink()).unwrap();
            }
            Err(e) => {
                println!("Failed to connect to {}:{} ({})", host, port, e);
            }
        }
    }
}

fn parse_port_range(port_range: &str) -> Result<Vec<u16>, &str> {
    let parts: Vec<&str> = port_range.split("-").collect();
    if parts.len() != 2 {
        return Err("Invalid port range");
    }

    let start_port = u16::from_str(parts[0]).unwrap();
    let end_port = u16::from_str(parts[1]).unwrap();

    if end_port < start_port {
        return Err("End port must be greater than or equal to start port");
    }

    let ports = (start_port..=end_port).collect();
    Ok(ports)
}
