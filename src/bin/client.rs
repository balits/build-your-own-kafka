use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let bytes: [u8; 39] = [
        0x00, 0x00, 0x00, 0x23, // Length (35 bytes after this)
        // BEGIN HEADER
        0x00, 0x12,             // API Key (18 = ApiVersions)
        0x00, 0x04,             // API Version (4)
        0x3a, 0x05, 0x33, 0x0b, // Correlation ID (973419275 in hex: 3a05330b)
        0x00, 0x09,             // Client ID length (9)
        0x6b, 0x61, 0x66, 0x6b, 0x61, 0x2d, 0x63, 0x6c, 0x69, // "kafka-cli"
        // no tag buffer
        // END HEADER
        0x00, 0x0a,             // Client software name length (10)
        0x6b, 0x61, 0x66, 0x6b, 0x61, 0x2d, 0x63, 0x6c, 0x69, // "kafka-cli"
        0x04,                   // Client software version length (4)
        0x30, 0x2e, 0x31, 0x00  // "0.1\0"
    ];

    // Connect to the TCP server
    let mut stream = TcpStream::connect("127.0.0.1:9092")?;
    println!("Connected to server, sending bytes...");

    // Write the raw bytes
    stream.write_all(&bytes)?;
    println!("Bytes sent successfully.");

    let mut buf = vec![0; 512];

    if let Ok(n) = stream.read(&mut buf) {
        println!("Read {n} bytes");
    }

    Ok(())
}
