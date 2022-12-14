/// An iterator that takes a `Option<u64>` items and yields sums of group of
/// `Some(u64)` items separated by `None` items
struct GroupSumIter<I> {
    inner: I,
}

impl<I> Iterator for GroupSumIter<I>
where
    I: Iterator<Item = Option<u64>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = loop {
            match self.inner.next() {
                Some(Some(v)) => break v,
                Some(None) => {
                    // weird didn't expect a separator here
                    // just skip it
                }
                None => return None,
            }
        };

        loop {
            match self.inner.next() {
                Some(Some(v)) => sum += v,
                Some(None) | None => {
                    // reached a separator or end of iterator
                    break Some(sum);
                }
            }
        }
    }
}

fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let lines = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok());
    let elven_lead = GroupSumIter { inner: lines }.max();
    println!("Part1: {elven_lead:?}");
    // return a result
    Ok(())
}
