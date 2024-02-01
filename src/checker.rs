use std::{fs, process::Command};

use crate::structs::{Problem, SaveFile};

pub fn index_to_problem(save_file: &SaveFile, current_problem: i64) -> Problem {
    let problem = save_file
        .problems
        .iter()
        .find(|&el| el.index == current_problem)
        .cloned()
        .expect("Failed to find problem by current_problem. Boundaries exceeded?");

    problem
}

pub fn check(mut contents: String, current_problem: &Problem) -> bool {
    let file_path = ".dump/mutated.ts";

    contents.push_str(&format!("\n\n\n{}", current_problem.append));

    let _ = fs::write(file_path, contents);

    let output = Command::new("bun")
        .args([file_path])
        .output()
        .expect("failed to execute process");

    let res = String::from_utf8_lossy(&output.stdout).to_string();

    if current_problem.name == "Hello World" {
        hello_world(res)
    } else if current_problem.name == "Fibonacci" {
        false
    } else {
        false
    }
}

fn hello_world(stdout: String) -> bool {
    stdout.contains("Hello, World!")
}
