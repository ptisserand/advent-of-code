fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let groups = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("Groups: {groups:?}");
    // return a result
    Ok(())
}
