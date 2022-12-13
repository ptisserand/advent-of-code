
fn main() -> color_eyre::Result<()>{
  // add color-eyre handler
  color_eyre::install()?;

  let input = include_str!("../input_test.txt");
  let mut lines = input.lines();
  while let Some(line) = lines.next() {
    println!("Got line: {}", line);
  }
  // return a result
  Ok(())
}

