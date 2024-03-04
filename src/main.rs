use fences::solver;
use fences::Board;
fn main() {
    let mut b = Board::new(5, 5);
    b.set_numbers("         2  3331 0 1 3  3");
    // b.set_solution("yyuuuuuuyyuyyuuyuuyuuyyuuuuuuuuuuuyuuuyuyuuyuyuuuyuyuyuuyuuy");

    solver::solve(&mut b);
    println!("{}", b);
}
