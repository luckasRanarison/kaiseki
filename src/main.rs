use anyhow::Error;
use kaiseki::build::build_fst;

fn main() -> Result<(), Error> {
    build_fst()
}
