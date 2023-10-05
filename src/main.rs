use directive::Directive;
use std::io::{stdin, stdout, Write};
use whoami::{username, devicename};
use std::env::current_dir;
use execution::handle_directives;

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
        let directives = process_input(&s);
        match handle_directives(directives) {
            Ok((false, _)) => break,                    // On success, but exit
            Ok((true, true)) => return_status = true,   // On success
            Ok((true, false)) => return_status = false, // On Error, but no error message
            Err(e) => {                         // On Error
                eprintln!("{e}");
                return_status = false;
            },
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
