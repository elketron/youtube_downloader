use std::process::Command;

pub fn get_clipboard_and_empty() -> String {
    let command = Command::new("wl-paste")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    let command = Command::new("wl-copy")
        .arg("")
        .output()
        .expect("failed to execute process");

    output.to_string()
}
