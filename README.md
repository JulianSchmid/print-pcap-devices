# print-pcap-devices

Tool to quickly print the devices visible by the PCAP crate.

## Usage Example:

```sh
print-pcap-devices
```

Example Output:

```
Found device! Device { name: "en0", desc: None, addresses: [Address { addr: abcd::abcd:abcd:abcd:abcd, netmask: Some(ffff:ffff:ffff:ffff::), broadcast_addr: None, dst_addr: None }, Address { addr: 1.2.3.4, netmask: Some(255.255.255.0), broadcast_addr: Some(1.2.3.1), dst_addr: None }, Address { addr: abcd:abcd:abcd:abcd:abcd:abcd:abcd:abcd, netmask: Some(ffff:ffff:ffff:ffff::), broadcast_addr: None, dst_addr: None }, Address { addr: abcd:abcd:abcd:abcd:abcd:abcd:abcd:abcd, netmask: Some(ffff:ffff:ffff:ffff::), broadcast_addr: None, dst_addr: None }], flags: DeviceFlags { if_flags: UP | RUNNING | WIRELESS, connection_status: Connected } }
  Main data link: Linktype(1) Ok("EN10MB") Ok("Ethernet")
    Linktype(1) Ok("EN10MB") Ok("Ethernet")
    Linktype(12) Ok("RAW") Ok("Raw IP")
Found device! Device { name: "awdl0", desc: None, addresses: [Address { addr: abcd::abcd:abcd:abcd:abcd, netmask: Some(ffff:ffff:ffff:ffff::), broadcast_addr: None, dst_addr: None }], flags: DeviceFlags { if_flags: UP | RUNNING | WIRELESS, connection_status: Connected } }
  Main data link: Linktype(1) Ok("EN10MB") Ok("Ethernet")
    Linktype(1) Ok("EN10MB") Ok("Ethernet")
    Linktype(147) Err(InvalidLinktype) Err(InvalidLinktype)
    Linktype(12) Ok("RAW") Ok("Raw IP")
Found device! Device { name: "lo0", desc: None, addresses: [Address { addr: 127.0.0.1, netmask: Some(255.0.0.0), broadcast_addr: None, dst_addr: None }, Address { addr: ::1, netmask: Some(ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff), broadcast_addr: None, dst_addr: None }, Address { addr: fe80::1, netmask: Some(ffff:ffff:ffff:ffff::), broadcast_addr: None, dst_addr: None }], flags: DeviceFlags { if_flags: LOOPBACK | UP | RUNNING, connection_status: NotApplicable } }
  Main data link: Linktype(0) Ok("NULL") Ok("BSD loopback")
    Linktype(0) Ok("NULL") Ok("BSD loopback")
```

## Installation:

You will need the Rust compiler & tools installed (e.g. from https://rustup.rs ) and then you can simply run:

```sh
cargo install print-pcap-devices
```

It will then be installed to `~/.cargo/bin`.

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option. The corresponding license texts can be found in the LICENSE-APACHE file and the LICENSE-MIT file.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
