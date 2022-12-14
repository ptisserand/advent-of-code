fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let lines = include_str!("../input_test.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let groups = lines
        .split(|line| line.is_none())
        .collect::<Vec<_>>();
    println!("Groups: {groups:?}");
    // return a result
    Ok(())
}
