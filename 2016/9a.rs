use std::io::Read;

fn main() {
  let mut stdin = std::io::stdin();
  let mut input = String::new();
  stdin.read_to_string(&mut input).expect("input");

  let mut ci = input.chars().filter(|c| !c.is_whitespace());
  let mut n = 0;
  while let Some(c) = ci.next() {
    match c {
      '(' => {
        let len = ci.by_ref()
          .take_while(|c| *c != 'x')
          .collect::<String>()
          .parse::<usize>()
          .expect("len");
        let count = ci.by_ref()
          .take_while(|c| *c != ')')
          .collect::<String>()
          .parse::<usize>()
          .expect("count");
        ci.nth(len - 1);
        n += len * count;
      }
      _ => n += 1,
    }
  }

  println!("{}", n);
}