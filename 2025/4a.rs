use std::collections::HashSet;
use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let rolls = lines.enumerate().fold(
    HashSet::new(),
    |mut rolls, (y, l)| {
      rolls.extend(l.chars().enumerate().filter_map(
        move |(x, c)| {
          (c == '@').then(|| (x as i32, y as i32))
        },
      ));
      rolls
    },
  );

  let n = rolls
    .iter()
    .filter(|(x, y)| {
      ((x - 1)..=(x + 1))
        .flat_map(|x| {
          ((y - 1)..=(y + 1)).map(move |y| (x, y))
        })
        .filter(|p| rolls.contains(&p))
        .count()
        < 5
    })
    .count();

  println!("{n}");
}
