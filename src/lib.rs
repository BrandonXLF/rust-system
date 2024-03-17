use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub trait System<T> {
    fn system(command: &str) -> T;
}

impl System<Command> for Command {
    #[cfg(target_os = "windows")]
    fn system(command: &str) -> Command {
        let mut rust_command = Command::new("cmd");
        rust_command.arg("/c");
        rust_command.raw_arg(&command);
        return rust_command;
    }

    #[cfg(not(target_os = "windows"))]
    fn system(command: &str) -> Command {
        let mut rust_command = Command::new("sh");
        rust_command.arg("-c");
        rust_command.arg(&command);
        return rust_command;
    }
}

pub fn system(command: &str) -> io::Result<Output> {
    Command::system(command).output()
}
