mod checker;
pub mod structs;
mod tui;

use std::fs;
use std::{env, process};

use crossterm::style::Stylize;
use fancy_regex::Regex;
use rand::Rng;
use serde_json::{Result, Value};

use structs::{BattlepassTier, SaveFile};
use structs::{Feature, Problem};

fn read_save_data() -> SaveFile {
    let file_path = "src/data/savedata.json";

    let json_string = std::fs::read_to_string(file_path).unwrap();

    let json: Value = serde_json::from_str(&json_string).unwrap();

    let features_raw = json.get("features").unwrap().as_array().unwrap();
    let money = json.get("money").unwrap().as_i64().unwrap();
    let bp_xp = json.get("bp_xp").unwrap().as_i64().unwrap();
    let bp_tier = json.get("bp_tier").unwrap().as_i64().unwrap();
    let premium = json.get("premium").unwrap().as_bool().unwrap();

    let current_problem = json.get("current_problem").unwrap().as_i64().unwrap();

    // the code you are about to read is very disgusting
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
            let starting_code = value
                .get("starting_code")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let description = value
                .get("description")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let money = value.get("money").unwrap().as_i64().unwrap();
            let index = value.get("index").unwrap().as_i64().unwrap();

            Problem {
                name,
                money,
                index,
                append,
                starting_code,
                description,
            }
        })
        .collect();

    let raw_battlepass = json.get("battlepass").unwrap().as_array().unwrap();

    let battlepass: Vec<BattlepassTier> = raw_battlepass
        .iter()
        .map(|value| {
            let _type = value.get("type").unwrap().as_str().unwrap().to_string();
            let index = value.get("index").unwrap().as_str().unwrap().to_string();
            let amount = value.get("amount").unwrap().as_i64().unwrap();
            let p = value.get("p").unwrap().as_bool().unwrap();

            BattlepassTier {
                index,
                _type,
                amount,
                p,
            }
        })
        .collect();
    SaveFile {
        features,
        money,
        problems,
        current_problem,
        battlepass,
        bp_xp,
        bp_tier,
        premium,
    }
}

