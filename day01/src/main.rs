
fn main() {
  let input = read_input().unwrap();
  println!("{input}");
}

fn read_input() -> Result<String, std::io::Error> {
  let path = "input2.txt";
  fs_err::read_to_string(path)
}