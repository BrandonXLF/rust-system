use std::process::{Command, Stdio};

use system::System;

#[test]
fn test() {
    let out = Command::system("echo Hello, world! && echo 123")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to create child process!")
        .wait_with_output()
        .expect("Failed to read stdout");

    #[cfg(target_os = "windows")]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world! \r\n123\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n123\n");
}