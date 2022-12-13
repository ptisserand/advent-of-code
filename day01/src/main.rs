
fn main() -> color_eyre::Result<()>{
  // add color-eyre handler
  color_eyre::install()?;

  for line in include_str!("../input_test.txt").lines() {
    println!("Got line: {}", line);
  }
  // return a result
  Ok(())
}

