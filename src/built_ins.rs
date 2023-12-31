use crate::directive::Directive;
use std::env::set_current_dir;

type CmdFn = fn(Vec<Directive>) -> Result<(bool, bool), String>;

#[cfg(target_family = "windows")]
pub const BUILT_INS: [(&'static str, CmdFn); 7] = [
    ("cd", change_directory),
    ("view", view_cmds),
    ("exit", exit_term),
    ("history", view_history),
    ("help", help),
    ("ls", ls),
    ("echo", echo),
];

#[cfg(target_family = "unix")]
pub const BUILT_INS: [(&'static str, CmdFn); 5] = [
    ("cd", change_directory),
    ("view", view_cmds),
    ("exit", exit_term),
    ("history", view_history),
    ("help", help),
];

pub static mut HISTORY: Vec<String> = vec![];

fn view_cmds(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    println!("{:#?}", &directives[1..]);
    Ok((true, true))
}

fn change_directory(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    let new_dir = &directives[0].args;
    if new_dir.len() > 1 {
        return Err(String::from("Invalid syntax"));
    }
    if let Err(e) = set_current_dir(&new_dir[0]) {
        return Err(format!("{e}"));
    };
    Ok((true, true))
}

fn exit_term(_: Vec<Directive>) -> Result<(bool, bool), String> {
    Ok((false, true))
}

fn view_history(_: Vec<Directive>) -> Result<(bool, bool), String> {
    unsafe { println!("{HISTORY:#?}") }
    Ok((true, true))
}

fn help(_: Vec<Directive>) -> Result<(bool, bool), String> {
    println!("This s the CSCI-442 shell (but in rust)!\n");
    println!("The built-in commands are:");
    let cmds = BUILT_INS
        .iter()
        .map(|f| f.0)
        .collect::<Vec<&str>>()
        .join(" ");
    println!("  {cmds}");
    Ok((true, true))
}

#[cfg(target_family = "windows")]
fn ls(_: Vec<Directive>) -> Result<(bool, bool), String> {
    use std::env::current_dir;
    if let Ok(curdir) = current_dir() {
        if let Ok(readir) = curdir.read_dir() {
            for direntry in readir {
                let file = direntry.unwrap().file_name();
                let file = file.to_string_lossy();
                println!("{file}");
            }
        };
    };
    Ok((true, true))
}

#[cfg(target_family = "windows")]
fn echo(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    let first = &directives[0];
    println!("{}", first.args.join(" "));
    Ok((true, true))
}
