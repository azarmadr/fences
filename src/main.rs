use anyhow::Result;
use fences::solver;
use fences::Board;
use std::io;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let b = &mut Board::from_task_string(5, "         2  3331 0 1 3  3", None);
    let mut i = 0;
    println!("\nMove {i}:\n{b}");
    solver::solve(b);
    println!("Solver {i}.\n{b}");

    let mut moves = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        log::trace!("{input}");
        let mut res = input.trim().split_whitespace();
        match res.next() {
            Some("q") | Some("Q") => {
                println!("Exiting...");
                return Ok(());
            }
            Some(x) => {
                moves.push(input.clone());
                let i: usize = x.parse()?;
                let row = res.next().unwrap().parse()?;
                let col = res.next().unwrap().parse()?;
                log::info!("[{i}]({row}, {col})");
                *b.fences_mut()[i][(row, col)] = Some(match res.next().unwrap() {
                    "y" => true,
                    "n" => false,
                    x => unreachable!("{x}"),
                });
            }
            _ => {
                log::trace!("Continuing...")
            }
        }
        i += 1;
        println!("Move {i}:\n{b}");
        solver::solve(b);
        println!("Solver {i}.\n{b}");
    }
}
