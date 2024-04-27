use anyhow::Result;
use fences::game;
use std::env;

fn main() -> Result<()> {
    let file = env::args().last().unwrap();
    println!("{file}");
    let sol_file = if file.ends_with(".sol.txt") {
        file.clone()
    } else {
        file.clone().replace(".txt", ".sol.txt")
    };
    let mut board = std::fs::read_to_string(&if std::path::Path::new(&sol_file).exists() {
        sol_file.clone()}else{file})?.parse().unwrap();

    simple_logger::init_with_level(log::Level::Info).unwrap();
    game(&mut board, &sol_file)
}
