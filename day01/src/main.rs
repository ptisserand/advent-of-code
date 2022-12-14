use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let max = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .coalesce(|a, b| match(a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .max()
        .flatten()
        .unwrap_or_default();
        println!("Part1: {max:?}");
    // return a result
    Ok(())
}
