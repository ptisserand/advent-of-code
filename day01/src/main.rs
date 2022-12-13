
fn main() -> color_eyre::Result<()>{
  // add color-eyre handler
  color_eyre::install()?;

  let input = include_str!("../input_test.txt");
  let mut lines = input.lines();
  loop {
    let maybe_line = lines.next();
    match maybe_line {
      Some(line) => {
        println!("Got line: {}", line);
      }
      None => break,
    }
  }
  // return a result
  Ok(())
}

