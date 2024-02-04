use crossterm::style::Stylize;
use rand::Rng;

use crate::checker::index_to_problem;
use crate::structs::Feature;
use crate::structs::SaveFile;

use std::io::{stdin, stdout, Write};
use std::process;
use std::thread;
use std::time;

pub fn display_errors(errors: Vec<String>) {
    println!(
        "{}",
        "𐂃  Hey there, seems like you're using locked features :)".cyan()
    );
    println!(
        "{}",
        "|********************************************************|".dark_grey()
    );
    println!(
        "{}",
        "|                       Error dump                       |".dark_grey()
    );
    println!(
        "{}",
        "|                                                        |".dark_grey()
    );

    for error in errors.iter().rev() {
        println!("{}", error);
    }

    println!(
        "{}",
        "|________________________________________________________|".dark_grey()
    );
}

pub fn display_current_task(save_file: &SaveFile) {
    let problem = index_to_problem(save_file, save_file.current_problem).unwrap_or_else(|| {
        println!("There are no problems at index {}. Maybe you completed all of them?", save_file.current_problem);
        process::exit(1)
    });
    println!(
        "{}",
        "|********************************************************|".dark_grey()
    );
    println!("{} {}", "| Problem:".dark_grey(), problem.name.cyan());
    println!(
        "{} {}",
        "| Description:".dark_grey(),
        problem.description.cyan()
    );
    println!(
        "{} {}{}",
        "| Bounty:".dark_grey(),
        problem.money.to_string().cyan(),
        "$".cyan()
    );
    println!(
        "{} {}",
        "| #:".dark_grey(),
        problem.index.to_string().cyan(),
    );
    println!(
        "{} {}",
        "| Boilerplate:".dark_grey(),
        problem.starting_code.replace("\n", "").cyan(), // it breaks things lol
    );
    println!(
        "{}",
        "|________________________________________________________|".dark_grey()
    );
}

fn _format_index(index: &String) -> String {
    if index.len() == 1 {
        "0".to_owned() + &index
    } else {
        index.to_string()
    }
}

fn create_box(content: &str, max_chars: usize) -> String {
    let content_len = content.len();

    if content_len >= max_chars {
        let trimmed_content = &content[..max_chars];
        trimmed_content.to_owned()
    } else {
        let padding = (max_chars - content_len) / 2;
        let mut padded_content = format!("{:^width$}", content, width = content_len + padding * 2);

        if content_len == 3 {
            padded_content.pop(); // weird ass bug, dont ask, chatgpt code 👍
        }
        if content.contains("$") {
            padded_content += " "
        }
        padded_content
    }
}

fn print_cell(title: String, reward: String, p: String, premium: bool) {
    println!("{title}");
    println!("{reward}");
    
    if premium { println!("{p}") }
}

pub fn display_battlepass(save_file: &SaveFile) {
    println!("┏━  {}  ━┓", "BATTLE PASS".yellow());

    let mut i = 1;
    for tier in save_file.battlepass.iter() {
        i += 1;

        let index = &tier.index;

        let mut end = "┗━━━━━━━━━━━━━━━━━┛".dark_grey();
        let mut semiend = "┃━━━━━━━━━━━━━━━━━┃".dark_grey();
        let mut in_between = format!(
            "┣#{}━━━━━━━━━━━━━━┫",
            _format_index(&(i - 1).to_string())
        ).dark_grey();
        let _title = create_box(&tier._type, 15);
        let _reward = create_box(&(tier.amount.to_string() + "$"), 15);
        let _p = create_box(&"PREMIUM", 15);

        let mut bar = "┃".dark_grey();
        let mut modify_title = false;
        let mut modify_title_completed = false;

        if save_file.bp_tier > i - 2 {
            bar = "┃".green();
            in_between = in_between.green();
            end = end.green();
            semiend = semiend.green();
        
            modify_title_completed = true;
        }

        if save_file.bp_tier == i - 2 {
            bar = "┃".yellow();
            in_between = in_between.yellow();
            end = end.yellow();
            semiend = semiend.yellow();

            modify_title = true;
        }

        let title = format!("{bar} {} {bar}", _title.on_black().cyan());
        let mut reward = format!("{bar} {} {bar}", _reward.on_black().cyan());
        let p = format!("{bar} {} {bar}", _p.on_black().red());

        if modify_title {
            reward += &format!("   ◄ {}/200 XP", save_file.bp_xp).blue().to_string()
        }
        if modify_title_completed {
            reward += &"   🮤 200/200 XP".dark_blue().to_string()
        }

        if index == "1" {
            println!(
                "┃━━━━━━━━━━━━━━━━━┃\n┣#{}━━━━━━━━━━━━━━┫",
                _format_index(&(i - 1).to_string()) // fuck this
            );
            print_cell(title, reward, p, tier.p);
        } else if index == "30" {
            println!("{in_between}");
            print_cell(title, reward, p, tier.p);
            println!("{end}")
        } else {
            println!("{in_between}");
            print_cell(title, reward, p, tier.p);
            println!("{semiend}");
        }
    }
}

