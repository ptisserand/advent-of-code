
fn main() {
  let input = read_input().unwrap();
  println!("{input}");
}

fn read_input() -> color_eyre::Result<String> {
  let path = "input2.txt";
  let input = std::fs::read_to_string(path)?;
  Ok(input)
}