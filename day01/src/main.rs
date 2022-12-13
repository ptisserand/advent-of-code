fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    for group in include_str!("../input_test.txt").split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
        }
        println!("Group has sum {sum}");
    }

    // return a result
    Ok(())
}
