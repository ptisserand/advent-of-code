/// And [std::io::Error] associated with a path
#[derive(Debug)]
struct PathedIoError {
  path: String,
  inner: std::io::Error,
}

fn main() {
  let input = read_input().unwrap();
  println!("{input}");
}

fn read_input() -> Result<String, PathedIoError> {
  let path = "input2.txt";
  match std::fs::read_to_string(path) {
    Ok(s) => Ok(s),
    Err(e) => Err(PathedIoError {path: path.into(), inner: e,})
  }
}