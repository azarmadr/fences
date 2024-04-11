use fences::solver;
use fences::Board;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let mut b = Board::from_task_string(5, "         2  3331 0 1 3  3", None);
    // b.set_solution("yyuuuuuuyyuyyuuyuuyuuyyuuuuuuuuuuuyuuuyuyuuyuyuuuyuyuyuuyuuy");
    solver::solve(&mut b);
    solver::solve2(&mut b);

    println!("{}", b);
    *b.fences_mut()[0][(1, 0)] = Some(false);
    for i in 1..5 {
        *b.fences_mut()[1][(0, i)] = Some(false);
    }
    *b.fences_mut()[0][(2, 0)] = Some(false);
    *b.fences_mut()[0][(3, 0)] = Some(false);
    *b.fences_mut()[1][(3, 5)] = Some(true);
    *b.fences_mut()[0][(5, 0)] = Some(true);
    *b.fences_mut()[0][(5, 1)] = Some(false);
    *b.fences_mut()[0][(5, 2)] = Some(true);
    *b.fences_mut()[1][(4, 1)] = Some(true);
    for i in 0..5 {
        *b.fences_mut()[0][(0, i)] = Some(true);
        *b.fences_mut()[1][(i, 0)] = Some(true);
    }
    println!("{}", b);
    solver::solve(&mut b);
    println!("{}", b);
    *b.fences_mut()[1][(0, 5)] = Some(true);
    solver::solve(&mut b);
    println!("{}", b);
}
