
fn main() -> color_eyre::Result<()>{
  // add color-eyre handler
  color_eyre::install()?;

  let input = include_str!("../input.txt");
  println!("{input}");

  // return a result
  Ok(())
}