pub fn peek(save_file: &SaveFile) {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    
    let bar = "┃";

    let _intro = create_box(&"Problem set:", 35);

    println!("{bar} {} {bar}", _intro.cyan());

    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    for problem in save_file.problems.iter() {
        let mut additional = "".white();

        if problem.index == save_file.current_problem {
            additional = "🮤 You're here!".dark_grey()
        }

        println!("┣ {}. {} {}", problem.index, problem.name.clone().cyan(), additional);
    }

    if save_file.current_problem > save_file.problems.len() as i64 {
        println!("┗ {}", "Congrats, you've completed the game! Was a blast having you on :D".yellow())
    }
}

pub fn display_shop(save_file: &SaveFile) -> &Feature {
    let money = save_file.money.to_string() + "$";

    println!("{} {}", "! Wallet:".dark_grey(), money.dark_green());

    let features = &save_file.features;
    let mut listed_features: Vec<&Feature> = vec![];

    let mut i = 0;
    for feature in features {
        if feature.unlocked {
            continue;
        }
        i += 1;
        let item = &feature.item;
        let cost = feature.cost.to_string() + "$";

        // fuck you and your performance.clone().unwrap().unwrap().unwrap().unwrap().clone()
        let info = format!(
            "{}. {} - {}",
            i.to_string().dark_grey(),
            item.clone().cyan(),
            if save_file.money >= feature.cost {
                cost.clone().green()
            } else {
                cost.clone().red()
            }
        );

        listed_features.push(feature);

        println!("{info}");
    }

    print!("I want to buy... > ");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let index: usize = s.parse().unwrap_or_else(|_| {
        println!("Received input failed to be cast to usize.");
        process::exit(1)
    });
    let feature = listed_features.get(index - 1).unwrap_or_else(|| {
        println!("Received input exceeds array boundaries.");
        process::exit(1)
    });

    feature
}

pub fn display_spinning_wheel() {
    let steps = vec![1, 5, 5, 5, 5, 5, 5, 5, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 120, 150];

    for step in steps {
        let _1 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _2 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _3 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _4 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _5 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _6 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();
        let _7 = (rand::thread_rng().gen_range(10..100).to_string() + "%").yellow();

        let __1 = (rand::thread_rng().gen_range(1..10).to_string() + "%").yellow();
        let __2 = (rand::thread_rng().gen_range(1..10).to_string() + "%").yellow();
        let __3 = (rand::thread_rng().gen_range(1..10).to_string() + "%").yellow();
        let __4 = (rand::thread_rng().gen_range(1..10).to_string() + "%").yellow();
        let __5 = (rand::thread_rng().gen_range(1..10).to_string() + "%").yellow();

        print!("\x1B[2J\x1B[1;1H");

        println!("{}", "It's your lucky chance!".yellow());

        println!(
            r"           __
       . ' || ' .
    .` {} || {} `.
  .   \    ||    /   .
'/ _ {}\ .-''-. /{}_ \
J {}`- .' .--. '. -`{} L
F======' ((<>)) '======J
L  {} '. `||' .' {}  F
 \  _.-  `-||-'  -._  /
  .  {}/  ||  \ {} .
    .  / {}|| {}\  .
      ` . _||_ . `",
            _1, _2, __1, _3, __2, __3, _4, _5, _6, _7, __4, __5
        );

        thread::sleep(time::Duration::from_millis(step * 10));
    }

    println!("\n{} {} {}", "Congratulations!".green(), "[Click here]".yellow(), "to claim your COUPON!".green())
}