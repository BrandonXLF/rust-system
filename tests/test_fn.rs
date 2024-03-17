use system::system;

#[test]
fn test() {
    let out = system("echo Hello, world!").expect("Failed to run command.");
    let stdout = String::from_utf8_lossy(&out.stdout);

    #[cfg(target_os = "windows")]
    assert_eq!(stdout, "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(stdout, "Hello, world!\n");
}
