use std::cmp::Reverse;

use itertools::{Itertools, FoldWhile};

fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let max = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        // consider all lines separated by 'None'
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                // group separator , it's done
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
        println!("Part2: {max:?}");
    // return a result
    Ok(())
}
