#[macro_use]
extern crate clap;

mod problem;
mod file_utils;
mod sample_cases;

use clap::{App, Arg, SubCommand};
use std::io;
use problem::Problem;


fn execute_fetching_problem(contest_id: &str, problem_id: &str) -> Result<(), io::Error> {
    let problem = Problem::new(contest_id, &problem_id);
    if let Some(problem) = problem {
        problem.create_sample_cases_files()?;
    }
    Ok(())
}

fn execute_fetching_problems_in_contest(contest_id: &str) -> Result<(), io::Error> {
    let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
    for alphabet in alphabets {
        let problem_id = format!("{}", alphabet);
        execute_fetching_problem(contest_id, &problem_id)?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("gen")
                .about("Generate input/output format example fetched from AtCoder's website.")
                .arg(
                    Arg::with_name("contest name")
                        .help("Specify which contest to fetch.")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("problem id")
                        .help("Specify which problem to fetch when a contest name is set.")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // Fetch input/output examples and write each of them into text files.
    match matches.subcommand_matches("gen") {
        Some(ref matches) => {
            let contest_id = matches.value_of("contest name");
            let problem_id = matches.value_of("problem id");

            // Problem is specified (such as "a", "b", "c"...).
            match (contest_id, problem_id) {
                (Some(contest_id), Some(problem_id)) => {
                    execute_fetching_problem(contest_id, &problem_id)?;
                }
                (Some(contest_id), None) => {
                    execute_fetching_problems_in_contest(contest_id)?;
                }
                (_, _) => {}
            }
            Ok(())
        }
        None => {
            Ok(())
        }
    }
}
