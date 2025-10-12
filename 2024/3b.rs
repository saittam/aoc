use std::io::Read;

fn parse(s: &str) -> Option<u64> {
  let open = s.find('(')?;
  let comma = open + s.get(open..)?.find(',')?;
  let close = comma + s.get(comma..)?.find(')')?;

  let a = s.get((open + 1)..comma)?.parse::<u64>().ok()?;
  let b = s.get((comma + 1)..close)?.parse::<u64>().ok()?;

  if s.get(0..open)? == "mul" && a < 1000 && b < 1000 {
    Some(a * b)
  } else {
    None
  }
}

fn main() {
  let mut stdin = std::io::stdin();
  let mut input = String::new();
  stdin.read_to_string(&mut input).expect("input");

  let (n, _) = input.split("do")
    .fold(
      (0, true),
      |(mut n, e), s| {
        let e = (e || s.starts_with("()")) &&
          !s.starts_with("n't()");
        if e {
          n += s.match_indices("mul")
            .filter_map(|(i, _)| parse(s.get(i..).unwrap()))
            .sum::<u64>();
        }
        (n, e)
      });

  println!("{n}");
}