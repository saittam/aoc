use std::io::BufRead;

#[derive(Clone, Copy)]
enum Sym {
  Rock,
  Paper,
  Scissors,
}

fn score((a, b): &(Sym, Sym)) -> usize {
  const TABLE: [[usize; 3]; 3] =
  [[4, 1, 7],
   [8, 5, 2],
   [3, 9, 6]];
  TABLE[*a as usize][*b as usize]
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut s = Vec::new();
  for l in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut i = l.split(' ');
    let s1 = match i.next().expect("bad input") {
      "A" => Sym::Rock,
      "B" => Sym::Paper,
      "C" => Sym::Scissors,
      s => panic!("bad sym {}", s),
    };
    let s2 = match i.next().expect("bad input") {
      "X" => Sym::Rock,
      "Y" => Sym::Paper,
      "Z" => Sym::Scissors,
      s => panic!("bad sym {}", s),
    };
    s.push((s2, s1));
  }

  let total = s.iter().map(score).sum::<usize>();
  println!("{}", total);
}