use std::{io::Write, process::Command};
pub(crate) fn menu(menu_items: Vec<String>, rofi_theme: Option<&str>) -> String {
    let command = &mut Command::new("rofi");
    if let Some(theme) = rofi_theme {
        command.arg("-theme").arg("-dmenu").arg(theme);
    };
    command
        .arg("-dmenu")
        .arg("-i")
        .arg("-l")
        .arg("10")
        .arg("-p")
        .arg("Select a tmuxp configuration:")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            let menu_items_str = menu_items.join("\n");
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(menu_items_str.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        })
        .unwrap_or_default()
}
