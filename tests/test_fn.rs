use system::system;

#[test]
fn test() {
    let out = system("echo Hello, world!").expect("Failed to run command.");

    #[cfg(target_os = "windows")]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
}
