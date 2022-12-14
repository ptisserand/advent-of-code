use std::cmp::Reverse;

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let answer = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        // consider all lines separated by 'None'
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    println!("Part2: {answer:?}");
    Ok(())
}
