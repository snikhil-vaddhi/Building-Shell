#[allow(unused_imports)]
use std::io::{self, Write};

pub fn prompt_and_echo<R: io::BufRead, W: Write>(mut reader: R, mut writer: W) {
    write!(writer, "$ ").unwrap();
    writer.flush().unwrap();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    write!(writer, "{}", input).unwrap();
}

fn main() {
    prompt_and_echo(std::io::stdin().lock(), std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_prompt_and_echo() {
        let input = Cursor::new("hello world\n");
        let mut output = Vec::new();

        prompt_and_echo(input, &mut output);

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "$ hello world\n");
    }
}
