use std::fs::OpenOptions;
use std::io::Read;
use std::process::{Command, Stdio, Child};
use crate::directive::{Directive, FileOutputType};
use crate::built_ins::*;

fn execute_directives(directives: Vec<Directive>) -> Result<(bool, bool), String> {
    let mut in_stream = None;
    let mut out_stream = None;
    let mut last_child: Option<Child> = None;
    for (num, directive) in directives.iter().enumerate() {
        if let Some(ref in_file) = directive.input_filename {
			let Ok(in_f) = OpenOptions::new().read(true).open(in_file) else {
				return Err(String::from("Unable to open input file"));
			};
			in_stream = Some(Stdio::from(in_f));
		}
		if let Some(ref out_file) = directive.output_filename {
			if let Some(FileOutputType::Append) = directive.file_output_type {
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
			if let Some(mut lc) = last_child.take() {
				if let Some(mut stderr) = lc.stderr.take() {
					let mut err_buf = vec![];
					if let Ok(1..) = stderr.read_to_end(&mut err_buf) {
						return Err(String::from_utf8_lossy(&err_buf).to_string());
					};
				}
				cmd.stdin(Stdio::from(lc.stdout.take().unwrap()));
			}
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
    let output = last_child.take().unwrap().wait_with_output().unwrap();
    if output.stderr.len() > 0 {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok((true, output.status.success()))
}

pub fn handle_directives(directives: Vec<Directive>) -> Result<(bool, bool), String> {
	for (bi_s, built_in_fn) in BUILT_INS {
		if bi_s == &directives[0].cmd {
			return built_in_fn(directives);
		}
	}
	execute_directives(directives)
}
