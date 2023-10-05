use directive::Directive;
use std::io::{stdin, stdout, Write};
use whoami::{username, devicename};
use std::env::current_dir;
use execution::handle_directives;

use crate::built_ins::HISTORY;

mod directive;
mod built_ins;
mod execution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut return_status = true;
    let user = username();
    let d_name = devicename();
    loop {
        let path = current_dir()?;
        print!("{user}@{d_name} {} {} ", path.display(), get_face(return_status));
        stdout().flush()?;
        let mut s = String::new();
        stdin().read_line(&mut s)?;
        let Some(directives) = process_input(&s) else {
            return_status = false;
            continue;
        };
        match handle_directives(directives) {
            Ok((false, _)) => break,                    // On success, but exit
            Ok((true, true)) => return_status = true,   // On success
            Ok((true, false)) => return_status = false, // On Error, but no error message
            Err(e) => {                         // On Error
                if let Some('\n') = e.chars().last() {
                    eprint!("{e}");
                } else {
                    eprintln!("{e}");
                }
                return_status = false;
            },
        };
        unsafe { HISTORY.push(s.trim().to_owned()) }
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

fn process_input(s: &str) -> Option<Vec<Directive>> {
    let s = s.trim();
    let mut directives = vec![];
    let splits = s.split("|").collect::<Vec<&str>>();
    let l = splits.len();
    for (pos, split) in splits.iter().enumerate() {
        let d = Directive::from_str(split)?;
        if d.output_filename.is_some() && pos < l - 1 {
            eprintln!("Output file defined before final pipe");
            return None;
        }
        if d.input_filename.is_some() && pos != 0 {
            eprintln!("Input file defined after first pipe");
            return None;
        }
        directives.push(d);
    }

    Some(directives)
}
