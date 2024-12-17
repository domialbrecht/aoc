use day_09::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file.trim()).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
