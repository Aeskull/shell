use crate::directive::Directive;
use std::env::set_current_dir;

type CmdFn = fn(Vec<Directive>) -> Result<(bool, bool), String>;

pub const BUILT_INS: [(&'static str, CmdFn); 4] = [
	("cd", change_directory),
	("view", view_cmds),
	("exit", exit_term),
	("history", view_history),
];

fn view_cmds(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    for directive in &directives[1..] {
        dbg!(directive);
    }
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
	Ok((true, true))
}
