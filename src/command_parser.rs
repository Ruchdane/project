use std::collections::HashMap;
use std::process::Command;

pub fn replace_env(array: Vec<String>, env: &HashMap<String, String>) -> Vec<String> {
    debug!("{:?}", env);
    array
        .iter()
        .map(|arg| match arg.starts_with('$') {
            true => env
                .get(&arg[1..])
                .unwrap_or_else(|| panic!("Couldn't find {} in env", arg))
                .to_owned(),
            false => arg.to_owned(),
        })
        .collect()
}
pub fn command_from_array(mut array: Vec<String>) -> Command {
    let command = array.remove(0);
    let mut command = Command::new(command);
    command.args(array);
    command
}

pub fn parse_command(array: Vec<String>, env: &HashMap<String, String>) -> Command {
    command_from_array(replace_env(array, env))
}
