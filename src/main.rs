use directive::Directive;
use std::io::stdin;
use std::process::{Command, Stdio};

mod directive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s)?;
        let directives = process_input(&s);
        match execute_directives(directives) {
            Ok((false, _)) => break,
            Err(e) => println!("{e:?}"),
            Ok((true, true)) => continue, // On success
            Ok((true, false)) => continue, // On error
        };
    }
    Ok(())
}

fn process_input(s: &str) -> Vec<Directive> {
    let s = s.trim();
    let directives = s
        .split("|")
        .map(|f| Directive::from(f.trim()))
        .collect::<Vec<Directive>>();
    directives
}

fn execute_directives(directives: Vec<Directive>) -> Result<(bool, bool), Box<dyn std::error::Error>> {
    if directives.iter().any(|f| f.cmd == "exit") {
        return Ok((false, false));
    }
    match &directives[..] {
        [first, middle @ .., last] => {
            let mut first_cmd = Command::new(first.cmd.clone());
            first_cmd.args(&first.args);
            if let Some(ref input) = first.input_filename {
                let f = std::fs::OpenOptions::new().read(true).open(input)?;
                first_cmd.stdin(Stdio::from(f));
            }
            first_cmd.stdout(Stdio::piped());
            let output = first_cmd.output()?;
            if output.stderr.len() > 0 {
                println!("{}", String::from_utf8(output.stderr)?);
                return Ok((true, false));
            }

            for cmd in middle {
                let mut mid_cmd = Command::new(cmd.cmd.clone());
                mid_cmd.args(&cmd.args);
                mid_cmd.stdin(Stdio::piped());
                mid_cmd.stdout(Stdio::piped());

                let output = first_cmd.output()?;
                if output.stderr.len() > 0 {
                    println!("{}", String::from_utf8(output.stderr)?);
                    return Ok((true, false));
                }
            }

            let mut last_cmd = Command::new(last.cmd.clone());
            last_cmd.args(&last.args);
            if let Some(ref output) = last.output_filename {
                let f = std::fs::OpenOptions::new().write(true).create(true).open(output)?;
                last_cmd.stdout(Stdio::from(f));
            }
            last_cmd.stdin(Stdio::piped());
            let output = last_cmd.output()?;
            if output.stderr.len() > 0 {
                println!("{}", String::from_utf8(output.stderr)?);
                return Ok((true, false));
            } else if output.stdout.len() > 0 {
                println!("{}", String::from_utf8(output.stdout)?);
            }
        },
        [command] => {
            let mut cmd = Command::new(command.cmd.clone());
            dbg!(command);
            cmd.args(&command.args);
            if let Some(ref input) = command.input_filename {
                let f = std::fs::OpenOptions::new().read(true).open(input)?;
                cmd.stdin(Stdio::from(f));
            }
            if let Some(ref output) = command.output_filename {
                let f = std::fs::OpenOptions::new().write(true).create(true).open(output)?;
                cmd.stdout(Stdio::from(f));
            }
            let output = cmd.output()?;
            if output.stderr.len() > 0 {
                println!("{}", String::from_utf8(output.stderr)?);
                return Ok((true, false));
            } else if output.stdout.len() > 0 {
                println!("{}", String::from_utf8(output.stdout)?);
            }
        },
        _ => {}
    }
    Ok((true, false))
}

#[cfg(test)]
mod tests {
    use crate::process_input;

    #[test]
    fn parse_test() {
        let input = "cat cat_names.txt | sort";
        let dirs = process_input(input);
        dbg!(dirs);
    }

    #[test]
    fn parse_redir() {
        let input = "sort < cat_names.txt > sorted.txt";
        let input2 = "sort > sorted.txt < cat_names.txt";

        let dir = process_input(input);
        let dir2 = process_input(input2);

        assert_eq!(dir[0].cmd, "sort");
        assert_eq!(dir[0].input_filename, Some(String::from("cat_names.txt")));
        assert_eq!(dir[0].output_filename, Some(String::from("sorted.txt")));
        assert_eq!(dir, dir2);
    }
}
