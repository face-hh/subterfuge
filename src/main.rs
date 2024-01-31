mod tui;

use std::{env, process};

use colored::Colorize;
use fancy_regex::Regex;
use serde_json::{Result, Value};

#[derive(Debug)]
struct SaveFile {
    money: i64,
    features: Vec<Feature>,
}

#[derive(Debug)]
struct Feature {
    item: String,
    cost: i64,
    unlocked: bool,
    regex: String,
}

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

            let err1 = format!("|- {}. {}", (line_number + 1), line.replace(snippet, &snippet.red().to_string()).yellow()).white();
            let err2 = format!("|___  Usage of \"{}\" is forbidden.", feature.item.red());

            errors.push(format!("{}\n{}", err1, err2));
        }
    }

    tui::display_errors(errors);
}

fn main() -> Result<()> {
    let _ = tui::init();
    
    let save_file = read_save_data();

    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please specify the file name to use. (cargo run main.ts)");

    run_checks(save_file, file_path);

    Ok(())
}
