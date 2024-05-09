mod ecs;
mod transformations;

use std::io;

use ecs::solve_ecs;

fn main() -> io::Result<()> {
    solve_ecs().unwrap();
    Ok(())
}
