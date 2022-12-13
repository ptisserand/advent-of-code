fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let mut max = 0;
    for group in include_str!("../input_test.txt").split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
        }
        if sum > max {
          max = sum;
        }
    }
    println!("Part 1: {max}");
    // return a result
    Ok(())
}
