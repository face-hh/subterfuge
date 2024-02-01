pub mod structs;
mod tui;

use std::{env, process};

use crossterm::style::Stylize;
use fancy_regex::Regex;
use serde_json::{Result, Value};

use structs::Feature;
use structs::SaveFile;

fn read_save_data() -> SaveFile {
    let file_path = "src/data/savedata.json";

    let json_string = std::fs::read_to_string(file_path).unwrap();

    let json: Value = serde_json::from_str(&json_string).unwrap();

    let features_raw = json.get("features").unwrap().as_array().unwrap();
    let money = json.get("money").unwrap().as_i64().unwrap();

    let features: Vec<Feature> = features_raw
        .iter()
        .map(|value| {
            let item = value.get("item").unwrap().as_str().unwrap().to_string();
            let cost = value.get("cost").unwrap().as_i64().unwrap();
            let unlocked = value.get("unlocked").unwrap().as_bool().unwrap();
            let regex = value.get("regex").unwrap().as_str().unwrap().to_string();

            Feature {
                item,
                cost,
                unlocked,
                regex,
            }
        })
        .collect();

    SaveFile { features, money }
}

fn run_checks(save_file: SaveFile, file_path: &String) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut errors: Vec<String> = Vec::new();

    for feature in save_file.features.iter() {
        let re = Regex::new(&feature.regex).unwrap();
        let res = re
            .find(&contents)
            .expect("Failed to perform find on content.");
        let is_matched = res.is_some();

        if is_matched {
            let start = res.unwrap().start();
            let end = res.unwrap().end();
            let snippet = &contents[start..end];

            let (line_number, line) = contents
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains(snippet))
                .unwrap();

            let err1 = format!(
                "|- {}. {}",
                (line_number + 1),
                line.replace(snippet, &snippet.red().to_string()).yellow()
            )
            .white();
            let err2 = format!(
                "|___  Usage of \"{}\" is forbidden.",
                feature.item.clone().red()
            );

            errors.push(format!("{}\n{}", err1, err2));
        }
    }

    tui::display_errors(errors);
}

fn main() -> Result<()> {
    let save_file = read_save_data();

    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap_or_else(|| {
        println!(
            "{}",
            format!(
                "{}\n{}\n{}\n{}",
                "| Please specify a second argument.".red(),
                "|_ Example:".dark_grey(),
                "|___ sbf main.ts".dark_grey(),
                "|___ sbf shop".dark_grey()
            )
        );
        process::exit(1);
    });

    let needs_to_run = tui::init(&arg, &save_file);

    if needs_to_run {
        run_checks(save_file, arg);
    }

    Ok(())
}
