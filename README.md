# DNS Server in Rust

Lightweight and modular DNS server implementation in Rust, designed to handle basic DNS queries, encode and decode messages, and support customisable functionality. It supports functionalities such as parsing headers, questions, and resource records.

## Features

- **DNS Message Parsing:**

  - Decodes and encodes DNS messages according to the DNS protocol specification.
  - Handles headers, questions, and resource records.

- **Big-Endian Encoding and Decoding:**

  - Converts integers (e.g., `u16`, `u32`) to and from big-endian byte arrays.
  - Encodes IP addresses and domain names into byte streams.

## Quick Start

### Prerequisites

- Rust (minimum version: 1.70)

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building the Project

Clone the repository:

```bash
git clone https://github.com/yourusername/dns-server.git
cd dns-server
```

Build the project:

```bash
cargo build
```

### Running the DNS Server

Run the server:

```bash
cargo run
```

## Acknowledgments

- [Rust Programming Language](https://www.rust-lang.org/)
- [DNS Protocol Specification (RFC 1035)](https://www.ietf.org/rfc/rfc1035.txt)

---

Feel free to explore, modify, and use this DNS server as a learning resource or a building block for more advanced projects!

