use divan::Bencher;
use fences::{
    solver::{solve1, solve2},
    Board,
};
fn main() {
    // Run registered benchmarks.
    divan::main();
}

macro_rules! bench_solver {
    ($a:ident, $b:ident) => {

#[divan::bench(consts = [5, 15, 30])]
pub fn $a<const N: usize>(bencher: Bencher) {
    let rows = N;
    bencher
        .with_inputs(|| Board::from_task_string(rows, match rows {
            5 => "         2  3331 0 1 3  3",
            15 => "3 32 1   23  3  3   2    23  3  0322  0    3 3       3  3  21  33 3    1 3 2 1 121  2  3      3    0   23 222     2   0 2  2 3       32 3    21 3   2     32221   2  3     1 31  2  3021231   2222     222   23    222 1232 3    ",
            30 =>"   32 32 332 23 32 23 3 12 1  2  22               2   022  23   2211  23 2 2   2212   2  32    3      02 02 3     3    22   0  123  32  3 1   1     2 32  0   2 0 2 1 32 1 3 0  222 3  2 31 1  3 2 3 2212    22   3 2213  2   3 1  3    3  21 2  112131  23   2    20   2  3   1   3 33  2      3   2 1 1213  22 2 2 23 2  22  1 3 2 2   2321 213  13222   1     3 3  1        2 21 23 233 23 122 02 131 33 32  2 1 20  203  01  2   2 1 22  3   33 212    0    2 3   21 1  2  2 22  131 11 3     1 22 3    122 3 22   22 0 1 11 3   212 3   2 1   2 22 3223 231    1    1  3     1 10  22 12  3 202 2110        31 1 2   2   22      10  1 322    22  1  2  32  3   2  2  2  3   213     321      1113 21    32 32  3   2  12122       3  3 12 3  3  220 122 3 0211  12  0 1 2 2  2 0   3     3 12 2223 3 1 3 3 2   1 231 2  32      22  12  22  2 3 123  32  2322   3  313  13 3 3    1 21 1  2 1       13       32   2 3   1222 3",
            _ => unreachable!()
        }, None))
        .bench_refs(|b| $b(b));
}

    }
}

bench_solver! {rules_over_board, solve1}
bench_solver! {hashmap, solve2}
