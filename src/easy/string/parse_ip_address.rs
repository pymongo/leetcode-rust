/*! https://leetcode.com/problems/validate-ip-address/
```rust,no_run
fn valid_posix_ip_str(ip: String) -> String {
    match ip.parse::<std::net::IpAddr>() {
        Ok(std::net::IpAddr::V4(_)) => "IPv4",
        Ok(std::net::IpAddr::V6(_)) => "IPv6",
        Err(_) => "Neither"
    }.to_string()
}
assert_eq!(valid_posix_ip_str("01.01.01.01".into()), "IPv4");
assert_eq!(valid_posix_ip_str("2001:db8:85a3:0::8a2E:0370:7334".into()), "IPv6");
```
*/
// use std::net::AddrParseError;
/// 由于Rust孤儿规则限制+std的AddrParseError成员是私有的导致我无法new一个标准库的AddrParseError实例
#[derive(Debug)]
struct IpAddrParseError;

impl std::fmt::Display for IpAddrParseError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("invalid IP address syntax")
    }
}

impl std::error::Error for IpAddrParseError {}

fn valid_ipv4_address(bytes: &[u8]) -> Result<String, IpAddrParseError> {
    fn checked_restore_u8(chunk: &[u8]) -> Option<u8> {
        let mut val = 0_u8;
        for &byte in chunk.iter() {
            if !byte.is_ascii_digit() {
                return None;
            }
            val = val.checked_mul(10)?.checked_add(byte - b'0')?;
        }
        Some(val)
    }
    let chunks = bytes.split(|&byte| byte == b'.');
    let mut chunks_count = 0;
    for chunk in chunks {
        let len = chunk.len();
        if !matches!(len, 1..=3) {
            return Err(IpAddrParseError);
        }
        if chunk[0] == b'0' && len != 1 {
            return Err(IpAddrParseError);
        }
        match checked_restore_u8(chunk) {
            Some(_) => {}
            None => return Err(IpAddrParseError),
        }
        chunks_count += 1;
    }
    if chunks_count != 4 {
        return Err(IpAddrParseError);
    }
    Ok("IPv4".to_string())
}

fn valid_ipv6_address(bytes: &[u8]) -> Result<String, IpAddrParseError> {
    let chunks = bytes.split(|&byte| byte == b':');
    let mut chunks_count = 0;
    for chunk in chunks {
        if !matches!(chunk.len(), 1..=4) {
            return Err(IpAddrParseError);
        }
        if chunk
            .iter()
            .any(|&byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'))
        {
            return Err(IpAddrParseError);
        }
        chunks_count += 1;
    }
    if chunks_count != 8 {
        return Err(IpAddrParseError);
    }
    Ok("IPv6".to_string())
}

fn parse_ip_address_helper(ip: String) -> Result<String, IpAddrParseError> {
    let ip = ip.into_bytes();
    for &byte in &ip {
        match byte {
            b'.' => return valid_ipv4_address(&ip),
            b':' => return valid_ipv6_address(&ip),
            _ => {}
        }
    }
    Err(IpAddrParseError)
}

/// entrance
fn valid_ip_address(ip: String) -> String {
    parse_ip_address_helper(ip).unwrap_or_else(|_| "Neither".to_string())
}

#[test]
fn test_valid_ip_address() {
    const TEST_CASES: [(&str, &str); 5] = [
        ("2001:0db8:85a3:0:0:8A2E:0370:7334", "IPv6"),
        ("172.16.254.1", "IPv4"),
        ("256.256.256.256", "Neither"),
        ("01.01.01.01", "Neither"), // POSIX's IPv4, but not leetcode
        ("2001:db8:85a3:0::8a2E:0370:7334", "Neither"), // POSIX的IPv6是可以省略0的，但是leetcoe的测试用例不行
    ];
    for (ip, expected) in TEST_CASES {
        assert_eq!(valid_ip_address(ip.to_string()), expected);
    }
}
