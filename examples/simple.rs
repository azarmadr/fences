use anyhow::Result;
use fences::{game, get_input};

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Trace).unwrap();
    let rows = get_input("select puzzle:")
        .unwrap()
        .trim()
        .parse()
        .unwrap_or(15);
    game(rows)
}
