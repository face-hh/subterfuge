use crossterm::style::Stylize;

use crate::structs::Feature;
use crate::structs::SaveFile;

use std::io::{stdin, stdout, Write};
use std::process;

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

pub fn display_shop(save_file: &SaveFile) -> &Feature {
    let money = save_file.money.to_string() + "$";

    println!("{}", format!("{} {}", "! Wallet:".dark_grey(), money.dark_green()));

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
            if save_file.money >= feature.cost { cost.clone().green() } else { cost.clone().red() }
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

    return feature
}