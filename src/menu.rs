use crate::prelude::*;
use std::{io::Write, process::Command};
pub trait MenuItem {
    fn display_detail(&self) -> String;
}

pub(crate) fn menu<T>(mut command: Command, menu_items: &[T]) -> Result<Option<usize>>
where
    T: MenuItem,
{
    let result: String = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            let mut menu_items_str = String::new();
            for (index, item) in menu_items.iter().enumerate() {
                menu_items_str.push_str(format!("{}:", index).as_str());
                menu_items_str.push_str(item.display_detail().as_str());
                menu_items_str.push('\n');
            }
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(menu_items_str.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        })?;
    if result.is_empty() {
        Ok(None)
    } else {
        let item_index: usize = result
            .split_once(':')
            .expect("Nothing in string")
            .0
            .parse()?;
        Ok(Some(item_index))
    }
}
