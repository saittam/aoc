use std::collections::HashSet;
use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut rolls = lines.enumerate().fold(
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

  let mut n = 0;
  let mut q = rolls.iter().cloned().collect::<Vec<_>>();
  while let Some(p) = q.pop() {
    if !rolls.contains(&p) {
      continue;
    }

    let (x, y) = p;
    let neigh = ((x - 1)..=(x + 1))
      .flat_map(|x| {
        ((y - 1)..=(y + 1)).map(move |y| (x, y))
      })
      .filter(|p| rolls.contains(&p))
      .collect::<Vec<_>>();

    if neigh.len() < 5 {
      n += 1;
      rolls.remove(&p);
      q.extend(neigh);
    }
  }

  println!("{n}");
}
