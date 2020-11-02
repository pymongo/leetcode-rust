//! https://codeforces.com/problemset/problem/71/A

fn solution<R, W>(reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>>
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(str) = line {
            input.push(str);
        }
    }
    for string in input.iter().skip(1) {
        let len = string.len();
        if len <= 10 {
            writeln!(&mut writer, "{}", string)?;
        } else {
            let bytes = string.as_bytes();
            writeln!(
                &mut writer,
                "{}{}{}",
                bytes[0] as char,
                len - 2,
                *bytes.last().unwrap() as char
            )?;
        }
    }
    Ok(())
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}

#[cfg(test)]
const TESTCASES: [(&[u8], &[u8]); 1] = [
    (b"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis", b"word\nl10n\ni18n\np43s\n"),
];

#[test]
fn test_solution() {
    if cfg!(windows) {
        panic!("This unittest is using LF line_separator, should assertion failed in windows");
    }
    for &(input, expected_output) in &TESTCASES {
        let mut output = Vec::new();
        solution(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}
