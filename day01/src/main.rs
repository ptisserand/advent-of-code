fn main() -> color_eyre::Result<()> {
    // add color-eyre handler
    color_eyre::install()?;

    for group in include_str!("../input_test.txt").split("\n\n") {
        println!("In group:");
        for line in group.lines() {
            println!("Got line: {}", line);
        }
    }
    
    // return a result
    Ok(())
}
