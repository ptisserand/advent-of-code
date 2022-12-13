fn main() {
  let input = match std::fs::read_to_string("input2.txt") {
    Ok(s) => s,
    Err(e) => panic!("Couldn't read input.txt: {e}"),
  };
  println!("{input}");
}