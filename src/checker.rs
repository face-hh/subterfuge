use std::{fs::{self, create_dir_all}, process::Command};

use crate::structs::{BattlepassTier, Problem, SaveFile};

pub fn index_to_problem(save_file: &SaveFile, current_problem: i64) -> Option<Problem> {
    save_file
        .problems
        .iter()
        .find(|&el| el.index == current_problem)
        .cloned()
}

pub fn index_to_battlepass_tier(save_file: &SaveFile, index: i64) -> Option<BattlepassTier> {
    save_file
        .battlepass
        .iter()
        .find(|&el| el.index.parse::<i64>().unwrap() == index)
        .cloned()
}

pub fn check(mut contents: String, current_problem: &Problem) -> bool {
    let file_path = "dump/mutated.ts";
    let help_funcs = r"/** INJECTED BY SUBTERFUGE */
    
    function __arraysEqual(arr1, arr2) {
        return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
    }
    function __objectEqual(obj1, obj2) {
        return JSON.stringify(obj1) === JSON.stringify(obj2)
    }
    ";
    //                                                          ...... here ......
    contents.push_str(&format!("\n\n\n{}\n\n\n{}", help_funcs, current_problem.append));

    let file = fs::write(file_path, contents);

    match file {
        Err(_err) => {
            let dump_err = create_dir_all("dump");
            if dump_err.is_err() {
                println!("An error occurred while writing the file, and another error occurred while creating the dump directory. Please check your permissions and try again.");
            }
        }
        Ok(_) => {},
    }
    
    let output = Command::new("bun")
        .args([file_path])
        .output()
        .expect("failed to execute process");

    let res = String::from_utf8_lossy(&output.stdout).to_string();

    if current_problem.name == "Hello World" {
        res.contains("Hello, World!") || res.to_lowercase().contains("Hello World")
    } else {
        if res.contains("__PASS55__") {
            return true;
        }

        false
    }
}
