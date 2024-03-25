use fences::solver;
use fences::Board;

fn main() {
    let mut b = Board::from_task_string(5, "         2  3331 0 1 3  3", None);
    // b.set_solution("yyuuuuuuyyuyyuuyuuyuuyyuuuuuuuuuuuyuuuyuyuuyuyuuuyuyuyuuyuuy");

    solver::solve(&mut b);
    println!("{}", b);
}
