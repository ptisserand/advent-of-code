use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()>{
  // add color-eyre handler
  color_eyre::install()?;
  let input = read_input()?;
  println!("{input}");

  // return a result
  Ok(())
}

fn read_input() -> color_eyre::Result<String> {
  let path = "input2.txt";
  let input = std::fs::read_to_string(path).wrap_err("reading input.txt")?;
  Ok(input)
}