mod o11y;

use tracing::{info, instrument};

fn main() {
    o11y::init();
    run();
    o11y::ship();
}

#[instrument]
fn run() {
    info!("Hello World!");
    add1(3);
}

#[instrument]
fn add1(num: usize) -> usize {
    info!("Adding 1");
    add(num, 1)
}

#[instrument]
fn add(a: usize, b: usize) -> usize {
    info!("Adding");
    a + b
}
