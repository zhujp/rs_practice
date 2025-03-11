use hostname;
use if_addrs::get_if_addrs;
use std::io;
use std::net::Ipv4Addr;

fn main() -> io::Result<()> {
    match get_if_addrs() {
        Ok(ifaces) => {
            for iface in ifaces {
                if let if_addrs::IfAddr::V4(v4_addr) = iface.addr {
                    // 判断是否为内网IP地址
                    if is_private_ipv4(&v4_addr.ip) {
                        println!("Interface: {}, IPv4: {}", iface.name, v4_addr.ip);
                    }
                }
            }
        }
        Err(e) => println!("Failed to get network interfaces: {}", e),
    }

    let host_name = hostname::get()?;
    println!("Hostname: {}", host_name.to_string_lossy());
    Ok(())
}

fn is_private_ipv4(ip: &Ipv4Addr) -> bool {
    let octets = ip.octets();
    // 检查是否在私有IP范围内
    (octets[0] == 10)
        || (octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31))
        || (octets[0] == 192 && octets[1] == 168)
}
