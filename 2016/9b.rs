use std::io::Read;

// A dynamically dispatched Iterator adaptor
// which can be used recursively without causing
// type recursion.
struct DynIter<'a, Item>(&'a mut dyn Iterator<Item=Item>);

impl<'a, Item> Iterator for DynIter<'a, Item> {
  type Item = Item;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

fn xlen(mut ci: impl Iterator<Item=char>) -> usize {
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
        n += xlen(DynIter(&mut ci).take(len)) * count;
      }
      _ => n += 1,
    }
  }
  n
}

fn main() {
  let mut stdin = std::io::stdin();
  let mut input = String::new();
  stdin.read_to_string(&mut input).expect("input");

  let ci = input.chars().filter(|c| !c.is_whitespace());
  println!("{}", xlen(ci));
}