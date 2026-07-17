use std::net::UdpSocket;

/// 获取局域网 IP 地址，通过 UDP 连接外部来判断出口网卡
pub fn local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let addr = socket.local_addr().ok()?;
    Some(addr.ip().to_string())
}

/// 获取完整的局域网监听地址 "ip:port"
pub fn local_addr(port: u16) -> Option<String> {
    let ip = local_ip()?;
    Some(format!("{ip}:{port}"))
}

/// 绑定地址 "0.0.0.0:{port}"
pub fn bind_addr(port: u16) -> String {
    format!("0.0.0.0:{port}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_addr() {
        assert_eq!(bind_addr(3000), "0.0.0.0:3000");
        assert_eq!(bind_addr(8080), "0.0.0.0:8080");
    }

    #[test]
    fn test_local_ip() {
        let ip = local_ip();
        assert!(ip.is_some());
        let ip = ip.unwrap();
        assert!(!ip.is_empty());
        assert!(ip.contains('.'));
    }
}
