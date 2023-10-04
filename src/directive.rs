#[derive(Debug, PartialEq, Eq)]
pub struct Directive {
    pub cmd: String,
    pub args: Vec<String>,
    pub output_filename: Option<String>,
    pub input_filename: Option<String>,
}

impl From<&str> for Directive {
    fn from(value: &str) -> Self {
        let mut input_filename = None;
        let mut cmd = String::new();
        let mut args = vec![];
        let mut output_filename = None;

        match &value.split(">").collect::<Vec<&str>>()[..] {
            [first, last] => {
                if first.contains("<") {
                    output_filename = Some(last.trim().to_string());
                    let splits = first.split("<").collect::<Vec<&str>>();
                    let cmd_args = splits[0]
                        .split_ascii_whitespace()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                    cmd = cmd_args[0].to_ascii_lowercase().clone();
                    args = cmd_args[1..].to_vec();
                    input_filename = Some(splits[1].trim().to_string());
                } else if last.contains("<") {
                    let cmd_args = first
                        .split_ascii_whitespace()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                    cmd = cmd_args[0].to_ascii_lowercase().clone();
                    args = cmd_args[1..].to_vec();
                    let splits = last.split("<").collect::<Vec<&str>>();
                    output_filename = Some(splits[0].trim().to_string());
                    input_filename = Some(splits[1].trim().to_string());
                } else {
                    let cmd_args = first
                        .split_ascii_whitespace()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                    output_filename = Some(last.trim().to_string());
                    cmd = cmd_args[0].to_ascii_lowercase().clone();
                    args = cmd_args[1..].to_vec();
                }
            }
            [input] => {
                if input.contains("<") {
                    let splits = input.split("<").collect::<Vec<&str>>();
                    let cmd_args = splits[0]
                        .split_ascii_whitespace()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                    cmd = cmd_args[0].to_ascii_lowercase().clone();
                    args = cmd_args[1..].to_vec();
                    input_filename = Some(splits[1].trim().to_string());
                } else {
                    let cmd_args = input
                        .split_ascii_whitespace()
                        .map(|f| f.trim().to_string())
                        .collect::<Vec<String>>();
                    cmd = cmd_args[0].to_ascii_lowercase().clone();
                    args = cmd_args[1..].to_vec();
                }
            }
            _ => {}
        }

        Self {
            cmd,
            args,
            output_filename,
            input_filename,
        }
    }
}
