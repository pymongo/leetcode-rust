fn solution<R, W>(mut reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>>
where
    R: std::io::Read,
    W: std::io::Write,
{
    // max input len testcase is "100\r\n"([49,48,48,13(CR),10(LF)])
    let mut read_buffer = [0u8; 5];
    let _read_size = reader.read(&mut read_buffer)?;
    let mut num = 0u8;
    for &byte in read_buffer.iter() {
        if byte == b'\r' || byte == b'\n' {
            break;
        }
        num = num * 10 + byte - b'0';
    }
    if num % 2 == 0 && num != 2 {
        write!(&mut writer, "YES")?;
    } else {
        write!(&mut writer, "NO")?;
    };
    Ok(())
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}
