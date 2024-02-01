use std::{fs, process::Command};

use crate::structs::{Problem, SaveFile};

pub fn index_to_problem(save_file: &SaveFile, current_problem: i64) -> Option<Problem> {
    println!("{}", current_problem);

    save_file
        .problems
        .iter()
        .find(|&el| el.index == current_problem)
        .cloned()
}

pub fn check(mut contents: String, current_problem: &Problem) -> bool {
    let file_path = ".dump/mutated.ts";
    let help_funcs = r"function __arraysEqual(arr1, arr2) {
        return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
    }";

    contents.push_str(&format!("\n\n\n{}\n\n\n{}", help_funcs, current_problem.append));

    let _ = fs::write(file_path, contents);

    let output = Command::new("bun")
        .args([file_path])
        .output()
        .expect("failed to execute process");

    let res = String::from_utf8_lossy(&output.stdout).to_string();

    if current_problem.name == "Hello World" {
        res.contains("Hello, World!")
    } else {
        if res.contains("__PASS55__") {
            return true;
        }

        false
    }
}