fn run_checks(save_file: SaveFile, file_path: &String) {
    let wheel_chance = rand::thread_rng().gen_range(1..10);

    if wheel_chance == 1 {
        tui::display_spinning_wheel()
    }

    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut errors: Vec<String> = Vec::new();

    for feature in save_file.features.iter() {
        if feature.unlocked {
            continue;
        }

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
                .unwrap_or_else(|| {
                    (0, "Could not retrieve snippet due to new lines.")
                });

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

    if !errors.is_empty() {
        return tui::display_errors(errors);
    }

    // check for the code to run successfully
    let problem = checker::index_to_problem(&save_file, save_file.current_problem).unwrap();

    println!(
        "Running your code against Problem #{}: \"{}\"...",
        &problem.index,
        &problem.clone().name.cyan()
    );

    let money = problem.money;
    let passed = checker::check(contents, &problem);

    if !passed {
        return println!("{}", "𐂃  Your code did not pass... :/".red());
    }

    problem_passed(&save_file, &problem);

    if let Some(next_problem) = checker::index_to_problem(&save_file, save_file.current_problem + 1)
    {
        move_file(next_problem, file_path)
    }

    println!(
        "{} (+{}$)",
        "𐂃  You did it! Run \"current\" to see your next challenge.".green(),
        money
    )
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

fn problem_passed(save_file: &SaveFile, problem: &Problem) {
    let file_path = "src/data/savedata.json";
    let json_string = std::fs::read_to_string(file_path).unwrap();

    let mut json: Value = serde_json::from_str(&json_string).unwrap();
    let mut money = json.get("money").unwrap().as_i64().unwrap();
    let mut bp_xp = json.get("bp_xp").unwrap().as_i64().unwrap();
    let mut bp_tier = json.get("bp_tier").unwrap().as_i64().unwrap();
    let mut current_problem = json.get("current_problem").unwrap().as_i64().unwrap();

    money += problem.money;
    current_problem += 1;
    bp_xp += (problem.money as f64 / 1.5).round() as i64;

    while bp_xp >= 200 && bp_tier != 31 {
        bp_tier += 1;
        if let Some(tier) = checker::index_to_battlepass_tier(&save_file, bp_tier) {
            // UNTESTED
            if (tier.p && save_file.premium) || !tier.p {
                money += tier.amount;
            }
    
            json["bp_tier"] = Value::from(bp_tier);
            bp_xp -= 200;
        } else {
            continue;
        }
    }    

    json["current_problem"] = Value::from(current_problem);
    json["money"] = Value::from(money);
    json["bp_xp"] = Value::from(bp_xp);

    let updated_json_string = serde_json::to_string_pretty(&json).unwrap();

    fs::write(file_path, updated_json_string).unwrap();
}

fn update_field(field: &str, value: bool) {
    let file_path = "src/data/savedata.json";
    let json_string = std::fs::read_to_string(file_path).unwrap();

    let mut json: Value = serde_json::from_str(&json_string).unwrap();

    json[field] = Value::from(value);

    let updated_json_string = serde_json::to_string_pretty(&json).unwrap();

    fs::write(file_path, updated_json_string).unwrap();
}
fn move_file(problem: Problem, file_path: &String) {
    let _ = fs::copy(file_path, "history/".to_owned() + &rand::thread_rng().gen_range(10..10000).to_string() + file_path);

    let _ = fs::write(
        file_path,
        format!("/** {} */\n{}", problem.description, problem.starting_code),
    );
}

fn main() -> Result<()> {
    let save_file = read_save_data();

    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap_or_else(|| {
        println!(
            "{}\n\n{}{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            format!(
                "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                   {}                      ┃
┃      You've received a {} for {} off!        ┃
┃                {}                  ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛", // fuck you VSCode
                "ATTENTION!".red(),
                "COUPON".cyan(),
                (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow(),
                "[Click to reveal]".on_green().black()
            ),
            "𐂃  Subterfuge\n".cyan(),
            "Learning TypeScript the Temu way!\n".dark_cyan().italic(),
            "This program's purpose is to restrict your TypeScript down to just console.log and functions. After completing \"problems\" (similar to LeetCode), you gain money. With money, you buy features. To complete the game, you need to solve all (10) problems. Commands are listed below for help.\n".dark_grey().italic(),
            "Commands:",
            "  - [filepath]     (Runs the given file against current problem. Example: main.ts)",
            "  - shop           (Displays the shop with the available features to purchase)",
            "  - current        (Displays information about your current problem)",
            "  - peek           (Displays the problem list & where you're at)",
            "  - battlepass     (Displays the battlepass)",
            "  - claim [code]   (Claim a promo code for Premium. Example: claim AOPMGBAEP)",
            "  - support [code] (Support a content creator that creates content on this game)",
            "  - use [code]     (Claim a COUPON code. Example: claim OKEGAOP)",
        );
        process::exit(1);
    });
    let arg2 = args.get(2);

    if arg == "shop" {
        let feature = tui::display_shop(&save_file);

        buy_feature(feature)
    } else if arg == "current" {
        tui::display_current_task(&save_file)
    } else if arg == "battlepass" {
        tui::display_battlepass(&save_file)
    } else if arg == "claim" {
        if arg2.unwrap_or_else(|| {
            println!("Please provide the promo code.");
            process::exit(1)
        }) == "KJGQ77"
        {
            update_field("premium", true)
        }
    } else if arg == "use" {
        // haha
    } else if arg == "peek" {
        tui::peek(&save_file)
    } else if arg == "support" {
    let arg = arg2.unwrap_or_else(|| {
        println!("Please provide the content creator.");
        process::exit(1)
    }).clone().yellow();
    
    println!("You are now supporting: {}.", arg)
} else {
        run_checks(save_file, arg);
    }

    Ok(())
}
