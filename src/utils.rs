use log::debug;
use std::process::Command;

pub fn get_version(repo_path: &str) -> String {
    debug!("Get Version");
    debug!("version: {}", repo_path);

    let mut argument = r#"grep -Eo "\"version\":\s\"(.*)" "#.to_owned();
    argument.push_str(&repo_path);
    argument.push_str(r#"/package.json | grep -Eo "[0-9]*\.[0-9]*\.[0-9]*""#);
    debug!("Argument: {:?}", argument);
    let version = Command::new("sh")
        .arg("-c")
        .arg(&argument)
        .output()
        .expect("Failed to execute command");
    debug!("Version: {:?}", version);
    let version_string = std::str::from_utf8(&version.stdout)
        .unwrap()
        .trim_start()
        .trim_end();

    debug!("Version string: {:?}", version_string);
    return version_string.to_string();
}

pub fn get_last_entry(repo_path: &str) -> String {
    let mut argument = r#"tail -n 3 "#.to_owned();
    argument.push_str(&repo_path);
    argument.push_str(r#"/TYPESCRIPT_EVOLUTION.csv | head -n 1"#);
    debug!("Last entry argument: {:?}", argument);
    let last_entry = Command::new("sh")
        .arg("-c")
        .arg(&argument)
        .output()
        .expect("Failed to execute command");

    let last_entry_string = std::str::from_utf8(&last_entry.stdout)
        .unwrap()
        .trim_start()
        .trim_end();

    debug!("Last entry string: {:?}", last_entry_string);
    return last_entry_string.to_string();
}

pub fn check_version_exists(last_entry: &str, version: &str) -> bool {
    debug!("Checking last entry exists: {}", last_entry);
    let mut argument = r#"echo ""#.to_owned();
    argument.push_str(&last_entry);
    argument.push_str(r#"" |  grep "^"#);
    argument.push_str(&version);
    argument.push_str(r#"""#);
    debug!("Checking last entry exists argument: {}", argument);
    let version_exists = Command::new("sh")
        .arg("-c")
        .arg(&argument)
        .output()
        .expect("Failed to execute command");

    let version_exists_string = std::str::from_utf8(&version_exists.stdout)
        .unwrap()
        .trim_start()
        .trim_end();

    debug!("Version already exists: {:?}", version_exists_string.len());
    if version_exists_string.len() > 0 {
        return true;
    }
    return false;
}
