use day11::part2::process;
use miette::Context;
use std::env;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let testonly = env::var("TEST_ONLY").is_ok();

    let file = include_str!("../../input.txt");
    let testfile = include_str!("../../input-test.txt");
    let inputfile = if testonly { testfile } else { file };
    let result = process(inputfile, 1000000).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
