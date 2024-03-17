//! Crate to easily run system/shell commands across different platforms, similar to the C `system` command.

use std::{
    io,
    process::{Command, Output},
};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Trait to allow for the creation of system commands.
pub trait System<T> {
    fn system(command: &str) -> T;
}

impl System<Command> for Command {
    #[cfg(target_os = "windows")]
    /// Construct a new [Command] that runs the given system command when executed.
    ///
    /// See [Command::new] for the default configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::process::Command;
    ///
    /// use system::System;
    ///
    /// fn main() {
    ///     let out = Command::system("echo Hello, world!")
    ///         .output()
    ///         .expect("Failed to run command.");
    ///
    ///     #[cfg(target_os = "windows")]
    ///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");
    ///
    ///     #[cfg(not(target_os = "windows"))]
    ///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
    /// }
    /// ```
    fn system(command: &str) -> Command {
        let mut rust_command = Command::new("cmd");
        rust_command.arg("/c");
        rust_command.raw_arg(&command);
        return rust_command;
    }

    #[cfg(not(target_os = "windows"))]
    /// Construct a new [Command] that runs the given system command when executed.
    ///
    /// See [Command::new] for the default configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::process::Command;
    ///
    /// use system::System;
    ///
    /// fn main() {
    ///     let out = Command::system("echo Hello, world!")
    ///         .output()
    ///         .expect("Failed to run command.");
    ///
    ///     #[cfg(target_os = "windows")]
    ///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");
    ///
    ///     #[cfg(not(target_os = "windows"))]
    ///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
    /// }
    /// ```
    fn system(command: &str) -> Command {
        let mut rust_command = Command::new("sh");
        rust_command.arg("-c");
        rust_command.arg(&command);
        return rust_command;
    }
}

/// Run a system/shell command and return the [Output].
///
/// # Example
/// use system::system;
///
/// ```rust
/// fn main() {
///     let out = system("echo Hello, world!").expect("Failed to run command.");
///
///     #[cfg(target_os = "windows")]
///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");
///
///     #[cfg(not(target_os = "windows"))]
///     assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
/// ```
pub fn system(command: &str) -> io::Result<Output> {
    Command::system(command).output()
}
