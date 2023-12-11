fn main() {
  let contents = advent_of_code_2023::run().unwrap();
  let res = process(&contents).unwrap();
  println!("{}", res);
}

fn process(contents: &str) -> Result<&str, ()> {
  Ok("")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() {
      let contents = "";
      assert_eq!("", process(contents).unwrap());
  }
}
