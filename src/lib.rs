//! Cross-platform crate to easily run shell commands, similar to the C `system` command.

use std::{
    io,
    process::{Command, ExitStatus, Output},
};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Trait to allow for the creation of objects representing a shell commands.
pub trait System<T> {
    /// Constructs a new `T` that runs the given shell command when executed.
    fn system(command: &str) -> T;
}

impl System<Command> for Command {
    #[cfg(target_os = "windows")]
    /// Constructs a new [Command] that runs the given shell command when executed.
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
    ///     let stdout = String::from_utf8_lossy(&out.stdout);
    ///
    ///     #[cfg(target_os = "windows")]
    ///     assert_eq!(stdout, "Hello, world!\r\n");
    ///
    ///     #[cfg(not(target_os = "windows"))]
    ///     assert_eq!(stdout, "Hello, world!\n");
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
    ///     let stdout = String::from_utf8_lossy(&out.stdout);
    ///
    ///     #[cfg(target_os = "windows")]
    ///     assert_eq!(stdout, "Hello, world!\r\n");
    ///
    ///     #[cfg(not(target_os = "windows"))]
    ///     assert_eq!(stdout, "Hello, world!\n");
    /// }
    /// ```
    fn system(command: &str) -> Command {
        let mut rust_command = Command::new("sh");
        rust_command.arg("-c");
        rust_command.arg(&command);
        return rust_command;
    }
}

/// Run a shell command and return the [ExitStatus].
///
/// Stdin, stdout, and stderr are inherited from the parent.
///
/// # Example
/// ```rust
/// use system::system;
///
/// fn main() {
///     // Prints "Hello, world!"
///     system("echo Hello, world!").expect("Failed to run command.");
/// }
/// ```
pub fn system(command: &str) -> io::Result<ExitStatus> {
    Command::system(command).status()
}

/// Run a shell command, capture its output, and return the [Output].
///
/// Stdout and stderr are captured and stdin is not inherited.
///
/// # Example
/// ```rust
/// use system::system_output;
///
/// fn main() {
///     let out = system_output("echo Hello, world!").expect("Failed to run command.");
///     let stdout = String::from_utf8_lossy(&out.stdout);
///
///     #[cfg(target_os = "windows")]
///     assert_eq!(stdout, "Hello, world!\r\n");
///
///     #[cfg(not(target_os = "windows"))]
///     assert_eq!(stdout, "Hello, world!\n");
/// }
/// ```
pub fn system_output(command: &str) -> io::Result<Output> {
    Command::system(command).output()
}
