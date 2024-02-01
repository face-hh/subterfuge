pub mod structs;
mod checker;
mod tui;

use std::fs;
use std::{env, process};

use crossterm::style::Stylize;
use fancy_regex::Regex;
use serde_json::{Result, Value};

use structs::{Feature, Problem};
use structs::SaveFile;

fn read_save_data() -> SaveFile {
    let file_path = "src/data/savedata.json";

    let json_string = std::fs::read_to_string(file_path).unwrap();

    let json: Value = serde_json::from_str(&json_string).unwrap();

    let features_raw = json.get("features").unwrap().as_array().unwrap();
    let money = json.get("money").unwrap().as_i64().unwrap();
    let current_problem = json.get("current_problem").unwrap().as_i64().unwrap();

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

    let problems_raw = json.get("problems").unwrap().as_array().unwrap();

    let problems: Vec<Problem> = problems_raw
        .iter()
        .map(|value| {
            let name = value.get("name").unwrap().as_str().unwrap().to_string();
            let append = value.get("append").unwrap().as_str().unwrap().to_string();
            let starting_code = value.get("starting_code").unwrap().as_str().unwrap().to_string();
            let description = value.get("description").unwrap().as_str().unwrap().to_string();
            let money = value.get("money").unwrap().as_i64().unwrap();
            let index = value.get("index").unwrap().as_i64().unwrap();

            Problem {
                name,
                money,
                index,
                append,
                starting_code,
                description
            }
        })
        .collect();
    SaveFile { features, money, problems, current_problem }
}

fn run_checks(save_file: SaveFile, file_path: &String) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut errors: Vec<String> = Vec::new();

    for feature in save_file.features.iter() {
        if feature.unlocked { continue }

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

    if errors.len() != 0 { return tui::display_errors(errors); }

    // check for the code to run successfully
    let problem = checker::index_to_problem(&save_file, save_file.current_problem);
    let next_problem = checker::index_to_problem(&save_file, save_file.current_problem + 1);

    println!("{}", format!("Running your code against Problem #{}: \"{}\"...", &problem.index, &problem.clone().name.cyan()));

    let money = problem.money;
    let passed = checker::check(contents, &problem);

    if !passed {
        return println!("{}", format!("{}", "𐂃  Your code did not pass... :/".red()))
    }

    problem_passed(&problem);
    move_file(next_problem, file_path);
    println!("{}", format!("{} (+{}$)", "𐂃  You did it! Run \"sbf current\" to see your next challenge.".green(), money))
}

fn buy_feature(feature: &Feature) {
    let file_path = "src/data/savedata.json";
    let json_string = std::fs::read_to_string(file_path).unwrap();

    let mut json: Value = serde_json::from_str(&json_string).unwrap();
    let mut money = json.get("money").unwrap().as_i64().unwrap();
    let features_raw = json.get_mut("features").unwrap().as_array_mut().unwrap();

    if money < feature.cost {
        return println!("{}", "𐂃  You can't afford this :(".red());
    }

    if let Some(found_feature) = features_raw
        .iter_mut()
        .find(|el| el.get("item").unwrap().as_str() == Some(feature.item.as_str()))
    {
        found_feature["unlocked"] = Value::Bool(true);
    } else {
        return println!("{}", "Feature not found :(".red());
    }

    money -= feature.cost;
    json["money"] = Value::from(money);

    let updated_json_string = serde_json::to_string_pretty(&json).unwrap();

    fs::write(file_path, updated_json_string).unwrap();
}

fn problem_passed(problem: &Problem) {
    let file_path = "src/data/savedata.json";
    let json_string = std::fs::read_to_string(file_path).unwrap();

    let mut json: Value = serde_json::from_str(&json_string).unwrap();
    let mut money = json.get("money").unwrap().as_i64().unwrap();
    let mut current_problem = json.get("current_problem").unwrap().as_i64().unwrap();

    money += problem.money;
    current_problem += 1;

    json["current_problem"] = Value::from(current_problem);
    json["money"] = Value::from(money);

    let updated_json_string = serde_json::to_string_pretty(&json).unwrap();

    fs::write(file_path, updated_json_string).unwrap();
}

fn move_file(problem: Problem, file_path: &String) {
    let _ = fs::copy(file_path, ".history/".to_owned() + file_path);
    let _ = fs::write(file_path, format!("/** {} */\n{}", problem.description, problem.starting_code));
}

fn main() -> Result<()> {
    let save_file = read_save_data();

    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap_or_else(|| {
        println!(
            "{}",
            format!(
                "{}\n{}\n{}\n{}",
                "𐂃  Subterfuge\n".cyan(),
                "Commands:",
                "  - filepath (ex. main.ts)",
                "  - shop",
            )
        );
        process::exit(1);
    });

    if arg == "shop" {
        let feature = tui::display_shop(&save_file);

        buy_feature(feature)
    } else {
        run_checks(save_file, arg);
    }

    Ok(())
}
