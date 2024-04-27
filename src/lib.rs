use colored::Colorize;
pub mod board;
pub use crate::board::Board;
pub mod solver;
use anyhow::Result;
use std::{io, process::exit};

pub fn add_idx(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub_idx(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 - b.0, a.1 - b.1)
}

pub fn game(b: &mut Board, sol_file: &str) -> Result<()> {
    let mut i = 0;
    println!("\nMove {i}:\n{b}");
    solver::solve(b);
    println!("Solver {i}.\n{b}");
    let mut cp = vec![];

    let mut moves = vec![format!(
        "{}#{}",
        b.cols(),
        b.task()
            .iter()
            .map(|t| char::from(t.clone()))
            .collect::<String>()
    )];
    if let Some(won) = b.result() {
        if won {
            println!(
                "You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!"
            );
            moves.push(b.solution());
            std::fs::write(sol_file, moves.join("\n"))?;
            exit(0);
        } else {
            println!("{}", "You made a mistake somewhere".red())
        }
    }
    let mut play = |input: String| -> Result<()> {
        log::trace!("{input}");
        let mut res = input.trim().split_whitespace();
        match res.next() {
            Some("s") => {
                println!("Saving...");
                std::fs::write(sol_file, moves.join("\n"))?;
            }
            Some("q") => {
                println!("Exiting...");
                std::fs::write(sol_file, moves.join("\n"))?;
                exit(0)
            }
            Some("u") => {
                if moves.len() < 2 {
                    return Ok(());
                }
                if let Some(m) = moves.pop() {
                    let cp = cp.pop().unwrap();
                    println!("undo: {m} cp: {cp}");
                    b.reset_to(cp)?;
                }
            }
            Some("m") => {
                for m in b.moves() {
                    println!("[{}]{:?}={}\n{}", m.direction, m.idx, m.value, m.name);
                }
                println!("User Moves: {}", moves.join("\n"))
            }
            Some("c") => cp.push(b.moves().len()),
            Some("cc") => cp.clear(),
            Some("cp") => println!("{:?}", cp.pop()),
            Some("C") => println!("{cp:?}"),
            Some("r") => b.reset_to(res.next().unwrap().parse()?)?,
            Some("p") => println!("Board:\n{b}"),
            Some(x) if x == "0" || x == "1" => {
                i += 1;
                let f: usize = x.parse()?;
                let row = res.next().unwrap().parse()?;
                let col = res.next().unwrap().parse()?;
                log::info!("[{f}]({row}, {col})");
                let val = match res.next().unwrap() {
                    "y" => true,
                    "n" => false,
                    x => unreachable!("{x}"),
                };
                cp.push(b.moves().len());
                moves.push(input.clone().trim().to_string());
                b.play(f, (row, col), val, &format!("player move {i}"));
                println!("Move {i}:\n{b}");
                solver::solve(b);
                println!("Solver {i}.\n{b}");
                println!("{}", input.clone().trim().to_string());
            }
            x => {
                log::warn!("Unknown input = {x:?}\nContinuing...")
            }
        }
        if let Some(won) = b.result() {
            if won {
                println!(
                    "You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!"
                );
                moves.push(b.solution());
                std::fs::write(sol_file, moves.join("\n"))?;
                exit(0);
            } else {
                println!("{}", "You made a mistake somewhere".red())
            }
        }
        Ok(())
    };

    loop {
        let input = get_input("Your Move:")?;
        if play(input).is_err() {
            println!("Wrong Input")
        }
    }
}
pub fn get_input(prompt: &str) -> Result<String> {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
