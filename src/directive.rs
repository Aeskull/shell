#[derive(Debug, PartialEq, Eq)]
pub enum FileOutputType {
    Append,
    Truncate,
}

enum Mode {
    Cmd,
    In,
    Out,
    Args,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directive {
    pub cmd: String,
    pub args: Vec<String>,
    pub output_filename: Option<String>,
    pub input_filename: Option<String>,
    pub file_output_type: Option<FileOutputType>,
}

impl Directive {
    pub fn from_str(value: &str) -> Option<Self> {
        let mut input_filename = None;
        let mut cmd = String::new();
        let mut args = vec![];
        let mut output_filename = None;
        let mut file_output_type = None;

        let mut token = String::new();
        let mut mode = Mode::Cmd;
        let mut have_cmd = false;
        let mut c_iter = value.chars();

        let mut handle_token =
            |mode: &mut Mode, new_mode: Mode, tok: &mut String, have_cmd: &mut bool| -> Option<()> {
                if !tok.is_empty() {
                    match mode {
                        Mode::Cmd => {
                            cmd = tok.clone();
                            *have_cmd = true;
                        }
                        Mode::Args => args.push(tok.clone()),
                        Mode::In => {
                            if input_filename.is_some() {
                                eprintln!("Too many input files defined");
                                return None;
                            }
                            input_filename = Some(tok.clone());
                        },
                        Mode::Out => {
                            if output_filename.is_some() {
                                eprintln!("Too many output files defined");
                                return None;
                            }
                            output_filename = Some(tok.clone());
                        },
                    }
                    tok.clear();
                    if !*have_cmd {
                        *mode = Mode::Cmd;
                        return Some(());
                    }
                }
                *mode = new_mode;
                return Some(())
            };

        while let Some(c) = c_iter.next() {
            match c {
                ' ' if !token.is_empty() => {
                    handle_token(&mut mode, Mode::Args, &mut token, &mut have_cmd)?
                }
                '<' => handle_token(&mut mode, Mode::In, &mut token, &mut have_cmd)?,
                '>' => {
                    handle_token(&mut mode, Mode::Out, &mut token, &mut have_cmd)?;
                    if let Some(c) = c_iter.next() {
                        if c == '>' {
                            file_output_type = Some(FileOutputType::Append);
                        } else {
                            file_output_type = Some(FileOutputType::Truncate);
                            if c != ' ' {
                                token.push(c);
                            }
                        }
                    }
                }
                _ if c != ' ' => token.push(c),
                _ => {}
            }
        }
        handle_token(&mut mode, Mode::Args, &mut token, &mut have_cmd)?;

        Some(
            Self {
                cmd,
                args,
                output_filename,
                input_filename,
                file_output_type,
            }
        )
    }
}
