use day_06::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    print!("{}", result);
    Ok(())
}