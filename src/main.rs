// #[macro_use]
extern crate log;
use chrono::{DateTime, Utc};
// use env_logger::Env;
use log::debug;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::process::Command;
use std::str;
use structopt::StructOpt;

mod types;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "language_evolution")]
struct Opt {
    #[structopt(short = "p", long = "repo_path", env, default_value = "")]
    repo_path: String,

    #[structopt(short = "f", long = "folders", default_value = "[.]")]
    folders: String,

    #[structopt(
        short = "v",
        long = "version",
        required_if("date", ""),
        default_value = ""
    )]
    version: String,

    #[structopt(short = "d", long = "date", required_if("version", ""))]
    date: bool,

    #[structopt(short = "r", long = "dry-run")]
    dry_run: bool,
}

fn main() {
    env_logger::init();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let opt = Opt::from_args();
    debug!("Options {:?}", opt);

    let folder_split: Vec<&str> = opt.folders.split(",").collect();
    debug!("Folders to iterate: {:?}", &folder_split);

    // let count_json = r#"{}"#.to_owned();
    let mut languages_vec = Vec::new();
    languages_vec.push(["ts", "ts,tsx"]);
    languages_vec.push(["js", "js,jsx"]);

    // Get current branch
    // let mut branch_arg = r#"cd "#.to_owned();
    // branch_arg.push_str(&opt.repo_path);
    // branch_arg.push_str(r#"  && git rev-parse --abbrev-ref HEAD"#);
    // debug!("Git branch arg is: {:?}", branch_arg);
    // let git_branch = Command::new("sh")
    //     .arg("-c")
    //     .arg(branch_arg)
    //     .output()
    //     .unwrap();
    // debug!("Git branch request is {:?}", git_branch.status);
    // let branch = str::from_utf8(&git_branch.stdout)
    // .unwrap()
    // .strip_suffix("\n")
    // .unwrap();
    // debug!("Git branch is {:?}", branch);
    let mut json_counts = r#"{"#.to_owned();

    let utc: DateTime<Utc> = Utc::now();
    let current_date = &utc.format("%Y-%m-%d").to_string();
    let current_version = &opt.version;
    let last_version_exists = utils::check_version_exists(&opt.repo_path, &current_version);
    debug!("Last entry exists: {:?}", last_version_exists);

    if last_version_exists == true {
        writeln!(
            handle,
            "File already has an entry for version {}.",
            &current_version
        )
        .unwrap();
        return;
    }

    if opt.version.len() == 0 {
        let last_date_exists = utils::check_date_exists(&opt.repo_path, &current_date);
        debug!("Last date exists: {:?}", last_date_exists);

        if last_date_exists == true {
            writeln!(
                handle,
                "File already has an entry for date {}.",
                &current_date
            )
            .unwrap();
            return;
        }
    }
    let last_entry = utils::get_last_entry(&opt.repo_path);
    debug!("Last entry: {:?}", last_entry);

    let parsed_select: Vec<&str> = last_entry.split(",").collect();
    debug!("Parse Select split: {:?}", parsed_select);

    for (language_pos, language) in languages_vec.iter().enumerate() {
        debug!("Language being checked is {:?}", language[0]);
        let mut json_language_counts = r#""#.to_owned();
        json_language_counts.push_str(r#"""#);
        json_language_counts.push_str(language[0]);
        json_language_counts.push_str(r#"":"#);
        json_language_counts.push_str(r#"{"#);
        for (folder_pos, folder) in folder_split.iter().enumerate() {
            debug!("Iterating folder {:?}", folder);
            let mut folder_language_counts = r#""#.to_owned();
            folder_language_counts.push_str(r#"""#);
            folder_language_counts.push_str(folder);
            folder_language_counts.push_str(r#"":"#);
            let mut ts_argument = "rg --files --type-add '".to_owned();
            ts_argument.push_str(language[0]);
            ts_argument.push_str(":*{");
            ts_argument.push_str(language[1]);
            ts_argument.push_str("}' -t");
            ts_argument.push_str(&language[0]);
            ts_argument.push_str(" ");
            ts_argument.push_str(&opt.repo_path);
            ts_argument.push_str("/");
            ts_argument.push_str(&folder);
            ts_argument.push_str(" | wc -l");
            debug!("Argument for folder {:?} is {:?}", folder, ts_argument);
            let current = Command::new("sh")
                .arg("-c")
                .arg(&ts_argument)
                .output()
                .expect("Failed to execute command");

            let count_string = std::str::from_utf8(&current.stdout)
                .unwrap()
                .trim_start()
                .trim_end();
            debug!(
                "Count for language {:?} in folder {:?} is {:?}",
                language[0], folder, count_string
            );
            folder_language_counts.push_str(count_string);
            if folder_pos != folder_split.len() - 1 {
                folder_language_counts.push_str(",");
            } else {
                folder_language_counts.push_str("}");
            }
            json_language_counts.push_str(&folder_language_counts);
            debug!(
                "JSON for {:?} in {:?}: {:?}",
                language[0], folder, folder_language_counts
            );
        }
        if language_pos != languages_vec.len() - 1 {
            json_language_counts.push_str(",");
        }
        json_counts.push_str(&json_language_counts);
        debug!("JSON value is {:?}", json_language_counts);
    }
    json_counts.push_str(r#"}"#);

    debug!("JSON string is {:?}", json_counts);
    let json_value: Value = serde_json::from_str(&json_counts).unwrap();
    debug!(" JSON value is {:?}", json_value);

    // let mut select_statement: String =
    // r#"sqlite3 /Users/adalbertoteixeira/Documents/ben/management/management.db "SELECT date,counts FROM language_evolution"#.to_owned();
    // select_statement.push_str(r#" WHERE branch = '"#);
    // select_statement.push_str(&branch);
    // select_statement.push_str(r#"' ORDER BY date DESC LIMIT 1;""#);
    // debug!("Select statement is {:?}", &select_statement);
    // let select_sqlite = Command::new("sh")
    //     .arg("-c")
    //     .arg(select_statement)
    //     .output()
    //     .expect("Failed to execute command");
    // debug!("Select statement result is {:?}", select_sqlite.stdout);
    // let parsed_select: Vec<&str> = std::str::from_utf8(&select_sqlite.stdout)
    //     .unwrap()
    //     .trim_end_matches("\n")
    //     .split("|")
    //     .collect();
    // debug!("Parsed select {:?}", parsed_select);

    // let mut previous_counts: Value;
    // if parsed_select.len() > 1 {
    //     previous_counts = serde_json::from_str(&parsed_select[1]).unwrap();
    // } else {
    //     previous_counts = serde_json::from_str("{}").unwrap();
    // }
    // debug!("JSON select counts {:?}", previous_counts);
    let mut folder_header = "Language Count,,".to_owned();
    let mut language_header = ",,".to_owned();
    let mut previous_row = "".to_owned();
    previous_row.push_str(parsed_select.get(0).unwrap());
    previous_row.push_str(",");
    previous_row.push_str(parsed_select.get(1).unwrap());
    // let previous_date = if parsed_select[0].len() > 1 {
    // parsed_select[0]
    // } else {
    // ","
    // };
    previous_row.push_str(",");

    let mut current_row = "".to_owned();
    current_row.push_str(&current_version);
    current_row.push_str(",");
    current_row.push_str(current_date);
    current_row.push_str(",");

    let mut diff_row = ",,".to_owned();
    let mut diff_percent_row = ",,".to_owned();

    // debug!("\nCheck CSV position {}, {}\n", folder_pos, language_pos);
    let empty_json = serde_json::from_str("{}").unwrap();
    let empty_value = json!(0);
    for (folder_pos, folder) in folder_split.iter().enumerate() {
        let mut folder_total = 0;
        folder_header.push_str(folder);
        for language in languages_vec.iter() {
            let cell_value = json_value
                .get(language[0])
                .unwrap_or(&empty_json)
                .get(folder)
                .unwrap_or(&empty_value);
            let current_integer: i64 = cell_value.as_i64().unwrap();
            folder_total = folder_total + current_integer;
        }
        for (language_pos, language) in languages_vec.iter().enumerate() {
            language_header.push_str(language[0]);
            let cell_value = json_value
                .get(language[0])
                .unwrap_or(&empty_json)
                .get(folder)
                .unwrap_or(&empty_value);
            current_row.push_str(&cell_value.to_string());
            debug!(
                "\n\n\nPrevious cell: {}, {}, {}, {:?}",
                folder_pos, language_pos, folder, language,
            );
            let mut csv_position = 2;
            if folder_pos != 0 {
                csv_position = csv_position + (folder_pos * 4);
            }

            if language_pos > 0 {
                csv_position = csv_position + (language_pos * 2);
            }
            debug!("csv position: {}", csv_position);
            let previous_cell_value = parsed_select.get(csv_position).unwrap();
            debug!("previous_cell_value: {:?}", previous_cell_value);
            //             let previous_cell_value = previous_counts
            //                 .get(language[0])
            //                 .unwrap_or(&empty_json)
            //                 .get(folder)
            //                 .unwrap_or(&empty_value);
            previous_row.push_str(&previous_cell_value.to_string());
            let previous_integer: i64 = previous_cell_value.parse().unwrap();
            let current_integer: i64 = cell_value.as_i64().unwrap();
            let language_percent = (current_integer as f64 / folder_total as f64) * 100_f64;
            current_row.push_str(",");
            current_row.push_str(&format!("{:.2}%", &language_percent));

            debug!(
                "Total for {:?} and {:?} is {} / {} = {}\n",
                folder, language[0], &current_integer, &folder_total, language_percent
            );
            debug!(
                "To calculate: ({} - {}) / {}",
                current_integer, previous_integer, previous_integer
            );
            let diff_integer = if previous_integer != 0 {
                current_integer - previous_integer
            } else {
                0
            };
            let diff_string = diff_integer.to_string();
            diff_row.push_str(&diff_string);

            let diff_percent: f64 = if diff_integer != 0 {
                (diff_integer as f64 / previous_integer as f64) * 100_f64
            } else {
                100 as f64
            };
            // let diff_percent_string = diff_percent.to_string();
            diff_percent_row.push_str(&format!("{:.2}%", &diff_percent));
            let is_last =
                language_pos == languages_vec.len() - 1 && folder_pos == folder_split.len() - 1;
            debug!(
                "boolean: {}
            language_pos: {}, languages_vec.len(): {}, folder_pos: {}, folder_split.len(): {}
                            ",
                is_last,
                language_pos,
                languages_vec.len(),
                folder_pos,
                folder_split.len(),
            );

            if is_last != true {
                language_header.push_str(",");
                folder_header.push_str(",");
                previous_row.push_str(",");
                current_row.push_str(",");
                diff_row.push_str(",");
                diff_percent_row.push_str(",");
            }
            language_header.push_str(",");
            folder_header.push_str(",");
            previous_row.push_str(",");
            diff_row.push_str(",");
            diff_percent_row.push_str(",");
        }
    }
    writeln!(
        handle,
        "{}\n{}\n{}\n{}\n{}\n{}",
        folder_header, language_header, previous_row, current_row, diff_row, diff_percent_row
    )
    .unwrap();

    utils::remove_last_lines(&opt.repo_path, opt.dry_run);
    if opt.dry_run != true {
        let mut add_counts = r#"echo '"#.to_owned();
        add_counts.push_str(&current_row);
        add_counts.push_str(r#"' >> "#);
        add_counts.push_str(&opt.repo_path);
        add_counts.push_str(r#"/TYPESCRIPT_EVOLUTION.csv"#);
        debug!("Last line removal command is {:?}", &add_counts);
        if opt.dry_run != true {
            let add_counts_result = Command::new("sh")
                .arg("-c")
                .arg(&add_counts)
                .output()
                .unwrap();
            debug!("Last line removal result is {:?}", &add_counts_result);
        }

        let mut add_integer_differences = r#"echo '"#.to_owned();
        add_integer_differences.push_str(&diff_row);
        add_integer_differences.push_str(r#"' >> "#);
        add_integer_differences.push_str(&opt.repo_path);
        add_integer_differences.push_str(r#"/TYPESCRIPT_EVOLUTION.csv"#);
        debug!(
            "Last line removal command is {:?}",
            &add_integer_differences
        );
        let add_integer_differences_result = Command::new("sh")
            .arg("-c")
            .arg(&add_integer_differences)
            .output()
            .unwrap();
        debug!(
            "Last line removal result is {:?}",
            &add_integer_differences_result
        );

        let mut add_percent_differences = r#"echo '"#.to_owned();
        add_percent_differences.push_str(&diff_percent_row);
        add_percent_differences.push_str(r#"' >> "#);
        add_percent_differences.push_str(&opt.repo_path);
        add_percent_differences.push_str(r#"/TYPESCRIPT_EVOLUTION.csv"#);
        debug!(
            "Last line removal command is {:?}",
            &add_percent_differences
        );
        let add_percent_differences_result = Command::new("sh")
            .arg("-c")
            .arg(&add_percent_differences)
            .output()
            .unwrap();
        debug!(
            "Last line removal result is {:?}",
            &add_percent_differences_result
        );
    }
}
