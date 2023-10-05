#[derive(Debug, PartialEq, Eq)]
pub enum OutputType {
    Append,
    Truncate,
    Pipe,
    Stdout,
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
    pub output_type: Option<OutputType>,
}

impl From<&str> for Directive {
    fn from(value: &str) -> Self {
        let mut input_filename = None;
        let mut cmd = String::new();
        let mut args = vec![];
        let mut output_filename = None;
        let mut output_type = None;
        
        let mut token = String::new();
        let mut mode = Mode::Cmd;
        let mut have_cmd = false;
        let mut c_iter = value.chars();

        let mut handle_token = |mode: &mut Mode, new_mode: Mode, tok: &mut String, have_cmd: &mut bool| {
            if !tok.is_empty() {
                match mode {
                    Mode::Cmd => {
                        cmd = tok.clone();
                        *have_cmd = true;
                    },
                    Mode::Args => args.push(tok.clone()),
                    Mode::In => input_filename = Some(tok.clone()),
                    Mode::Out => output_filename = Some(tok.clone()),
                }
                tok.clear();
                if !*have_cmd {
                    *mode = Mode::Cmd;
                    return;
                }
            }
            *mode = new_mode;
        };

        while let Some(c) = c_iter.next() {
            match c {
                ' ' if !token.is_empty() => handle_token(&mut mode, Mode::Args, &mut token, &mut have_cmd),
                '<' => handle_token(&mut mode, Mode::In, &mut token, &mut have_cmd),
                '>' => {
                    handle_token(&mut mode, Mode::Out, &mut token, &mut have_cmd);
                    if let Some(c) = c_iter.next() {
                        if c == '>' {
                            output_type = Some(OutputType::Append);
                        } else {
                            output_type = Some(OutputType::Truncate);
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
        handle_token(&mut mode, Mode::Args, &mut token, &mut have_cmd);

        Self {
            cmd,
            args,
            output_filename,
            input_filename,
            output_type,
        }
    }
}
