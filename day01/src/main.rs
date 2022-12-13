use color_eyre::eyre::Context;


fn main() {
  // add color-eyre handler
  color_eyre::install().unwrap();
  let input = read_input().unwrap();
  println!("{input}");
}

fn read_input() -> color_eyre::Result<String> {
  let path = "input2.txt";
  let input = std::fs::read_to_string(path).wrap_err("reading input.txt")?;
  Ok(input)
}