use std::io::BufRead;

#[derive(Clone, Copy)]
enum Sym {
  Rock,
  Paper,
  Scissors,
}

#[derive(Clone, Copy)]
enum Goal {
  Lose,
  Draw,
  Win,
}

fn score((a, b): &(Sym, Sym)) -> usize {
  const TABLE: [[usize; 3]; 3] = [
    [4, 1, 7],
    [8, 5, 2],
    [3, 9, 6]
  ];
  TABLE[*a as usize][*b as usize]
}

fn choose(g: Goal, s: Sym) -> Sym {
  const TABLE: [[Sym; 3]; 3] = [
    [ Sym::Scissors, Sym::Rock, Sym::Paper ],
    [ Sym::Rock, Sym::Paper, Sym::Scissors ],
    [ Sym::Paper, Sym::Scissors, Sym::Rock ],
  ];
  TABLE[s as usize][g as usize]
}

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines().map(|r| r.unwrap());

  let mut s = Vec::new();
  for l in lines.by_ref().take_while(|l| l.len() > 0) {
    let mut i = l.split(' ');
    let o = match i.next().expect("bad input") {
      "A" => Sym::Rock,
      "B" => Sym::Paper,
      "C" => Sym::Scissors,
      o => panic!("bad sym {}", o),
    };
    let g = match i.next().expect("bad input") {
      "X" => Goal::Lose,
      "Y" => Goal::Draw,
      "Z" => Goal::Win,
      g => panic!("bad goal {}", g),
    };
    s.push((g, o));
  }

  let total = s.iter()
    .map(|(g, s)| score(&(choose(*g, *s), *s)))
    .sum::<usize>();
  println!("{}", total);
}