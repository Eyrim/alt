use std::io;

// delim is basically IFS
pub fn read_stdin(delim: Vec<u8>) -> std::io::Result<Vec<String>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let pattern = delim.iter()
        .map(|b| *b as char)
        .collect::<Vec<char>>();

    let split = buffer.split(pattern.as_slice())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    Ok(split)
}

