use std::process;

pub struct ChildProcess {
    pub process: process::Child,
    pub options: ProcessOptions,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessOptions {
    pub exit_on_error: bool,
    pub exit_on_success: bool,
    pub strip_ansi: bool,
    pub resolve_names_to_executables: bool
}
