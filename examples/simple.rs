use anyhow::Result;
use fences::solver;
use fences::Board;
use std::io;
use std::io::BufRead;
use std::process::exit;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Trace).unwrap();
    let rows = get_input("select puzzle:")
        .unwrap()
        .trim()
        .parse()
        .unwrap_or(15);
    let task = match rows {
            5 => "         2  3331 0 1 3  3",
            15 => "3 32 1   23  3  3   2    23  3  0322  0    3 3       3  3  21  33 3    1 3 2 1 121  2  3      3    0   23 222     2   0 2  2 3       32 3    21 3   2     32221   2  3     1 31  2  3021231   2222     222   23    222 1232 3    ",
            30 =>"   32 32 332 23 32 23 3 12 1  2  22               2   022  23   2211  23 2 2   2212   2  32    3      02 02 3     3    22   0  123  32  3 1   1     2 32  0   2 0 2 1 32 1 3 0  222 3  2 31 1  3 2 3 2212    22   3 2213  2   3 1  3    3  21 2  112131  23   2    20   2  3   1   3 33  2      3   2 1 1213  22 2 2 23 2  22  1 3 2 2   2321 213  13222   1     3 3  1        2 21 23 233 23 122 02 131 33 32  2 1 20  203  01  2   2 1 22  3   33 212    0    2 3   21 1  2  2 22  131 11 3     1 22 3    122 3 22   22 0 1 11 3   212 3   2 1   2 22 3223 231    1    1  3     1 10  22 12  3 202 2110        31 1 2   2   22      10  1 322    22  1  2  32  3   2  2  2  3   213     321      1113 21    32 32  3   2  12122       3  3 12 3  3  220 122 3 0211  12  0 1 2 2  2 0   3     3 12 2223 3 1 3 3 2   1 231 2  32      22  12  22  2 3 123  32  2322   3  313  13 3 3    1 21 1  2 1       13       32   2 3   1222 3",
            _ => unreachable!()
        };
    let b = &mut Board::from_task_string(rows, task, None);
    let mut i = 0;
    println!("\nMove {i}:\n{b}");
    solver::solve(b);
    println!("Solver {i}.\n{b}");

    let mut moves = vec![format!("# {rows}: '{task}'")];
    let mut play = |input: String| -> Result<()> {
        log::trace!("{input}");
        let mut res = input.trim().split_whitespace();
        match res.next() {
            Some("s") => {
                println!("Saving...");
                std::fs::write(format!("moves-{rows}.txt"), moves.join("\n"))?;
            }
            Some("q") => {
                println!("Exiting...");
                std::fs::write(format!("moves-{rows}.txt"), moves.join("\n"))?;
                exit(0)
            }
            Some("u") => {
                if let Some(m) = moves.pop() {
                    println!("undo: {m}")
                }
            }
            Some("m") => {
                for m in b.moves() {
                    println!("[{}]{:?}={}\n{}", m.direction, m.idx, m.value, m.name);
                }
                println!("User Moves: {}", moves.join("\n"))
            }
            Some("p") => println!("Board:\n{b}"),
            Some(x) if x == "0" || x == "1" => {
                i += 1;
                moves.push(input.clone().trim().to_string());
                let f: usize = x.parse()?;
                let row = res.next().unwrap().parse()?;
                let col = res.next().unwrap().parse()?;
                log::info!("[{f}]({row}, {col})");
                b.play(
                    f,
                    (row, col),
                    match res.next().unwrap() {
                        "y" => true,
                        "n" => false,
                        x => unreachable!("{x}"),
                    },
                    format!("player move {i}"),
                );
                println!("Move {i}:\n{b}");
                solver::solve(b);
                println!("Solver {i}.\n{b}");
            }
            x => {
                log::warn!("Unknown input = {x:?}\nContinuing...")
            }
        }
        /*
        if let Some(won) = b.result() {
            if won {
                println!(
                    "You completed the puzzle. Checkout your moves at `moves-{rows}.txt`!!!
                         Press 'q' to save and exit"
                );
            }
        }
        */
        Ok(())
    };

    if let Ok(file) = std::fs::File::open(format!("moves-{rows}.txt")) {
        for line in io::BufReader::new(file)
            .lines()
            .map(|x| x.unwrap())
            .filter(|x| !x.starts_with('#'))
        {
            play(line)?
        }
    }
    loop {
        let input = get_input("Your Move:")?;
        play(input)?;
    }
}
fn get_input(prompt: &str) -> Result<String> {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(dbg!(input))
}
