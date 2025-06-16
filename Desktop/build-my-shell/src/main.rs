#[allow(unused_imports)]
use std::io::{self, Write};

pub fn prompt_and_echo<R: io::BufRead, W: Write>(mut reader: R, mut writer: W) {
    loop {
        write!(writer, "$ ").unwrap();
        writer.flush().unwrap();

        let mut input = String::new();
        reader.read_line(&mut input).unwrap();

        let trimmed = input.trim();
        let mut args = trimmed.split_whitespace();

        match args.next() {
            Some("quit") => break,
            Some("exit") => match args.next() {
                Some("0") => return,
                Some(arg) => writeln!(writer, "exit {}: command not found", arg).unwrap(),
                None => writeln!(writer, "exit: command not found").unwrap(),
            },
            Some("echo") => {
                let rem: Vec<&str> = args.collect();
                writeln!(writer, "{}", rem.join(" ")).unwrap();
            }
            Some("type") => match args.next() {
                Some(arg) => {
                    if arg == "echo" || arg == "type" || arg == "exit" {
                        writeln!(writer, "{} is a shell builtin", arg).unwrap();
                    } else if let Ok(path_var) = std::env::var("PATH") {
                        let mut found = false;
                        for dir in path_var.split(":") {
                            let full_path = format!("{}/{}", dir, arg);
                            if std::path::Path::new(&full_path).exists() {
                                found = true;
                                writeln!(writer, "{} is {}", arg, full_path).unwrap();
                            }
                        }
                        if !found {
                            writeln!(writer, "{}: not found", arg).unwrap();
                        }
                    } else {
                        writeln!(writer, "{}: not found", arg).unwrap();
                    }
                }
                None => {
                    writeln!(writer, "No argument provided").unwrap();
                }
            },
            // Some(_) => writeln!(writer, "{}: command not found", trimmed).unwrap(),
            Some(rem) => {
                let ex_args: Vec<&str> = args.collect();
                if let Ok(path_var) = std::env::var("PATH") {
                    let mut found = false;
                    for dir in path_var.split(":") {
                        let full_path = format!("{}/{}", dir, rem);
                        if std::path::Path::new(&full_path).exists() {
                            let status = std::process::Command::new(rem).args(&ex_args).status();
                            match status {
                                Ok(_) => {}
                                Err(e) => eprintln!("Failed to execute {}: {}", rem, e),
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        writeln!(writer, "{}: not found", rem).unwrap();
                    }
                } else {
                    println!("{}: not found", rem);
                }
            }

            None => continue,
        }
    }
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
