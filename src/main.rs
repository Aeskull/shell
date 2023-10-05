use directive::{Directive, OutputType};
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};
use whoami::{username, devicename};
use std::env::current_dir;

mod directive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut return_status = true;
    let user = username();
    let d_name = devicename();
    loop {
        let path = current_dir()?;
        print!("{user}@{d_name} {} {} ", path.display(), get_face(return_status));
        stdout().flush().unwrap();
        let mut s = String::new();
        stdin().read_line(&mut s)?;
        let directives = process_input(&s);
        match execute_directives(directives) {
            Ok((false, _)) => break,
            Err(e) => {
                eprintln!("{e}");
                return_status = false;
            },
            Ok((true, true)) => return_status = true,  // On success
            Ok((true, false)) => return_status = false, // On error
        };
    }
    Ok(())
}

#[inline]
fn get_face(b: bool) -> &'static str {
    if b {
        ":)"
    } else {
        ":("
    }
}

fn process_input(s: &str) -> Vec<Directive> {
    let s = s.trim();
    let directives = s
        .split("|")
        .map(|f| Directive::from(f.trim()))
        .collect::<Vec<Directive>>();
    directives
}

fn change_directory(new_dir: &[String]) -> Result<(bool, bool), String> {
    if new_dir.len() > 1 {
        return Err(String::from("Invalid syntax"));
    }
    if let Err(e) = std::env::set_current_dir(&new_dir[0]) {
        return Err(format!("{e}"));
    };
    Ok((true, true))
}

fn execute_directives(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    let mut in_stream = None;
    let mut out_stream = None;
    let mut last_child: Option<std::process::Child> = None;
    for (num, directive) in directives.iter().enumerate() {
        match directive.cmd.as_str() {
            "exit" => return Ok((false, false)),
            "cd" => return change_directory(&directive.args),
            _ => {
                if num != (directives.len() - 1) && directive.output_filename.is_some() {
                    return Err(String::from("Specified output before the final pipe"));
                }
                if num != 0 && directive.input_filename.is_some() {
                    return Err(String::from("Input specified after the first pipe"));
                }
        
                if let Some(ref in_file) = directive.input_filename {
                    let Ok(in_f) = OpenOptions::new().read(true).open(in_file) else {
                        return Err(String::from("Unable to open input file"));
                    };
                    in_stream = Some(Stdio::from(in_f));
                }
                if let Some(ref out_file) = directive.output_filename {
                    if let Some(OutputType::Append) = directive.output_type {
                        let Ok(out_f) = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(out_file)
                        else {
                            return Err(String::from("Unable to open output file"));
                        };
                        out_stream = Some(Stdio::from(out_f));
                    } else {
                        let Ok(out_f) = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(out_file)
                        else {
                            return Err(String::from("Unable to open output file"));
                        };
                        out_stream = Some(Stdio::from(out_f));
                    };
                }
        
                let mut cmd = Command::new(&directive.cmd);
                cmd.args(&directive.args);
        
                if let Some(in_f) = in_stream.take() {
                    cmd.stdin(in_f);
                } else if num > 0 {
                    cmd.stdin(Stdio::from(last_child.take().unwrap().stdout.unwrap()));
                }
        
                if let Some(out_f) = out_stream.take() {
                    cmd.stdout(out_f);
                } else if num < directives.len() - 1 {
                    cmd.stdout(Stdio::piped());
                }
        
                let child = match cmd.spawn() {
                    Ok(c) => c,
                    Err(e) => return Err(format!("{e}")),
                };
                let _ = last_child.insert(child);
            }
        }
    }
    let output = last_child.take().unwrap().wait_with_output().unwrap().stdout;
    print!("{}", String::from_utf8_lossy(&output));

    Ok((true, true))
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
