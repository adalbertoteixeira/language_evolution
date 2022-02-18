use log::{debug, info};
use std::process::Command;

pub fn get_version(version: &str, repo_path: &str) -> String {
    debug!("Get Version");
    debug!("Existing version: {}", version);

    let existing_version = version.to_owned();
    if existing_version.len() > 0 {
        return existing_version;
    }

    debug!("Version path: {}", repo_path);

    let mut argument = r#"grep -Eo '\"version\": \"([\d.]*)\"' "#.to_owned();
    argument.push_str(&repo_path);
    argument.push_str(r#"/package.json | grep -Po '[\d\.]*'"#);
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
    debug!("Getting record count");
    let argument = format!(r#"xsv count {}/TYPESCRIPT_EVOLUTION.csv"#, &repo_path);
    debug!("Getting record count argument {:?}", argument);
    let last_entry = Command::new("sh")
        .arg("-c")
        .arg(&argument)
        .output()
        .expect("Failed to execute command");

    let last_entry_string = std::str::from_utf8(&last_entry.stdout)
        .unwrap()
        .trim_start()
        .trim_end();

    debug!("Last entry count string: {:?}", last_entry_string);
    let last_entry_int: i32 = last_entry_string.parse().unwrap();
    debug!("Last entry count int: {:?}", last_entry_int);

    debug!("Checking for last entry record");
    let last_record_index = last_entry_int - 2;
    debug!(
        "Checking for last entry record with index {}.",
        &last_record_index
    );
    let argument = format!(
        r#"xsv slice -i {} -n {}/TYPESCRIPT_EVOLUTION.csv"#,
        last_record_index, &repo_path
    );
    debug!("Checking for last entry record: {:?}", argument);
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

    return last_entry_string.to_owned();
}

pub fn check_version_exists(repo_path: &str, version: &str) -> bool {
    if version.len() > 0 {
        info!("Checking for existing entry with version: {:?}", &version);
        let mut argument = r#"xsv search -s 1 -n ""#.to_owned();
        argument.push_str(&version);
        argument.push_str(r#"" "#);
        argument.push_str(&repo_path);
        argument.push_str("/TYPESCRIPT_EVOLUTION.csv");
        debug!("Checking for existing entry with version: {:?}", argument);
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
        if last_entry_string.to_string().len() > 0 {
            return true;
        }
        return false;
    }
    return false;
}

pub fn check_date_exists(repo_path: &str, date: &str) -> bool {
    if date.len() > 0 {
        info!("Checking for existing entry with date: {:?}", &date);
        let mut argument = r#"xsv search -s 2 -n ""#.to_owned();
        argument.push_str(&date);
        argument.push_str(r#"" "#);
        argument.push_str(&repo_path);
        argument.push_str("/TYPESCRIPT_EVOLUTION.csv");
        debug!("Checking for existing entry with version: {:?}", argument);
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
        if last_entry_string.to_string().len() > 0 {
            return true;
        }
        return false;
    }
    return true;
}

pub fn remove_last_lines(repo_path: &str, dry_run: bool) {
    if dry_run == true {
        return;
    }
    // @TODO check if gnu-sed exists, otherwise add `-i ''`
    let mut remove_last_line = r#"sed -i '$d' "#.to_owned();
    remove_last_line.push_str(repo_path);
    remove_last_line.push_str(r#"/TYPESCRIPT_EVOLUTION.csv"#);
    debug!("Last line removal command is {:?}", &remove_last_line);
    let remove_last_line_result = Command::new("sh")
        .arg("-c")
        .arg(&remove_last_line)
        .output()
        .unwrap();
    debug!("Last line removal result is {:?}", &remove_last_line_result);
    let remove_last_line_result_again = Command::new("sh")
        .arg("-c")
        .arg(remove_last_line)
        .output()
        .unwrap();
    debug!(
        "Last line removal result is {:?}",
        &remove_last_line_result_again
    );
}
